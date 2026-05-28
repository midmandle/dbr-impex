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

- **`DataSource`** — async `read() -> Result<Vec<u8>, _>` (via `impl Future`)
- **`DataSink`** — async `write(&[u8]) -> Result<(), _>` (via `impl Future`)
- **`transfer(source, sink)`** — driver-agnostic, reads all bytes from the source then writes to the sink

Each driver lives in `src/driver/` and implements one or both traits. The builder pattern is used consistently: `FooConfig::new(...).build()` returns a `FooDriver`.

| Driver | Source | Sink | Status |
|--------|--------|------|--------|
| `local` | yes | yes | complete |
| `databricks` | yes | yes | complete |
| `s3` | yes | yes | complete |
| `azure_blob` | yes | yes | complete |
| `gcs` | yes | yes | complete |
| `ftp` | yes | yes | complete |
| `sftp` | yes | yes | complete |

All modules are public and re-exported from `src/driver.rs`.

All drivers build an OpenDAL `Operator` via their respective `services::*` builder and delegate read/write to it. Paths are always trimmed of a leading `/` before passing to `op.read()` / `op.write()` since OpenDAL paths are relative to root.

The Databricks driver uses `services::Dbfs` (`.endpoint()`, `.token()`). The S3 driver uses `services::S3` (`.bucket()`, `.region()`, `.access_key_id()`, `.secret_access_key()`). Azure Blob uses `services::Azblob` (`.container()`, `.account_name()`, `.account_key()`). GCS uses `services::Gcs` (`.bucket()`, `.credential()`). FTP uses `services::Ftp` (`.endpoint()`, `.user()`, `.password()`). SFTP uses `services::Sftp` (`.endpoint()`, `.user()`, `.key()`).

`main.rs` wires the CLI (clap) to the drivers. Global args (`workspace_url`, `dbfs_path`, `auth_token`) always target Databricks; subcommands select the external backend. Both `import` and `export` are fully wired for all drivers.

## Testing approach

- `transfer.rs` tests use `LocalDriver` for both source and sink (no mocking needed).
- `databricks.rs` has no unit tests — the HTTP behaviour is opendal's responsibility; integration tests require a real Databricks instance.
- `local.rs` tests use `tempdir` for scratch files.
- `#[tokio::test]` is used throughout since both `DataSource::read` and `DataSink::write` are async.
