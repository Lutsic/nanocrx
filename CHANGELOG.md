# Changelog

All notable changes to `nanocrx` are documented in this file.

## [Unreleased]

### Added

- Functions for checking if file is `CRX` and if it uses `CRX2` or `CRX3`.
- Support for CRX2.
- Conversion:
    Webstore URL => Extension ID, 
    Webstore URL => Extension Slug, 
    Extesion ID & Slug => Webstore URL

### Changed

- Renamed struct CrxHeader to Crx3Header.
- header_size in Crx3 now includes the first 12 bytes.
- Crx3Error renamed to CrxError.
- crx3_to_zip renamed to crx_to_zip.
- crx_to_zip now supports both CRX3 and CRX2.

## [0.1.0] - 2026-10-16

### Added

- Build Chrome Web Store download URLs.
- Validate extension IDs and product version formats.
- Parse CRX3 package headers and access metadata.
- Convert CRX3 files into ZIP archives.