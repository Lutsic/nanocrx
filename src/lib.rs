//! A fast, simple and powerful Rust library for working with Chrome extension packages.
//!
//! `nanocrx` provides utilities for constructing Chrome Web Store download
//! URLs, parsing CRX3 package headers, and converting CRX3 files into ZIP
//! archives.
//!
//! # Features
//!
//! - Build Chrome Web Store download URLs.
//! - Validate extension IDs and product version parameters format.
//! - Parse CRX3 headers and access package metadata.
//! - Convert CRX3 files into ZIP archives.
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
//! Convert a downloaded CRX3 file into a ZIP archive:
//!
//! ```no_run
//! use nanocrx::crx3_to_zip;
//!
//! crx3_to_zip("extension.crx", "extension.zip")?;
//! # Ok::<(), nanocrx::Crx3Error>(())
//! ```

#![warn(missing_docs)]

mod crx_url;
mod error;
mod crx3;
mod zip;

pub use crx_url::{CrxUrl, validate_id, validate_prodversion};
pub use error::{CrxUrlError, Crx3Error};
pub use crx3::{Crx3, CrxHeader, validate_magic};
pub use zip::crx3_to_zip;
