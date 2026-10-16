use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, copy},
    path::Path,
};

use crate::error::Crx3Error;
use crate::validate_magic;

/// Converts `CRX3` file into `ZIP` archive. 
///
/// If output zip already exists, it will be overwrited.
///
/// # Errors
///
/// Returns an error if the CRX header is corrupted or not using CRX3 format.
///
/// # Examples
///
/// ```no_run
/// use nanocrx::crx3_to_zip;
///
/// crx3_to_zip("extension.crx", "extension.zip")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// 
/// ```
pub fn crx3_to_zip(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> Result<(), Crx3Error> {
    let mut file = File::open(input)?;

    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;
    validate_magic(&magic)?;

    let mut version_bytes = [0u8; 4];
    file.read_exact(&mut version_bytes)?;
    let version = u32::from_le_bytes(version_bytes);

    if version != 3 {
        return Err(Crx3Error::InvalidCrxVersion);
    }

    let mut size_bytes = [0u8; 4];
    file.read_exact(&mut size_bytes)?;
    let header_size = u32::from_le_bytes(size_bytes) as u64;

    const MAX_HEADER_SIZE: u64 = 16 * 1024 * 1024;

    if header_size > MAX_HEADER_SIZE {
        return Err(Crx3Error::CrxHeaderTooLarge);
    }

    let file_len = file.metadata()?.len();

    if 12 + header_size > file_len {
        return Err(Crx3Error::CrxHeaderTruncated);
    }

    file.seek(SeekFrom::Start(12 + header_size))?;

    let mut output_file = File::create(output)?;
    copy(&mut file, &mut output_file)?;

    Ok(())
}
