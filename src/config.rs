use crate::error::{ConfigReadError, LockWriteError};
use serde::{Deserialize, Serialize};
use tokio::fs;

use std::default::Default;

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub fabric: Fabric,
    #[serde(default)]
    pub minecraft: Minecraft,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Fabric {
    pub version: Option<String>,
    #[serde(default)]
    pub installer: Installer,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Installer {
    pub version: Option<String>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Minecraft {
    pub version: Option<String>,
}

pub async fn read_config() -> Result<Config, ConfigReadError> {
    read_toml("fabricd.toml").await
}

pub async fn read_lock() -> Result<Config, ConfigReadError> {
    read_toml("fabricd.lock").await
}

pub async fn write_lock(lock: &Config) -> Result<(), LockWriteError> {
    let serialized = toml::to_string(lock)?;
    fs::write("fabricd.lock", &serialized).await?;

    Ok(())
}

async fn read_toml(filename: &str) -> Result<Config, ConfigReadError> {
    let contents = fs::read_to_string(filename).await?;
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}
