# dbr-impex

A command-line tool for importing and exporting files to and from Databricks.

## Overview

`dbr-impex` transfers files between external storage locations and Databricks. It is built around a simple `DataSource` / `DataSink` trait pair: any driver that implements `DataSource` can feed data into any driver that implements `DataSink`, and the core `transfer` function handles the rest.

## Architecture

```
src/
├── main.rs          # CLI entry point (clap)
├── lib.rs
├── transfer.rs      # DataSource / DataSink traits + transfer()
└── driver/
    ├── local.rs     # Local filesystem (source + sink) ✓
    ├── databricks.rs# Databricks (sink)               ⚙ in progress
    ├── ftp.rs       # FTP (source)                    ⚙ in progress
    └── sftp.rs      # SFTP (source)                   ⚙ in progress
```

The `transfer(source, sink)` function is driver-agnostic — adding a new storage backend only requires implementing one or both traits for that driver.

## Usage

```
dbr-impex <COMMAND>

Commands:
  import   Transfer data into Databricks
  export   Transfer data out of Databricks
```

> **Note:** The CLI is under active development. Currently only local-to-local transfers are wired up end-to-end; Databricks, FTP, and SFTP integrations are in progress.

## Building

Requires Rust (edition 2024 / Rust 1.85+).

```bash
cargo build --release
```

## Running tests

```bash
cargo test
```

Tests cover the local driver (read/write) and the core transfer function (success, source failure, and sink failure cases).

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `reqwest` | HTTP client for the Databricks REST API |
| `thiserror` | Typed error definitions per driver |
| `anyhow` | Top-level error propagation |
| `tempdir` | Temporary directories used in tests |
