use nanocrx::{
    validate_id,
    validate_prodversion,
    validate_magic,
    CrxUrl,
    CrxUrlError,
    Crx3,
    Crx3Error,
};

use std::{
    fs,
    path::PathBuf,
    process,
};

// ---------- Helpers ----------

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "nanocrx-test-{}-{name}.crx",
        process::id()
    ))
}

fn make_crx(version: u32, header_size: u32, payload: &[u8]) -> Vec<u8> {
    let mut data = Vec::new();

    data.extend_from_slice(b"Cr24");
    data.extend_from_slice(&version.to_le_bytes());
    data.extend_from_slice(&header_size.to_le_bytes());
    data.extend_from_slice(payload);

    data
}

// ---------- validate_id ----------

#[test]
fn valid_extension_ids() {
    assert!(validate_id("abcdefghijklmnopabcdefghijklmnop").is_ok());
    assert!(validate_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").is_ok());
}

#[test]
fn invalid_extension_ids() {
    assert!(matches!(
        validate_id("short"),
        Err(CrxUrlError::InvalidId)
    ));

    assert!(matches!(
        validate_id("abcdefghijklmnopabcdefghijklmnopq"),
        Err(CrxUrlError::InvalidId)
    ));

    assert!(matches!(
        validate_id("abcdefghijklmnopabcdefghijklmnoq"),
        Err(CrxUrlError::InvalidId)
    ));

    assert!(matches!(
        validate_id("ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP"),
        Err(CrxUrlError::InvalidId)
    ));
}

// ---------- validate_prodversion ----------

#[test]
fn valid_prodversions() {
    assert!(validate_prodversion("122.0.0.0").is_ok());
    assert!(validate_prodversion("1.2.3.4").is_ok());
    assert!(validate_prodversion("999.999.999.999").is_ok());
}

#[test]
fn invalid_prodversions() {
    assert!(matches!(
        validate_prodversion(""),
        Err(CrxUrlError::InvalidProdVersion)
    ));

    assert!(matches!(
        validate_prodversion("122.0.0"),
        Err(CrxUrlError::InvalidProdVersion)
    ));

    assert!(matches!(
        validate_prodversion("122.a.0.0"),
        Err(CrxUrlError::InvalidProdVersion)
    ));

    assert!(matches!(
        validate_prodversion("122.0.0.0.1"),
        Err(CrxUrlError::InvalidProdVersion)
    ));
}

// ---------- CrxUrl ----------

#[test]
fn crx_url_creation_and_builders() {
    let id = "abcdefghijklmnopabcdefghijklmnop";

    let url = CrxUrl::new(id)
        .expect("valid ID should be accepted")
        .prodversion("122.0.0.0")
        .expect("valid product version should be accepted")
        .allow_crx2(false)
        .to_url();

    assert!(url.contains(id));
    assert!(url.contains("122.0.0.0"));
    assert!(!url.contains("crx2"));
}

#[test]
fn crx_url_rejects_invalid_id() {
    assert!(matches!(
        CrxUrl::new("invalid"),
        Err(CrxUrlError::InvalidId)
    ));
}

#[test]
fn crx_url_rejects_invalid_prodversion() {
    let result = CrxUrl::new("abcdefghijklmnopabcdefghijklmnop")
        .unwrap()
        .prodversion("invalid");

    assert!(matches!(
        result,
        Err(CrxUrlError::InvalidProdVersion)
    ));
}

#[test]
fn crx_url_crx2_setting_changes_url() {
    let id = "abcdefghijklmnopabcdefghijklmnop";

    let base = CrxUrl::new(id).unwrap();

    let with_crx2 = base
        .clone()
        .allow_crx2(true)
        .to_url();

    let without_crx2 = base
        .allow_crx2(false)
        .to_url();

    assert_ne!(with_crx2, without_crx2);
}

// ---------- validate_magic ----------

#[test]
fn valid_magic_bytes() {
    assert!(validate_magic(b"Cr24").is_ok());
}

#[test]
fn invalid_magic_bytes() {
    assert!(matches!(
        validate_magic(b"Cr23"),
        Err(Crx3Error::InvalidMagicBytes)
    ));

    assert!(matches!(
        validate_magic(b"nope"),
        Err(Crx3Error::InvalidMagicBytes)
    ));
}

// ---------- Crx3::open ----------

#[test]
fn rejects_invalid_magic_in_file() {
    let path = temp_path("bad-magic");

    fs::write(&path, make_crx(3, 0, b"")).unwrap();

    let result = Crx3::open(&path);

    let _ = fs::remove_file(&path);

    assert!(matches!(
        result,
        Err(Crx3Error::InvalidMagicBytes)
    ));
}

#[test]
fn rejects_invalid_crx_version() {
    let path = temp_path("bad-version");

    fs::write(&path, make_crx(2, 0, b"")).unwrap();

    let result = Crx3::open(&path);

    let _ = fs::remove_file(&path);

    assert!(matches!(
        result,
        Err(Crx3Error::InvalidCrxVersion)
    ));
}

#[test]
fn rejects_oversized_header() {
    let path = temp_path("oversized");

    fs::write(
        &path,
        make_crx(3, 16 * 1024 * 1024 + 1, b""),
    )
    .unwrap();

    let result = Crx3::open(&path);

    let _ = fs::remove_file(&path);

    assert!(matches!(
        result,
        Err(Crx3Error::CrxHeaderTooLarge)
    ));
}

#[test]
fn rejects_truncated_header() {
    let path = temp_path("truncated");

    // Header claims to be 4 bytes long, but contains no payload.
    fs::write(&path, make_crx(3, 4, b"")).unwrap();

    let result = Crx3::open(&path);

    let _ = fs::remove_file(&path);

    assert!(matches!(
        result,
        Err(Crx3Error::CrxHeaderTruncated)
    ));
}

#[test]
fn rejects_invalid_protobuf() {
    let path = temp_path("bad-protobuf");

    // 0x00 is an invalid Protobuf field tag.
    fs::write(&path, make_crx(3, 1, &[0x00])).unwrap();

    let result = Crx3::open(&path);

    let _ = fs::remove_file(&path);

    assert!(matches!(
        result,
        Err(Crx3Error::Protobuf(_))
    ));
}

#[test]
fn rejects_missing_file() {
    let path = temp_path("does-not-exist");

    let _ = fs::remove_file(&path);

    assert!(Crx3::open(&path).is_err());
}

// ---------- Real CRX file ----------

#[test]
fn parses_real_extension() {
    let path = std::env::var_os("NANOCRX_TEST_CRX")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(
                "/home/lutsic/Downloads/IGINNFKHMMFHLKAGCMPGOFNJHANPMKLB_1_8_0_8.crx",
            )
        });

    if !path.exists() {
        eprintln!(
            "Skipping real CRX test: file not found at {}",
            path.display()
        );
        return;
    }

    let crx = Crx3::open(&path)
        .expect("real CRX should parse successfully");

    println!("File: {}", path.display());
    println!("Header size: {}", crx.header_size());

    let header = crx.header();

    println!(
        "RSA proofs: {}",
        header.sha256_with_rsa().len()
    );

    println!(
        "ECDSA proofs: {}",
        header.sha256_with_ecdsa().len()
    );

    println!(
        "Verified contents present: {}",
        header.verified_contents().is_some()
    );

    println!(
        "Signed header data present: {}",
        header.signed_header_data().is_some()
    );

    let crx_id = header
        .crx_id()
        .expect("CRX3 should contain a 16-byte extension ID");

    print!("Extension ID: ");

    for byte in crx_id {
        print!("{byte:02x}");
    }

    println!();

    // Every available proof should contain a public key and signature.
    for proof in header
        .sha256_with_rsa()
        .iter()
        .chain(header.sha256_with_ecdsa())
    {
        assert!(
            proof.public_key().is_some(),
            "proof has no public key"
        );

        assert!(
            proof.signature().is_some(),
            "proof has no signature"
        );
    }
}
