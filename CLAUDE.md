# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build --release   # build
cargo test              # run all tests
cargo test <test_name>  # run a single test by name
cargo clippy            # lint
```

Requires Rust edition 2024 (Rust 1.85+).

## Architecture

`dbr-impex` is a CLI tool that transfers files between external storage and Databricks DBFS. The core abstraction lives in `src/transfer.rs`:

- **`DataSource`** — async `read() -> Result<Vec<u8>, _>`
- **`DataSink`** — sync `write(&[u8]) -> Result<(), _>`
- **`transfer(source, sink)`** — driver-agnostic, reads all bytes from the source then writes to the sink

Each driver lives in `src/driver/` and implements one or both traits. The builder pattern is used consistently: `FooConfig::new(...).build()` returns a `FooDriver`.

| Driver | Source | Sink | Status |
|--------|--------|------|--------|
| `local` | yes | yes | complete |
| `databricks` | — | yes | complete |
| `ftp` | yes | — | in progress (not wired to CLI) |
| `sftp` | yes | — | in progress (not wired to CLI) |

`ftp` and `sftp` are private modules (not re-exported from `src/driver.rs`); `local` and `databricks` are public.

The Databricks driver uses OpenDAL's `services::Dbfs` builder (`.endpoint()`, `.token()`), configured at write time. The leading `/` of `dbfs_path` is stripped before passing to `op.write()` since OpenDAL paths are relative to root.

`main.rs` wires the CLI (clap) to the drivers. Global args (`workspace_url`, `dbfs_path`, `auth_token`) always target Databricks; subcommands select the source/sink backend. Currently only `import local` is wired end-to-end.

## Testing approach

- `transfer.rs` tests use `LocalDriver` for both source and sink (no mocking needed).
- `databricks.rs` has no unit tests — the HTTP behaviour is opendal's responsibility; integration tests require a real Databricks instance.
- `local.rs` tests use `tempdir` for scratch files.
- No async runtime is needed for sync sink tests; `#[tokio::test]` is used where `DataSource::read` is awaited.
