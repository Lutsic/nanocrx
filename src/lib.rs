//! A fast, simple and powerful Rust library for working with Chrome extension packages.
//!
//! `nanocrx` provides utilities for constructing Chrome Web Store download
//! URLs, parsing CRX package headers, and converting CRX files into ZIP
//! archives.
//!
//! # Features
//!
//! - Build direct download URLs for extension CRX files.
//! - Parse CRX headers and access package metadata.
//! - Convert CRX files into ZIP archives.
//! - Supports both CRX2 and CRX3 formats.
//! - Validate extension IDs and product version parameters format.
//! - Simple tools for working with extensions, like extracting ID or slug from a Web Store URL.
//!
//! # Examples
//!
//! Build a download URL:
//!
//! ```
//! use nanocrx::CrxUrl;
//!
//! let url = CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb")?
//!     .prodversion("155.0.0.0")?
//!     .allow_crx2(true)
//!     .to_url();
//!
//! # Ok::<(), nanocrx::CrxUrlError>(())
//! ```
//!
//! Get extension name from a CRX3 file:
//!
//! ```no_run
//! use nanocrx::Crx3;
//!
//! let crx = Crx3::open("extension.crx")?;
//! println!("Extension name: {}", crx.manifest().name);
//! # Ok::<(), nanocrx::CrxError>(())
//! ```
//!
//! Convert a downloaded CRX3 or CRX2 file into a ZIP archive:
//!
//! ```no_run
//! use nanocrx::crx_to_zip;
//!
//! crx_to_zip("extension.crx", "extension.zip")?;
//! # Ok::<(), nanocrx::CrxError>(())
//! ```

#![warn(missing_docs)]

mod crx_url;
mod crx_url_error;
mod validation;
mod crx;
mod crx_error;
mod crx3;
mod crx3_header;
mod zip;
mod crx2;
mod crx2_header;
mod conversion_error;
mod conversion;
mod manifest;
mod manifest_error;

pub use crx_url::CrxUrl;
pub use crx_url_error::CrxUrlError;
pub use validation::{validate_prodversion, validate_id, validate_magic};
pub use crx_error::CrxError;
pub use crx::{is_crx_file, is_crx3_file, get_crx_version, is_crx2_file};
pub use crx3::Crx3;
pub use crx3_header::{Crx3Header};
pub use zip::crx_to_zip;
pub use crx2::Crx2;
pub use crx2_header::Crx2Header;
pub use conversion_error::ConversionError;
pub use conversion::{id_from_webstore_url, slug_from_webstore_url, webstore_url_from_id_and_slug, extension_id_from_crx_id};
pub use manifest::{Manifest, parse_manifest};
pub use manifest_error::ManifestError;