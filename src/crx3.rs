use std::{fs::File, io::{Read}, path::Path};
use crate::error::Crx3Error;
use prost::Message;

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
/// # Ok::<(), nanocrx::Crx3Error>(())
/// ```
pub struct Crx3 {
    header_size: u32,
    header: CrxHeader,
}

/// Contains metadata parsed from a `CRX3` package header.
///
/// Provides access to the package's signature proofs, verified contents
/// metadata, signed header data, and extension ID when available.
///
/// The presence of signature data does not mean that the signature
/// has been cryptographically verified.
pub struct CrxHeader {
    sha256_with_rsa: Vec<AsymmetricKeyProof>,
    sha256_with_ecdsa: Vec<AsymmetricKeyProof>,
    verified_contents: Option<Vec<u8>>,
    signed_header_data: Option<Vec<u8>>,
    crx_id: Option<[u8; 16]>,
}

pub struct AsymmetricKeyProof {
    public_key: Option<Vec<u8>>,
    signature: Option<Vec<u8>>,
}

#[derive(Clone, PartialEq, Message)]
struct ProtoCrxFileHeader {
    #[prost(message, repeated, tag = "2")]
    sha256_with_rsa: Vec<ProtoAsymmetricKeyProof>,

    #[prost(message, repeated, tag = "3")]
    sha256_with_ecdsa: Vec<ProtoAsymmetricKeyProof>,

    #[prost(bytes = "vec", optional, tag = "4")]
    verified_contents: Option<Vec<u8>>,

    #[prost(bytes = "vec", optional, tag = "10000")]
    signed_header_data: Option<Vec<u8>>,
}

#[derive(Clone, PartialEq, Message)]
struct ProtoAsymmetricKeyProof {
    #[prost(bytes = "vec", optional, tag = "1")]
    public_key: Option<Vec<u8>>,

    #[prost(bytes = "vec", optional, tag = "2")]
    signature: Option<Vec<u8>>,
}

#[derive(Clone, PartialEq, Message)]
struct ProtoSignedData {
    #[prost(bytes = "vec", optional, tag = "1")]
    crx_id: Option<Vec<u8>>,
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
    /// bytes or CRX`] version are invalid, the header is truncated or too
    /// large, or the header contains invalid Protocol Buffers data.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nanocrx::Crx3;
    ///
    /// let crx = Crx3::open("extension.crx")?;
    /// println!("Header size: {}", crx.header_size());
    /// # Ok::<(), nanocrx::Crx3Error>(())
    /// ```
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Crx3Error> {
        let mut file = File::open(path)?;

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
        let header_size = u32::from_le_bytes(size_bytes) as usize;
        const MAX_HEADER_SIZE: usize = 16 * 1024 * 1024;
        if header_size > MAX_HEADER_SIZE {
            return Err(Crx3Error::CrxHeaderTooLarge);
        }

        let file_len = file.metadata()?.len();
        if 12u64 + header_size as u64 > file_len {
            return Err(Crx3Error::CrxHeaderTruncated);
        }

        let mut data = vec![0u8; header_size];
        file.read_exact(&mut data)?;

        let header = parse_header(&data)?;

        Ok(Self {
            header_size: header_size as u32,
            header,
        })
    }

    /// Returns the size of the `CRX3` header in bytes.
    ///
    /// This size excludes the fixed 12-byte CRX file header.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nanocrx::Crx3;
    ///
    /// let crx = Crx3::open("extension.crx")?;
    /// println!("Header size: {}", crx.header_size());
    /// # Ok::<(), nanocrx::Crx3Error>(())
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
    /// # Ok::<(), nanocrx::Crx3Error>(())
    /// ```
    pub fn header(&self) -> &CrxHeader {
        &self.header
    }
}

impl CrxHeader {
    /// Returns the `RSA` signature proofs included in the `CRX3` header.
    ///
    /// Each proof contains a public key and its associated signature.
    ///
    /// An empty slice is returned if the header contains no RSA proofs.
    pub fn sha256_with_rsa(&self) -> &[AsymmetricKeyProof] {
        &self.sha256_with_rsa
    }
    
    /// Returns the `ECDSA` signature proofs included in the `CRX3` header.
    ///
    /// Each proof contains a public key and its associated signature.
    ///
    /// An empty slice is returned if the header contains no ECDSA proofs.
    pub fn sha256_with_ecdsa(&self) -> &[AsymmetricKeyProof] {
        &self.sha256_with_ecdsa
    }

    /// Returns the verified contents metadata, if present.
    ///
    /// The returned bytes represent serialized metadata. They are not
    /// automatically decoded or cryptographically verified by this method.
    pub fn verified_contents(&self) -> Option<&[u8]> {
        self.verified_contents.as_deref()
    }
    
    /// Returns the signed header data, if present.
    ///
    /// The returned bytes contain serialized signed header metadata.
    /// Use [`Self::crx_id`] to access the parsed extension ID, if available.
    pub fn signed_header_data(&self) -> Option<&[u8]> {
        self.signed_header_data.as_deref()
    }
    
    /// Returns the CRX extension ID extracted from the signed header data,
    /// if present.
    ///
    /// The ID is represented as a 16-byte array.
    ///
    /// This method returns `None` if the header does not contain a usable ID.
    pub fn crx_id(&self) -> Option<&[u8; 16]> {
        self.crx_id.as_ref()
    }
}

impl AsymmetricKeyProof {
    pub fn public_key(&self) -> Option<&[u8]> {
        self.public_key.as_deref()
    }

    pub fn signature(&self) -> Option<&[u8]> {
        self.signature.as_deref()
    }
}

/// Validates the magic bytes inside `CRX` header.
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
pub fn validate_magic (magic: &[u8; 4]) -> Result<(), Crx3Error> {
    if magic != b"Cr24" {
        return Err(Crx3Error::InvalidMagicBytes);
    }
    Ok(())
}

fn parse_header(data: &[u8]) -> Result<CrxHeader, Crx3Error> {
    let proto = ProtoCrxFileHeader::decode(data)?;

    let crx_id = match proto.signed_header_data.as_deref() {
        Some(data) => {
            let signed_data = ProtoSignedData::decode(data)?;

            match signed_data.crx_id {
                Some(id) => {
                    let id: [u8; 16] = id
                        .try_into()
                        .map_err(|_| Crx3Error::InvalidCrxIdLength)?;

                    Some(id)
                }
                None => None,
            }
        }
        None => None,
    };

    let sha256_with_rsa = proto
        .sha256_with_rsa
        .into_iter()
        .map(|proof| AsymmetricKeyProof {
            public_key: proof.public_key,
            signature: proof.signature,
        })
        .collect();

    let sha256_with_ecdsa = proto
        .sha256_with_ecdsa
        .into_iter()
        .map(|proof| AsymmetricKeyProof {
            public_key: proof.public_key,
            signature: proof.signature,
        })
        .collect();

    Ok(CrxHeader {
        sha256_with_rsa,
        sha256_with_ecdsa,
        verified_contents: proto.verified_contents,
        signed_header_data: proto.signed_header_data,
        crx_id,
    })
}
