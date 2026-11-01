use std::fmt;

/// Errors that can occur when converting.
#[derive(Debug, PartialEq, Eq)]
pub enum ConversionError {
    /// Provided webstore URL is invalid.
    BadWebstoreUrl,
    /// Invalid extension ID.
    InvalidExtensionId,
    /// Invalid CRX ID.
    InvalidCrxId,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadWebstoreUrl => write!(f, "Invalid webstore URL"),
            Self::InvalidExtensionId => write!(f, "Invalid extension ID"),
            Self::InvalidCrxId => write!(f, "Invalid CRX ID"),
        }
    }
}

impl std::error::Error for ConversionError {}