use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, copy},
    path::Path,
};

use crate::error::CrxError;
use crate::validate_magic;

/// Converts `CRX` file into `ZIP` archive. 
///
/// If output zip already exists, it will be overwrited.
///
/// # Errors
///
/// Returns an error if the CRX header is corrupted or file is not a valid CRX.
///
/// # Examples
///
/// ```no_run
/// use nanocrx::crx_to_zip;
///
/// crx_to_zip("extension.crx", "extension.zip")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// 
/// ```
pub fn crx_to_zip(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> Result<(), CrxError> {
    let mut file = File::open(input)?;

    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;
    validate_magic(&magic)?;

    let mut version_bytes = [0u8; 4];
    file.read_exact(&mut version_bytes)?;
    let version = u32::from_le_bytes(version_bytes);

    let header_size;

    if version == 2 {
        header_size = header_size_from_crx2(&mut file)? as u64;
    } else if version == 3 {
        header_size = header_size_from_crx3(&mut file)? as u64;
    } else {
        return Err(CrxError::InvalidCrxVersion);
    }

    const MAX_HEADER_SIZE: u64 = 16 * 1024 * 1024;

    if header_size > MAX_HEADER_SIZE {
        return Err(CrxError::CrxHeaderTooLarge);
    }

    let file_len = file.metadata()?.len();

    if header_size > file_len {
        return Err(CrxError::CrxHeaderTruncated);
    }

    file.seek(SeekFrom::Start(header_size))?;

    let mut output_file = File::create(output)?;
    copy(&mut file, &mut output_file)?;

    Ok(())
}

fn header_size_from_crx2(file: &mut File) -> Result<u32, CrxError> {
    let mut public_key_size_bytes = [0u8; 4];
    file.read_exact(&mut public_key_size_bytes)?;
    let public_key_size = u32::from_le_bytes(public_key_size_bytes);

    let mut signature_size_bytes = [0u8; 4];
    file.read_exact(&mut signature_size_bytes)?;
    let signature_size = u32::from_le_bytes(signature_size_bytes);

    let header_size = 16 + public_key_size + signature_size;

    Ok(header_size)
}

fn header_size_from_crx3(file: &mut File) -> Result<u32, CrxError> {
    let mut size_bytes = [0u8; 4];
    file.read_exact(&mut size_bytes)?;
    let header_size = 12 + u32::from_le_bytes(size_bytes);

    Ok(header_size)
}