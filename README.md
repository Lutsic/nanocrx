# NanoCrx
A fast, simple and powerful Rust library for working with Chrome extension packages.

`nanocrx` provides utilities for constructing Chrome Web Store download
URLs, parsing CRX3 package headers, and converting CRX3 files into ZIP
archives.

# Features

- Build Chrome Web Store download URLs.
- Validate extension IDs and product version parameters format.
- Parse CRX3 headers and access package metadata.
- Convert CRX3 files into ZIP archives.

# Examples

Build a download URL:
```
use nanocrx::CrxUrl;

let url = CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb")?
    .prodversion("155.0.0.0")?
    .to_url();
```

Convert a downloaded CRX3 file into a ZIP archive:
```n
use nanocrx::crx3_to_zip;

crx3_to_zip("extension.crx", "extension.zip")?;
```
