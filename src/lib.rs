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
//!     .to_url();
//!
//! assert!(url.contains("acceptformat=crx3"));
//! # Ok::<(), nanocrx::CrxUrlError>(())
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
mod error;
mod crx3;
mod crx2;
mod crx;
mod zip;
mod conversion;

pub use crx_url::{CrxUrl, validate_id, validate_prodversion};
pub use error::{CrxUrlError, CrxError};
pub use crx3::{Crx3, Crx3Header};
pub use crx2::{Crx2, Crx2Header};
pub use crx::{is_crx_file, is_crx2_file, is_crx3_file, get_crx_version, validate_magic};
pub use zip::{crx_to_zip};
pub use conversion::{id_from_webstore_url, slug_from_webstore_url, webstore_url_from_id_and_slug};

