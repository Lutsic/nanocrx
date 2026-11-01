use std::fs;
use std::env::temp_dir;
use std::path::PathBuf;
use nanocrx::{is_crx_file, is_crx2_file, is_crx3_file, get_crx_version};

fn create_test_file(name: &str, data: &[u8]) -> PathBuf {
    let path = temp_dir().join(format!(
        "nanocrx-{}-{}",
        std::process::id(),
        name
    ));

    fs::write(&path, data).unwrap();
    path
}

#[test]
fn valid_files() {
    let crx3 = create_test_file("valid_crx3.crx", b"Cr24\x03\x00\x00\x00");
    let crx2 = create_test_file("valid_crx2.crx", b"Cr24\x02\x00\x00\x00");

    assert!(is_crx2_file(&crx2).unwrap());
    assert!(!is_crx2_file(&crx3).unwrap());

    assert!(is_crx3_file(&crx3).unwrap());
    assert!(!is_crx3_file(&crx2).unwrap());

    assert!(is_crx_file(&crx3).unwrap());
    assert!(is_crx_file(&crx2).unwrap());

    assert_eq!(get_crx_version(&crx3).unwrap(), 3);
    assert_eq!(get_crx_version(&crx2).unwrap(), 2);

    fs::remove_file(crx3).unwrap();
    fs::remove_file(crx2).unwrap();
}

#[test]
fn invalid_files() {
    let invalid_magic = create_test_file("invalid_magic.crx", b"Cr25\x03\x00\x00\x00");
    let invalid_version = create_test_file("invalid_version.crx", b"Cr24\x04\x00\x00\x00");

    assert!(!is_crx_file(&invalid_magic).unwrap());
    assert!(is_crx_file(&invalid_version).unwrap());

    assert!(!is_crx2_file(&invalid_magic).unwrap());
    assert!(!is_crx3_file(&invalid_magic).unwrap());

    assert!(!is_crx2_file(&invalid_version).unwrap());
    assert!(!is_crx3_file(&invalid_version).unwrap());

    assert!(get_crx_version(&invalid_magic).is_err());
    assert!(get_crx_version(&invalid_version).is_err());

    fs::remove_file(invalid_magic).unwrap();
    fs::remove_file(invalid_version).unwrap();
}