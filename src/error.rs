use crate::util::DownloadError;

#[derive(Debug)]
pub enum Error {
    ConfigRead(ConfigReadError),
    Io(std::io::Error),
    Join(tokio::task::JoinError),
    LockWrite(LockWriteError),
    Reqwest(reqwest::Error),
    Version(crate::server::error::VersionError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigRead(err) => err.fmt(f),
            Self::Io(err) => err.fmt(f),
            Self::Join(err) => err.fmt(f),
            Self::LockWrite(err) => err.fmt(f),
            Self::Reqwest(err) => err.fmt(f),
            Self::Version(err) => err.fmt(f),
        }
    }
}

impl From<ConfigReadError> for Error {
    fn from(err: ConfigReadError) -> Self {
        Self::ConfigRead(err)
    }
}

impl From<LockWriteError> for Error {
    fn from(err: LockWriteError) -> Self {
        Self::LockWrite(err)
    }
}

impl From<crate::server::error::VersionError> for Error {
    fn from(err: crate::server::error::VersionError) -> Self {
        Self::Version(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::Join(err)
    }
}

impl From<DownloadError> for Error {
    fn from(err: DownloadError) -> Self {
        match err {
            DownloadError::Io(err) => Self::Io(err),
            DownloadError::Reqwest(err) => Self::Reqwest(err),
        }
    }
}

#[derive(Debug)]
pub enum ConfigReadError {
    Io(std::io::Error),
    Deserialize(toml::de::Error),
}

impl std::error::Error for ConfigReadError {}

impl std::fmt::Display for ConfigReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => err.fmt(f),
            Self::Deserialize(err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for ConfigReadError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<toml::de::Error> for ConfigReadError {
    fn from(err: toml::de::Error) -> Self {
        Self::Deserialize(err)
    }
}

#[derive(Debug)]
pub enum LockWriteError {
    Io(std::io::Error),
    Serialize(toml::ser::Error),
}

impl std::error::Error for LockWriteError {}

impl std::fmt::Display for LockWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => err.fmt(f),
            Self::Serialize(err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for LockWriteError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<toml::ser::Error> for LockWriteError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Serialize(err)
    }
}
