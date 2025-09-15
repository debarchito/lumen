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
mod client;
mod config;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use protolumen::v1::client::client_server;
use std::{env, fs, path::PathBuf, process};
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
    working_directory: String,
    /// specify the path to the configuration file
    #[arg(short, long, default_value = "serverlumen.toml")]
    config: String,
  },
  /// start the serverlumen server
  Start {
    /// specify the working directory
    #[arg(short, long, default_value = ".")]
    working_directory: String,
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

  match args.command {
    Subcommands::Init {
      working_directory,
      config: config_path,
    } => {
      FmtSubscriber::builder().with_max_level(Level::INFO).init();

      let config_path = resolve_working_directory(working_directory)?.join(&config_path);
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
      working_directory,
      config: config_path,
      verbose,
      ..
    } => {
      FmtSubscriber::builder()
        .with_max_level(if verbose { Level::DEBUG } else { Level::INFO })
        .init();

      let config_path = resolve_working_directory(working_directory)?.join(&config_path);
      if !config_path.exists() {
        error!("configuration file {config_path:?} doesn't exist");
        process::exit(1);
      }

      let config = config::from_config(&config_path)?;
      let name = config.name;
      info!("[{name}] using configuration file {config_path:?}");

      let address = config.address.parse()?;
      info!("[{name}] listening on {address}");

      let client_service = client::ClientService;
      Server::builder()
        .add_service(client_server::ClientServer::new(client_service))
        .serve(address)
        .await?;
    }
  }

  Ok(())
}

fn resolve_working_directory(target: String) -> Result<PathBuf> {
  let target = match target.starts_with('.') {
    true => env::current_dir()
      .context("unable to resolve current working directory")?
      .join(&target),
    _ => PathBuf::from(&target),
  };
  if !target.exists() {
    error!("working directory {target:?} doesn't exist");
    process::exit(1);
  }

  Ok(target)
}
