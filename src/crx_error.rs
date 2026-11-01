use std::fmt;
use crate::ManifestError;

/// Errors that can occur when reading or parsing a `CRX` file.
#[derive(Debug)]
pub enum CrxError {
    /// The file does not contain the expected CRX magic bytes.
    InvalidMagicBytes,
    /// The CRX header exceeds the maximum supported size.
    CrxHeaderTooLarge,
    /// The file uses an incorrect CRX version.
    InvalidCrxVersion,
    /// The CRX header contains invalid Protocol Buffers data.
    Protobuf(prost::DecodeError),
    /// An input or output operation failed.
    Io(std::io::Error),
    /// The file ends before the complete CRX header can be read.
    CrxHeaderTruncated,
    /// The CRX extension ID has an invalid length.
    InvalidCrxIdLength,
    /// The manifest file is invalid.
    InvalidManifest(ManifestError),
}

impl From<std::io::Error> for CrxError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<prost::DecodeError> for CrxError {
    fn from(err: prost::DecodeError) -> Self {
        Self::Protobuf(err)
    }
}

impl From<ManifestError> for CrxError {
    fn from(err: ManifestError) -> Self {
        Self::InvalidManifest(err)
    }
}

impl fmt::Display for CrxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMagicBytes => write!(f, "Invalid Magic Bytes: Cr24 not found"),
            Self::CrxHeaderTooLarge => write!(f, "CRX header is too large"),
            Self::InvalidCrxVersion => write!(f, "Invalid CRX version"),
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Protobuf(err) => write!(f, "Invalid Protobuf data: {err}"),
            Self::CrxHeaderTruncated => write!(f, "CRX3 header extends beyond end of file"),
            Self::InvalidCrxIdLength => write!(f, "Invalid CRX ID length: expected 16 bytes"),
            Self::InvalidManifest(err) => write!(f, "Invalid manifest: {err}"),
        }
    }
}

impl std::error::Error for CrxError {}