use std::{fs::File, io::Read, path::Path};
use crate::{Crx3Header, CrxError, Manifest, parse_manifest};
use crate::crx3_header::parse_header;

/// Represents a `CRX3` extension package.
///
/// Provides access to the package's header metadata.
///
/// Opening a file validates its `CRX3` header and parses its metadata.
/// This does not necessarily verify the package's cryptographic signature.
///
/// # Examples
///
/// ```no_run
/// use nanocrx::Crx3;
///
/// let crx = Crx3::open("extension.crx")?;
/// println!("Header size: {}", crx.header_size());
/// # Ok::<(), nanocrx::CrxError>(())
/// ```
pub struct Crx3 {
    header_size: u32,
    header: Crx3Header,
    manifest: Manifest,
}

impl Crx3 {
    /// Opens a `CRX3` file and parses its header.
    ///
    /// Reads the file header, validates the CRX format and version,
    /// and parses the header metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or read, the magic
    /// bytes or CRX version are invalid, the header is truncated or too
    /// large, or the header contains invalid Protocol Buffers data.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nanocrx::Crx3;
    ///
    /// let crx = Crx3::open("extension.crx")?;
    /// println!("Header size: {}", crx.header_size());
    /// # Ok::<(), nanocrx::CrxError>(())
    /// ```
    pub fn open(path: impl AsRef<Path>) -> Result<Self, CrxError> {
        let mut file = File::open(path)?;
        let mut twelve_bytes = [0u8; 12];
        file.read_exact(&mut twelve_bytes)?;

        if &twelve_bytes[0..4] != b"Cr24" {
            return Err(CrxError::InvalidMagicBytes);
        }

        let version = u32::from_le_bytes([twelve_bytes[4], twelve_bytes[5], twelve_bytes[6], twelve_bytes[7]]);
        if version != 3 {
            return Err(CrxError::InvalidCrxVersion);
        }

        let header_size = u32::from_le_bytes([twelve_bytes[8], twelve_bytes[9], twelve_bytes[10], twelve_bytes[11]]);

        if header_size > 16 * 1024 * 1024 {
            return Err(CrxError::CrxHeaderTooLarge);
        }

        if header_size > file.metadata()?.len() as u32 || header_size == 0 {
            return Err(CrxError::CrxHeaderTruncated);
        }

        let mut header_data = vec![0u8; header_size as usize];
        file.read_exact(&mut header_data)?;

        let header = parse_header(&header_data)?;

        let mut zip_data = Vec::new();
        file.read_to_end(&mut zip_data)?;

        let manifest = parse_manifest(zip_data)?;

        Ok(Crx3 {
            header_size: header_size + 12,
            header,
            manifest,
        })
    }

    /// Returns the size of the `CRX3` header in bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nanocrx::Crx3;
    ///
    /// let crx = Crx3::open("extension.crx")?;
    /// println!("Header size: {}", crx.header_size());
    /// # Ok::<(), nanocrx::CrxError>(())
    /// ```
    pub fn header_size (&self) -> u32 {
        self.header_size
    }

    /// Returns a reference to the parsed `CRX3` header metadata.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nanocrx::Crx3;
    ///
    /// let crx = Crx3::open("extension.crx")?;
    /// let header = crx.header();
    /// println!("RSA proofs: {}", header.sha256_with_rsa().len());
    /// # Ok::<(), nanocrx::CrxError>(())
    /// ```
    pub fn header(&self) -> &Crx3Header {
        &self.header
    }

    /// Returns a reference to the parsed manifest.
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
}