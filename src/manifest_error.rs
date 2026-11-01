use std::fmt;

/// Represents errors that can occur while parsing a manifest file.
#[derive(Debug)]
pub enum ManifestError {
    /// Represents an error that occurred while reading a ZIP archive.
    ZipError(zip::result::ZipError),
    /// Represents an error that occurred while reading from an I/O stream.
    IoError(std::io::Error),
    /// Represents an error that occurred while parsing JSON data.
    JsonError(serde_json::Error),
}

impl From<zip::result::ZipError> for ManifestError {
    fn from(err: zip::result::ZipError) -> Self {
        Self::ZipError(err)
    }
}

impl From<std::io::Error> for ManifestError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<serde_json::Error> for ManifestError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err)
    }
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZipError(_) => write!(f, "ZIP error"),
            Self::IoError(_) => write!(f, "I/O error"),
            Self::JsonError(_) => write!(f, "JSON error"),
        }
    }
}

impl std::error::Error for ManifestError {}