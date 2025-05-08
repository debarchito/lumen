mod client;
mod config;
pub(crate) mod protolumen {
    pub(crate) mod v1 {
        pub(crate) mod client {
            pub(crate) mod auth {
                tonic::include_proto!("protolumen.v1.client.auth");
            }
            tonic::include_proto!("protolumen.v1.client");
        }
    }
}

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use deadpool_postgres::{Config, Pool, Runtime};
use deadpool_postgres::{ManagerConfig, RecyclingMethod};
use protolumen::v1::client::client_server;
use std::{env, fs, path::PathBuf, process};
use tokio_postgres::NoTls;
use tonic::transport::Server;
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

/// serverlumen is a server implementation of the protolumen-over-gRPC specification
#[derive(Parser)]
#[command(version, about)]
struct Arguments {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    /// initialize a dummy configuration file
    Init {
        /// specify the working directory
        #[arg(short, long, default_value = ".")]
        wd: String,
        /// specify the path to the configuration file
        #[arg(short, long, default_value = "serverlumen.toml")]
        config: String,
    },
    /// start the serverlumen server
    Start {
        /// specify the working directory
        #[arg(short, long, default_value = ".")]
        wd: String,
        /// specify the path to the configuration file
        #[arg(short, long, default_value = "serverlumen.toml")]
        config: String,
        /// enable verbose mode for finer logging
        #[arg(short, long)]
        verbose: bool,
    },
}

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct SharedState {
    pub(crate) pool: Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();

    match args.command {
        Subcommands::Init {
            wd,
            config: config_path,
        } => {
            FmtSubscriber::builder().with_max_level(Level::INFO).init();

            let config_path = working_directory(wd)?.join(&config_path);
            if config_path.exists() {
                error!("configuration file {config_path:?} already exists");
                process::exit(1);
            }

            fs::write(&config_path, config::init()?).with_context(|| {
                format!("failed to write serialized TOML from Config to {config_path:?}")
            })?;
            info!("initialized configuration file {config_path:?}");
        }
        Subcommands::Start {
            wd,
            config: config_path,
            verbose,
            ..
        } => {
            FmtSubscriber::builder()
                .with_max_level(if verbose { Level::DEBUG } else { Level::INFO })
                .init();

            let config_path = working_directory(wd)?.join(&config_path);
            if !config_path.exists() {
                error!("configuration file {config_path:?} doesn't exist");
                process::exit(1);
            }

            let config = config::from_config(&config_path)?;
            let name = config.name;
            info!("[{name}] using configuration file {config_path:?}");

            let address = config.address.parse()?;
            info!("[{name}] listening on {address}");

            let pool = database_pool(&config.database_url)?;
            info!("[{name}] pooling database on {}", config.database_url);

            let shared_state = SharedState { pool };
            let client_service = client::ClientService {
                state: shared_state.clone(),
            };

            Server::builder()
                .add_service(client_server::ClientServer::new(client_service))
                .serve(address)
                .await?;
        }
    }

    Ok(())
}

fn working_directory(wd: String) -> Result<PathBuf> {
    let wd = if wd.starts_with('.') {
        env::current_dir()
            .context("unable to resolve current working directory")?
            .join(&wd)
    } else {
        PathBuf::from(&wd)
    };
    if !wd.exists() {
        error!("working directory {wd:?} doesn't exist");
        process::exit(1);
    }

    Ok(wd)
}

fn database_pool(url: &str) -> Result<Pool> {
    let config = Config {
        url: Some(url.into()),
        manager: Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        }),
        ..Default::default()
    };

    config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .context("failed to create a database pool")
}
