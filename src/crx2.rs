use std::{fs::File, io::{Read}, path::Path};
use crate::{CrxError, Crx2Header, Manifest, parse_manifest};

/// Represents a `CRX2` extension package.
///
/// Provides access to the package's header metadata.
///
/// Opening a file validates its `CRX2` header and parses its metadata.
/// This does not necessarily verify the package's cryptographic signature.
///
/// # Examples
///
/// ```no_run
/// use nanocrx::Crx2;
///
/// let crx = Crx2::open("extension.crx")?;
/// println!("Header size: {}", crx.header_size());
/// # Ok::<(), nanocrx::CrxError>(())
/// ```
pub struct Crx2 {
    header_size: u32,
    header: Crx2Header,
    manifest: Manifest,
}

impl Crx2 {
    /// Opens a `CRX2` file and reads its header.
    ///
    /// This does not verify the signature or validate the contents.
    ///
    /// # Errors
    ///
    /// Returns an error if the file is not a valid `CRX2` file or if the header is corrupted.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, CrxError> {
        let mut file = File::open(path)?;
        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;
        if &magic != b"Cr24" {
            return Err(CrxError::InvalidMagicBytes);
        }

        let mut version_bytes = [0u8; 4];
        file.read_exact(&mut version_bytes)?;
        let version = u32::from_le_bytes(version_bytes);
        if version != 2 {
            return Err(CrxError::InvalidCrxVersion);
        }

        let mut public_key_size_bytes = [0u8; 4];
        file.read_exact(&mut public_key_size_bytes)?;
        let public_key_size = u32::from_le_bytes(public_key_size_bytes);

        let mut signature_size_bytes = [0u8; 4];
        file.read_exact(&mut signature_size_bytes)?;
        let signature_size = u32::from_le_bytes(signature_size_bytes);

        const MAX_HEADER_SIZE: u32 = 16 * 1024 * 1024;
        let header_size = 16 + public_key_size + signature_size;
        if header_size > MAX_HEADER_SIZE {
            return Err(CrxError::CrxHeaderTooLarge);
        }

        if header_size > file.metadata()?.len() as u32 || header_size == 0 {
            return Err(CrxError::CrxHeaderTruncated);
        }

        let mut signature = vec![0u8; signature_size as usize];
        file.read_exact(&mut signature)?;

        let mut public_key = vec![0u8; public_key_size as usize];
        file.read_exact(&mut public_key)?;  

        let header = Crx2Header {
            public_key_size,
            signature_size,
            signature,
            public_key,
        };

        let mut zip_data = Vec::new();
        file.read_to_end(&mut zip_data)?;

        let manifest = parse_manifest(zip_data)?;

        Ok(Self { header_size, header, manifest })
    }

    /// Returns the size of the `CRX2` header in bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nanocrx::Crx2;
    ///
    /// let crx = Crx2::open("extension.crx")?;
    /// println!("Header size: {}", crx.header_size());
    /// # Ok::<(), nanocrx::CrxError>(())
    /// ```
    pub fn header_size (&self) -> u32 {
        self.header_size
    }

    /// Returns a reference to the parsed `CRX2` header metadata.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nanocrx::Crx2;
    ///
    /// let crx = Crx2::open("extension.crx")?;
    /// let header = crx.header();
    /// println!("Public key size: {}", header.public_key_size());
    /// # Ok::<(), nanocrx::CrxError>(())
    /// ```
    pub fn header(&self) -> &Crx2Header {
        &self.header
    }

    /// Returns a reference to the parsed manifest data.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nanocrx::Crx2;
    ///
    /// let crx = Crx2::open("extension.crx")?;
    /// let manifest = crx.manifest();
    /// println!("Extension name: {}", manifest.name);
    /// # Ok::<(), nanocrx::CrxError>(())
    /// ```
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
}