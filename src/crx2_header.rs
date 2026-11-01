/// Contains metadata parsed from a `CRX2` package header.
///
/// Provides access to the package's signature proofs, verified contents
/// metadata, signed header data, and extension ID when available.
///
/// The presence of signature data does not mean that the signature
/// has been cryptographically verified.
pub struct Crx2Header {
    pub(crate) public_key_size: u32,
    pub(crate) signature_size: u32,
    pub(crate) signature: Vec<u8>,
    pub(crate) public_key: Vec<u8>,
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