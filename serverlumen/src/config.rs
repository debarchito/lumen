use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) address: String,
}

pub(crate) fn init() -> Result<String> {
    toml::to_string(&Config {
        address: "[::1]:6543".into(),
    })
    .context("failed to serialize Config into TOML")
}

pub(crate) fn from_config(config: &Path) -> Result<Config> {
    toml::from_str(&fs::read_to_string(config)?).context("something went wrong")
}
