use clap::{Args, Parser, Subcommand};

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
struct LocalSourceArgs {}

//SINK COMMANDS
#[derive(Subcommand)]
enum Sink {
    Local(LocalSinkArgs),
}

#[derive(Args)]
struct LocalSinkArgs {}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Import(import_args) => match import_args.source {
            Source::Local(local_source_args) => todo!(),
        },
        Commands::Export(export_args) => match export_args.sink {
            Sink::Local(local_sink_args) => todo!(),
        },
    }
}
