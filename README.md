# dbr-impex

A command-line tool for importing and exporting files to and from Databricks.

## Overview

`dbr-impex` transfers files between external storage locations and Databricks DBFS. It is built around a simple `DataSource` / `DataSink` trait pair: any driver that implements `DataSource` can feed data into any driver that implements `DataSink`, and the core `transfer` function handles the rest.

## Architecture

```
src/
├── main.rs          # CLI entry point (clap)
├── lib.rs
├── transfer.rs      # DataSource / DataSink traits + transfer()
└── driver/
    ├── local.rs     # Local filesystem (source + sink)
    ├── databricks.rs# Databricks DBFS (source + sink)
    ├── s3.rs        # AWS S3 (source + sink)
    ├── azure_blob.rs# Azure Blob Storage (source + sink)
    ├── gcs.rs       # Google Cloud Storage (source + sink)
    ├── ftp.rs       # FTP (source + sink)
    └── sftp.rs      # SFTP (source + sink)
```

The `transfer(source, sink)` function is driver-agnostic — adding a new storage backend only requires implementing one or both traits for that driver.

## Usage

```
dbr-impex <WORKSPACE_URL> <DBFS_PATH> <AUTH_TOKEN> <COMMAND>

Commands:
  import   Transfer data into Databricks
  export   Transfer data out of Databricks
```

Databricks (`WORKSPACE_URL`, `DBFS_PATH`, `AUTH_TOKEN`) is always the fixed endpoint. The subcommand selects the external backend.

### Import sources

```
import local   <PATH>
import s3      <BUCKET> <REGION> <ACCESS_KEY_ID> <SECRET_ACCESS_KEY> <PATH>
import azure-blob <CONTAINER> <ACCOUNT_NAME> <ACCOUNT_KEY> <PATH>
import gcs     <BUCKET> <CREDENTIAL> <PATH>
import ftp     <ADDRESS> <USERNAME> <PASSWORD> <PATH>
import sftp    <ADDRESS> <USERNAME> <KEY_PATH> <PATH>
```

### Export sinks

```
export local   <PATH>
export s3      <BUCKET> <REGION> <ACCESS_KEY_ID> <SECRET_ACCESS_KEY> <PATH>
export azure-blob <CONTAINER> <ACCOUNT_NAME> <ACCOUNT_KEY> <PATH>
export gcs     <BUCKET> <CREDENTIAL> <PATH>
export ftp     <ADDRESS> <USERNAME> <PASSWORD> <PATH>
export sftp    <ADDRESS> <USERNAME> <KEY_PATH> <PATH>
```

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
| `opendal` | Unified storage access (DBFS, S3, Azure Blob, GCS, FTP, SFTP) |
| `thiserror` | Typed error definitions per driver |
| `anyhow` | Top-level error propagation |
| `tokio` | Async runtime |
| `tempdir` | Temporary directories used in tests |
