#[derive(Debug)]
pub enum ConfigReadError {
    IO(std::io::Error),
    Deserialize(toml::de::Error),
}

impl std::error::Error for ConfigReadError {}

impl std::fmt::Display for ConfigReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(err) => err.fmt(f),
            Self::Deserialize(err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for ConfigReadError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<toml::de::Error> for ConfigReadError {
    fn from(err: toml::de::Error) -> Self {
        Self::Deserialize(err)
    }
}

#[derive(Debug)]
pub enum LockWriteError {
    IO(std::io::Error),
    Serialize(toml::ser::Error),
}

impl std::error::Error for LockWriteError {}

impl std::fmt::Display for LockWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(err) => err.fmt(f),
            Self::Serialize(err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for LockWriteError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<toml::ser::Error> for LockWriteError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Serialize(err)
    }
}
