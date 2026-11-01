/// Validates format of a Chrome product version parameter and returns `true` if it is valid.
///
/// A product version must contain from 2 to 4 numeric components separated by `.`.
///
/// # Examples
///
/// ```
/// use nanocrx::validate_prodversion;
///
/// validate_prodversion("155.0.0"); // Returns true
/// validate_prodversion("155"); // Returns false
/// ```
pub fn validate_prodversion(prodversion: &str) -> bool {
    if prodversion.is_empty() || !prodversion.chars().all(|c| c.is_ascii_digit() || c == '.') || !prodversion.contains(".") || prodversion.split('.').count() > 4 {
        return false;
    }
    true
}

/// Validates a Chrome Web Store extension ID and returns `true` if it is valid.
///
/// An extension ID must contain exactly 32 characters,
/// each in the range `a` through `p`.
///
/// # Examples
///
/// ```
/// use nanocrx::validate_id;
///
/// validate_id("iginnfkhmmfhlkagcmpgofnjhanpmklb"); // Returns true
/// validate_id("something"); // Returns false
/// ```
pub fn validate_id (id: &str) -> bool {
    if id.len() != 32 || !id.bytes().all(|b| (b'a'..=b'p').contains(&b)) {
        return false;
    }
    true
}

/// Validates the magic bytes to match the expected `CRX` format. Returns `true` if the magic bytes are valid.
///
/// A valid magic bytes must be equal to `Cr24`
///
/// # Examples
///
/// ```
/// use nanocrx::validate_magic;
///
/// validate_magic(&[0x43, 0x72, 0x32, 0x34]); // Returns true
/// validate_magic(b"Cr24"); // Also returns true
/// validate_magic(b"hi!!"); // Returns false :(
/// ```
pub fn validate_magic(magic: &[u8; 4]) -> bool {
    magic == b"Cr24"
}
