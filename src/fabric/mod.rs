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

async fn get_latest_version(component: Component) -> reqwest::Result<String> {
    let resp = reqwest::get(format!(
        "https://meta.fabricmc.net/v2/versions/{}",
        component.to_string()
    ))
    .await?
    .json::<Vec<Version>>()
    .await?;

    let version = resp
        .iter()
        .find(|version| version.stable)
        .expect("no stable version found");
    Ok(version.version.to_string())
}
