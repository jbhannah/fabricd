// use std::error::Error;

use crate::{
    fabric::{get_latest_version, Component},
    util::{download_file, DownloadError},
};

pub async fn latest_version() -> reqwest::Result<String> {
    get_latest_version(Component::Installer).await
}

pub async fn download_installer(version: &str) -> Result<(), DownloadError> {
    let filename = format!("fabric-installer-{}.jar", version);
    let url = format!(
        "https://maven.fabricmc.net/net/fabricmc/fabric-installer/{}/{}",
        version, filename
    );

    download_file(url, filename).await
}
