use prost::Message;
use crate::{CrxError, extension_id_from_crx_id};

/// Contains metadata parsed from a `CRX3` package header.
///
/// Provides access to the package's signature proofs, verified contents
/// metadata, signed header data, and extension ID when available.
///
/// The presence of signature data does not mean that the signature
/// has been cryptographically verified.
pub struct Crx3Header {
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

impl Crx3Header {
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
    
    /// Returns the CRX ID extracted from the signed header data,
    /// if present.
    ///
    /// The ID is represented as a 16-byte array.
    ///
    /// This method returns `None` if the header does not contain a usable ID.
    pub fn crx_id(&self) -> Option<&[u8; 16]> {
        self.crx_id.as_ref()
    }

    /// Returns the Extension ID converted from the `CRX ID`, if present.
    ///
    /// The ID is represented as a 32 A-P.
    ///
    /// This method returns `None` if the header does not contain a usable ID.
    pub fn extension_id(&self) -> Option<String> {
        self.crx_id.as_ref().and_then(|crx_id| {
            extension_id_from_crx_id(crx_id)
                .ok()
                .map(|id| id.to_string())
        })
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

pub(crate) fn parse_header(data: &[u8]) -> Result<Crx3Header, CrxError> {
    let proto = ProtoCrxFileHeader::decode(data)?;

    let crx_id = match proto.signed_header_data.as_deref() {
        Some(data) => {
            let signed_data = ProtoSignedData::decode(data)?;

            match signed_data.crx_id {
                Some(id) => {
                    let id: [u8; 16] = id
                        .try_into()
                        .map_err(|_| CrxError::InvalidCrxIdLength)?;

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

    Ok(Crx3Header {
        sha256_with_rsa,
        sha256_with_ecdsa,
        verified_contents: proto.verified_contents,
        signed_header_data: proto.signed_header_data,
        crx_id,
    })
}