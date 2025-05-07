// use proto::calculator_server::{Calculator, CalculatorServer};
// use tonic::transport::Server;
// pub(crate) mod proto {
//     tonic::include_proto!("calculator");
// }

// #[derive(Debug, Default)]
// struct CalculatorService;

// #[tonic::async_trait]
// impl Calculator for CalculatorService {
//     async fn add(
//         &self,
//         req: tonic::Request<proto::CalculationRequest>,
//     ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
//         let input = req.get_ref();
//         let res = proto::CalculationResponse {
//             result: input.a + input.b,
//         };
//         Ok(tonic::Response::new(res))
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "[::1]:6543".parse()?;
//     let calc = CalculatorService::default();
//     Server::builder()
//         .add_service(CalculatorServer::new(calc))
//         .serve(addr)
//         .await?;
//     Ok(())
// }
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
use protolumen::v1::client::client_server;
use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;
use tonic::transport::Server;
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

/// serverlumen is a server implementation of the protolumen-over-gRPC specification
#[derive(Parser)]
#[command(version, about)]
struct Arguments {
    #[command(subcommand)]
    sub_commands: Subcommands,
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();

    match args.sub_commands {
        Subcommands::Init {
            wd,
            config: config_path,
        } => {
            FmtSubscriber::builder().with_max_level(Level::INFO).init();

            let wd = if wd == "." || wd == "./" {
                &env::current_dir().context("unable to get current working directory")?
            } else {
                Path::new(&wd)
            };
            if !wd.exists() {
                error!("working directory {wd:?} doesn't exist");
                exit(1);
            }

            let config_path = wd.join(&config_path);
            if config_path.exists() {
                error!("configuration file {config_path:?} already exists");
                exit(1);
            }

            fs::write(&config_path, config::init()?).context(format!(
                "failed to write serialized TOML from Config to {config_path:?}"
            ))?;
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

            let wd = if wd == "." || wd == "./" {
                &env::current_dir().context("unable to resolve current working diretory")?
            } else {
                Path::new(&wd)
            };
            if !wd.exists() {
                error!("working directory {wd:?} doesn't exist");
                exit(1);
            }

            let config_path = wd.join(&config_path);
            if !config_path.exists() {
                error!("configuration file {config_path:?} doesn't exist");
                exit(1);
            }

            let config = config::from_config(&config_path)?;
            let name = config.name;
            info!("[{name}] using configuration file {config_path:?}");

            let address = config.address.parse()?;
            info!("[{name}] listening on {address}");

            let client_service = client::ClientService::default();
            Server::builder()
                .add_service(client_server::ClientServer::new(client_service))
                .serve(address)
                .await?;
        }
    }

    Ok(())
}
