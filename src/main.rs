use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use dbr_impex::{
    driver::{
        azure_blob::AzureBlobConfig,
        databricks::DatabricksConfig,
        ftp::FtpConfig,
        gcs::GcsConfig,
        local::LocalConfig,
        s3::S3Config,
        sftp::SftpConfig,
    },
    transfer::transfer,
};
use tokio::runtime::Runtime;

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

#[derive(Args)]
struct LocalArgs {
    path: String,
}

#[derive(Args)]
struct S3Args {
    bucket: String,
    region: String,
    access_key_id: String,
    secret_access_key: String,
    path: String,
}

#[derive(Args)]
struct AzureBlobArgs {
    container: String,
    account_name: String,
    account_key: String,
    path: String,
}

#[derive(Args)]
struct GcsArgs {
    bucket: String,
    credential: String,
    path: String,
}

#[derive(Args)]
struct FtpArgs {
    address: String,
    username: String,
    password: String,
    path: String,
}

#[derive(Args)]
struct SftpArgs {
    address: String,
    username: String,
    key_path: String,
    path: String,
}

#[derive(Subcommand)]
enum Source {
    Local(LocalArgs),
    S3(S3Args),
    AzureBlob(AzureBlobArgs),
    Gcs(GcsArgs),
    Ftp(FtpArgs),
    Sftp(SftpArgs),
}

#[derive(Subcommand)]
enum Sink {
    Local(LocalArgs),
    S3(S3Args),
    AzureBlob(AzureBlobArgs),
    Gcs(GcsArgs),
    Ftp(FtpArgs),
    Sftp(SftpArgs),
}

fn main() {
    let cli = Cli::parse();
    let rt = Runtime::new().unwrap();

    match cli.command {
        Commands::Import(import_args) => {
            let sink =
                DatabricksConfig::new(cli.workspace_url, cli.auth_token, cli.dbfs_path).build();

            match import_args.source {
                Source::Local(args) => {
                    let source = LocalConfig::new(PathBuf::from(args.path)).build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
                Source::S3(args) => {
                    let source = S3Config::new(
                        args.bucket,
                        args.region,
                        args.access_key_id,
                        args.secret_access_key,
                        args.path,
                    )
                    .build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
                Source::AzureBlob(args) => {
                    let source = AzureBlobConfig::new(
                        args.container,
                        args.account_name,
                        args.account_key,
                        args.path,
                    )
                    .build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
                Source::Gcs(args) => {
                    let source =
                        GcsConfig::new(args.bucket, args.credential, args.path).build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
                Source::Ftp(args) => {
                    let source =
                        FtpConfig::new(args.address, args.username, args.password, args.path)
                            .build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
                Source::Sftp(args) => {
                    let source =
                        SftpConfig::new(args.address, args.username, args.key_path, args.path)
                            .build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
            }
        }
        Commands::Export(export_args) => {
            let source =
                DatabricksConfig::new(cli.workspace_url, cli.auth_token, cli.dbfs_path).build();

            match export_args.sink {
                Sink::Local(args) => {
                    let sink = LocalConfig::new(PathBuf::from(args.path)).build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
                Sink::S3(args) => {
                    let sink = S3Config::new(
                        args.bucket,
                        args.region,
                        args.access_key_id,
                        args.secret_access_key,
                        args.path,
                    )
                    .build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
                Sink::AzureBlob(args) => {
                    let sink = AzureBlobConfig::new(
                        args.container,
                        args.account_name,
                        args.account_key,
                        args.path,
                    )
                    .build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
                Sink::Gcs(args) => {
                    let sink =
                        GcsConfig::new(args.bucket, args.credential, args.path).build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
                Sink::Ftp(args) => {
                    let sink =
                        FtpConfig::new(args.address, args.username, args.password, args.path)
                            .build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
                Sink::Sftp(args) => {
                    let sink =
                        SftpConfig::new(args.address, args.username, args.key_path, args.path)
                            .build();
                    rt.block_on(async { transfer(&source, &sink).await.unwrap() });
                }
            }
        }
    }
}
