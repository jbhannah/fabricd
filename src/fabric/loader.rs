use crate::fabric::{get_latest_version, Component};

pub async fn latest_version() -> reqwest::Result<String> {
    get_latest_version(Component::Loader).await
}
