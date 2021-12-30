#[derive(Debug)]
pub enum VersionError {
    Io(std::io::Error),
    Json(json::Error),
    Zip(async_zip::error::ZipError),
}

impl std::error::Error for VersionError {}

impl std::fmt::Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => err.fmt(f),
            Self::Json(err) => err.fmt(f),
            Self::Zip(err) => write!(f, "{}", err.description()),
        }
    }
}

impl From<async_zip::error::ZipError> for VersionError {
    fn from(err: async_zip::error::ZipError) -> Self {
        Self::Zip(err)
    }
}

impl From<json::Error> for VersionError {
    fn from(err: json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<std::io::Error> for VersionError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
