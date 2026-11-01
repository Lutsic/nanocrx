use nanocrx::validate_magic;

#[test]
fn accepts_valid_magic() {
    assert!(validate_magic(&[0x43, 0x72, 0x32, 0x34]));
    assert!(validate_magic(b"Cr24"));
}

#[test]
fn rejects_invalid_magic() {
    assert!(!validate_magic(&[0x00, 0x00, 0x00, 0x00]));
    assert!(!validate_magic(b"abcd"));
}