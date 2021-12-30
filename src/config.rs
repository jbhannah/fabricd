use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub minecraft: Minecraft,
}

#[derive(Default, Deserialize)]
pub struct Minecraft {
    pub version: Option<String>,
}

pub async fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("fabricd.toml").await?;
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}
