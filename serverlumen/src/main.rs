pub(crate) mod protolumen {
  pub(crate) mod v1 {
    pub(crate) mod server {
      pub(crate) mod conn {
        tonic::include_proto!("protolumen.v1.server.conn");
      }
      tonic::include_proto!("protolumen.v1.server");
    }
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
      tonic::include_file_descriptor_set!("protolumen_v1_file_descriptor_set");
  }
}
mod config;
mod service;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use protolumen::v1::server::federation_server as server_federation_server;
use std::{env, fs, path::PathBuf, process};
use tonic::transport::Server;
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

/// serverlumen is a server implementation of the protolumen-over-gRPC specification.
#[derive(Parser)]
#[command(version, about)]
struct Arguments {
  #[command(subcommand)]
  command: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
  /// Initialize a dummy configuration file.
  Init {
    /// Specify the working directory.
    #[arg(short, long, default_value = ".")]
    working_directory: String,
    /// Specify the path to the configuration file.
    #[arg(short, long, default_value = "serverlumen.toml")]
    config: String,
  },
  /// Start the serverlumen server.
  Start {
    /// Specify the working directory.
    #[arg(short, long, default_value = ".")]
    working_directory: String,
    /// Specify the path to the configuration file.
    #[arg(short, long, default_value = "serverlumen.toml")]
    config: String,
    /// Enable verbose mode for finer logging.
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

      let config_path = resolve_full_working_directory(working_directory)?.join(&config_path);
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

      let config_path = resolve_full_working_directory(working_directory)?.join(&config_path);
      if !config_path.exists() {
        error!("configuration file {config_path:?} doesn't exist");
        process::exit(1);
      }

      let config = config::from_config(&config_path)?;
      let name = config.node.name;
      info!("[{name}] using configuration file {config_path:?}");

      let address = config.node.address.parse()?;

      info!("[{name}] listening on {address}");

      let server_federation_service = service::ServerFederationService;
      let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(protolumen::v1::FILE_DESCRIPTOR_SET)
        .build_v1()?;
      Server::builder()
        .add_service(reflection)
        .add_service(server_federation_server::FederationServer::new(
          server_federation_service,
        ))
        .serve(address)
        .await?;
    }
  }

  Ok(())
}

/// Resolve the full working directory if the target is a relative path.
fn resolve_full_working_directory(target: String) -> Result<PathBuf> {
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
