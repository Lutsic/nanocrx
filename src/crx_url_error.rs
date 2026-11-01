use std::fmt;

/// Errors that can occur when creating or configuring a `CrxUrl`.
#[derive(Debug, PartialEq, Eq)]
pub enum CrxUrlError {
    /// The ID has invalid format.
    InvalidId,
    /// The product version has invalid format.
    InvalidProdVersion,
}

impl fmt::Display for CrxUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidId => write!(f, "Invalid extension ID"),
            Self::InvalidProdVersion => write!(f, "Invalid Chrome product version"),
        }
    }
}

impl std::error::Error for CrxUrlError {}