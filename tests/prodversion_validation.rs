use nanocrx::validate_prodversion;

#[test]
fn accepts_valid_prodversions() {
    assert!(validate_prodversion("1.0.0.0"));
    assert!(validate_prodversion("9999.0.0.0"));
    assert!(validate_prodversion("1.0"));
    assert!(validate_prodversion("1.999.999"));
}

#[test]
fn rejects_invalid_prodversions() {
    assert!(!validate_prodversion("1.0.0.0.0"));
    assert!(!validate_prodversion("ABC"));
    assert!(!validate_prodversion("123"));
}