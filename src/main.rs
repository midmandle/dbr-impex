use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use dbr_impex::{
    driver::{databricks::DatabricksConfig, local::LocalConfig},
    transfer::{self, transfer},
};

#[derive(Parser)]
#[command(name = "dbr-impex")]
#[command(version = "1.0")]
#[command(about = "Databricks importer/exporter", long_about = None)]
struct Cli {
    workspace_url: String,
    dbfs_path: String,
    auth_token: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Import(ImportArgs),
    Export(ExportArgs),
}

#[derive(Args)]
struct ImportArgs {
    #[command(subcommand)]
    source: Source,
}

#[derive(Args)]
struct ExportArgs {
    #[command(subcommand)]
    sink: Sink,
}

//SOURCE COMMANDS
#[derive(Subcommand)]
enum Source {
    Local(LocalSourceArgs),
}

#[derive(Args)]
struct LocalSourceArgs {
    path: String,
}

//SINK COMMANDS
#[derive(Subcommand)]
enum Sink {
    Local(LocalSinkArgs),
}

#[derive(Args)]
struct LocalSinkArgs {}

fn main() {
    let cli = Cli::parse();

    let client = reqwest::blocking::Client::new();
    let databricks_sink =
        DatabricksConfig::new(client, cli.workspace_url, cli.auth_token, cli.dbfs_path).build();

    match cli.command {
        Commands::Import(import_args) => match import_args.source {
            Source::Local(local_source_args) => {
                let local_file_path = PathBuf::from(local_source_args.path);
                let source = LocalConfig::new(local_file_path).build();

                transfer(&source, &databricks_sink).unwrap();
            }
        },
        Commands::Export(export_args) => match export_args.sink {
            Sink::Local(local_sink_args) => todo!(),
        },
    }
}
