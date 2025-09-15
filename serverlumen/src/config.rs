use anyhow::{Context, Result};
use names::Generator;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

/// The full configuration.
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Config {
  pub(crate) node: Node,
  pub(crate) trust_list: Vec<TrustList>,
}

/// Configure the node.
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Node {
  pub(crate) name: String,
  pub(crate) id: String,
  pub(crate) address: String,
}

/// Configure the trust list for the node.
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct TrustList {
  pub(crate) id: String,
  pub(crate) address: String,
}

/// Initialize the configuration.
pub(crate) fn init() -> Result<String> {
  let mut generator = Generator::default();
  toml::to_string(&Config {
    node: Node {
      name: generator
        .next()
        .context("failed to generate a random name")?,
      id: Uuid::new_v8(rand::random()).to_string(),
      address: "[::1]:6543".into(),
    },
    trust_list: Vec::new(),
  })
  .context("failed to serialize Config into TOML")
}

/// Load the configuration from a file.
pub(crate) fn from_config(config: &Path) -> Result<Config> {
  toml::from_str(&fs::read_to_string(config)?).context("something went wrong")
}
