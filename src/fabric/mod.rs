pub mod installer;
pub mod loader;

use serde::Deserialize;

#[derive(Deserialize)]
struct Version {
    // url: String,
    // maven: String,
    version: String,
    stable: bool,
}

#[derive(Debug)]
enum Component {
    // Game,
    Installer,
    Loader,
}

impl ToString for Component {
    fn to_string(&self) -> String {
        match self {
            // Self::Game => "game",
            Self::Installer => "installer",
            Self::Loader => "loader",
        }
        .to_string()
    }
}

async fn get_latest_version(component: Component) -> reqwest::Result<Option<String>> {
    let resp = reqwest::get(format!(
        "https://meta.fabricmc.net/v2/versions/{}",
        component.to_string()
    ))
    .await?
    .json::<Vec<Version>>()
    .await?;

    if let Some(version) = resp.iter().find(|version| version.stable) {
        Ok(Some(version.version.to_string()))
    } else {
        Ok(None)
    }
}
