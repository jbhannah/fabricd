#[derive(Debug)]
pub enum DownloadError {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
}

impl std::error::Error for DownloadError {}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => err.fmt(f),
            Self::Reqwest(err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for DownloadError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

pub async fn download_file(url: String, filename: String) -> Result<(), DownloadError> {
    let response = reqwest::get(url).await?.bytes().await?;
    tokio::fs::write(&filename, &response).await?;
    Ok(())
}
