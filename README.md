[![Crate](https://img.shields.io/crates/v/nanocrx.svg)](https://crates.io/crates/nanocrx)
[![Documentation](https://docs.rs/nanocrx/badge.svg)](https://docs.rs/nanocrx/)

# NanoCrx
A fast, simple and powerful Rust library for working with Chrome extension packages.

`nanocrx` provides utilities for constructing Chrome Web Store download
URLs, parsing CRX package headers, and converting CRX files into ZIP
archives.

## Using `nanocrx` in your project
Add `nanocrx` to your `Cargo.toml`
``` 
[dependencies]
nanocrx = "0.2.0"
```
or run
```
cargo add nanocrx
```

# Features

- Build direct download URLs for extension CRX files.
- Parse CRX headers and access package metadata.
- Convert CRX files into ZIP archives.
- Supports both CRX2 and CRX3 formats.
- Validate extension IDs and product version parameters format.
- Simple tools for working with extensions, like extracting ID or slug from a Web Store URL.

# Examples

Build a download URL:
```
use nanocrx::CrxUrl;

let url = CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb")?
    .prodversion("155.0.0.0")?
    .to_url();
```

Convert a downloaded CRX3 or CRX2 file into a ZIP archive:
```
use nanocrx::crx_to_zip;

crx_to_zip("extension.crx", "extension.zip")?;
```

## Documentation

See the API documentation on [docs.rs](https://docs.rs/nanocrx/).

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
