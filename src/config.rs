use crate::error::{ConfigReadError, LockWriteError};
use serde::{Deserialize, Serialize};
use tokio::fs;

use std::default::Default;

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub minecraft: Minecraft,
}

impl Config {
    pub fn new() -> Config {
        Default::default()
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Minecraft {
    pub version: Option<String>,
}

pub async fn read_config() -> Result<Config, ConfigReadError> {
    let contents = fs::read_to_string("fabricd.toml").await?;
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}

pub async fn write_lock(lock: &Config) -> Result<(), LockWriteError> {
    let serialized = toml::to_string(lock)?;
    fs::write("fabricd.lock", &serialized).await?;

    Ok(())
}
