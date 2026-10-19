use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::{CrxError};

/// Checks if it is a `CRX2` file.
///
/// A valid `CRX2` file must have the magic bytes equal to `Cr24` and the version equal to `2`.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or read.
///
/// # Examples
///
/// ```no_run
/// use nanocrx::is_crx2_file;
///
/// is_crx2_file("extension.crx")?;
/// # Ok::<(), nanocrx::CrxError>(())
/// ```
pub fn is_crx2_file(path: impl AsRef<Path>) -> Result<bool, CrxError> {
    let mut file = File::open(path)?;
    let mut eight_bytes = [0u8; 8];
    file.read_exact(&mut eight_bytes)?;
    Ok(eight_bytes == [0x43, 0x72, 0x32, 0x34, 0x02, 0x00, 0x00, 0x00])
}

/// Checks if it is a `CRX3` file.
///
/// A valid `CRX3` file must have the magic bytes `Cr24` and a version number of `3`.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or read.
///
/// # Examples
///
/// ```no_run
/// use nanocrx::is_crx3_file;
///
/// is_crx3_file("extension.crx")?;
/// # Ok::<(), nanocrx::CrxError>(())
/// ```
pub fn is_crx3_file(path: impl AsRef<Path>) -> Result<bool, CrxError> {
    let mut file = File::open(path)?;
    let mut eight_bytes = [0u8; 8];
    file.read_exact(&mut eight_bytes)?;
    Ok(eight_bytes == [0x43, 0x72, 0x32, 0x34, 0x03, 0x00, 0x00, 0x00])
}

/// Checks if it is a `CRX` file.
///
/// A valid `CRX` file must have the magic bytes equal to `Cr24`.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or read.
///
/// # Examples
///
/// ```no_run
/// use nanocrx::is_crx_file;
///
/// is_crx_file("extension.crx")?;
/// # Ok::<(), nanocrx::CrxError>(())
/// ```
pub fn is_crx_file(path: impl AsRef<Path>) -> Result<bool, CrxError> {
    let mut file = File::open(path)?;
    let mut four_bytes = [0u8; 4];
    file.read_exact(&mut four_bytes)?;
    Ok(four_bytes == [0x43, 0x72, 0x32, 0x34])
}

/// Checks if file uses `CRX3` or `CRX2` format.
///
/// Returns `2` for `CRX2`, `3` for `CRX3`.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or read or if the file contains invalid magic bytes or an invalid CRX version.
///
/// # Examples
///
/// ```no_run
/// use nanocrx::get_crx_version;
///
/// get_crx_version("extension.crx")?;
/// # Ok::<(), nanocrx::CrxError>(())
/// ```
pub fn get_crx_version(path: impl AsRef<Path>) -> Result<u32, CrxError> {
    let mut file = File::open(path)?;
    let mut eight_bytes = [0u8; 8];
    file.read_exact(&mut eight_bytes)?;
    if &eight_bytes[0..4] != b"Cr24" {
        return Err(CrxError::InvalidMagicBytes);
    }
    let version = u32::from_le_bytes([eight_bytes[4], eight_bytes[5], eight_bytes[6], eight_bytes[7]]);
    if version != 2 && version != 3 {
        return Err(CrxError::InvalidCrxVersion);
    }
    Ok(version)
}

/// Validates the magic bytes to match the expected `CRX` format.
///
/// A valid magic bytes must be equal to `Cr24`
///
/// # Errors
///
/// Returns an error if the magic bytes are not `Cr24`.
///
/// # Examples
///
/// ```
/// use nanocrx::validate_magic;
///
/// assert!(validate_magic(&[0x43, 0x72, 0x32, 0x34]).is_ok());
/// assert!(validate_magic(b"Cr24").is_ok());
/// assert!(validate_magic(&[0x00, 0x00, 0x00, 0x00]).is_err());
/// ```
pub fn validate_magic (magic: &[u8; 4]) -> Result<(), CrxError> {
    if magic != b"Cr24" {
        return Err(CrxError::InvalidMagicBytes);
    }
    Ok(())
}