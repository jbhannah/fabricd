#[derive(Debug)]
pub enum Error {
    ConfigRead(ConfigReadError),
    Io(std::io::Error),
    Json(json::Error),
    LockWrite(LockWriteError),
    Zip(async_zip::error::ZipError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigRead(err) => err.fmt(f),
            Self::Io(err) => err.fmt(f),
            Self::Json(err) => err.fmt(f),
            Self::LockWrite(err) => err.fmt(f),
            Self::Zip(err) => write!(f, "{}", err.description()),
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

impl From<async_zip::error::ZipError> for Error {
    fn from(err: async_zip::error::ZipError) -> Self {
        Self::Zip(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Self {
        Self::Json(err)
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
