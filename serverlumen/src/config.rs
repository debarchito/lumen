use anyhow::{Context, Result};
use names::Generator;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
  pub(crate) name: String,
  pub(crate) address: String,
  pub(crate) trust_list: Vec<TrustList>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TrustList {
  pub(crate) name: String,
  pub(crate) address: String,
}

pub(crate) fn init() -> Result<String> {
  let mut generator = Generator::default();
  toml::to_string(&Config {
    name: generator
      .next()
      .context("failed to generate a random name")?,
    address: "[::1]:6543".into(),
    trust_list: Vec::new(),
  })
  .context("failed to serialize Config into TOML")
}

pub(crate) fn from_config(config: &Path) -> Result<Config> {
  toml::from_str(&fs::read_to_string(config)?).context("something went wrong")
}
