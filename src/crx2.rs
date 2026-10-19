use std::{fs::File, io::{Read}, path::Path};
use crate::error::CrxError;

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
}

/// Contains metadata parsed from a `CRX2` package header.
///
/// Provides access to the package's signature proofs, verified contents
/// metadata, signed header data, and extension ID when available.
///
/// The presence of signature data does not mean that the signature
/// has been cryptographically verified.
pub struct Crx2Header {
    public_key_size: u32,
    signature_size: u32,
    signature: Vec<u8>,
    public_key: Vec<u8>,
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

        Ok(Self { header_size, header })
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
}

impl Crx2Header {
    /// Returns the size of the public key in bytes.
    pub fn public_key_size(&self) -> u32 {
        self.public_key_size
    }

    /// Returns the size of the signature in bytes.
    pub fn signature_size(&self) -> u32 {
        self.signature_size
    }

    /// Returns a reference to the signature bytes.
    pub fn signature(&self) -> &[u8] {
        &self.signature
    }

    /// Returns a reference to the public key bytes.
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }
}