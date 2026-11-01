use crate::ConversionError;

/// Extracts the extension ID from a Chrome Web Store URL.
///
/// # Errors
///
/// Returns an error if the URL does not contain a valid extension ID.
///
/// # Examples
///
/// ```
/// use nanocrx::id_from_webstore_url;
///
/// id_from_webstore_url("https://chromewebstore.google.com/detail/boxel-rebound/iginnfkhmmfhlkagcmpgofnjhanpmklb?utm_source=chatgpt.com").unwrap();
/// id_from_webstore_url("chromewebstore.google.com/detail/boxel-rebound/iginnfkhmmfhlkagcmpgofnjhanpmklb").unwrap();
/// ```
pub fn id_from_webstore_url(url: &str) -> Result<&str, ConversionError> {
    let url = url.split('?').next().unwrap_or("");

    url.rsplit('/')
        .find(|part| !part.is_empty())
        .filter(|id| {
            id.len() == 32
                && id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
        .ok_or(ConversionError::BadWebstoreUrl)
}

/// Extracts the extension slug from a Chrome Web Store URL.
///
/// # Errors
///
/// Returns an error if the URL does not contain a valid extension slug.
///
/// # Examples
///
/// ```
/// use nanocrx::slug_from_webstore_url;
///
/// assert_eq!(slug_from_webstore_url("https://chromewebstore.google.com/detail/sponsorblock-for-youtube/mnjggcdmjocbbbhaepdhchncahnbgone?utm_source=chatgpt.com"),Ok("sponsorblock-for-youtube"));
/// assert_eq!(slug_from_webstore_url("chromewebstore.google.com/detail/boxel-rebound/iginnfkhmmfhlkagcmpgofnjhanpmklb"),Ok("boxel-rebound"));
/// assert!(slug_from_webstore_url("https://chromewebstore.google.com/det").is_err());
/// ```
pub fn slug_from_webstore_url(url: &str) -> Result<&str, ConversionError> {
    let url = url.split('?').next().unwrap_or("");

    let mut parts = url.rsplit('/');

    let id = parts.find(|part| !part.is_empty());
    let slug = parts.find(|part| !part.is_empty());

    match (id, slug) {
        (Some(id), Some(slug))
            if id.len() == 32
                && id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
                && !slug.is_empty() =>
        {
            Ok(slug)
        }
        _ => Err(ConversionError::BadWebstoreUrl),
    }
}

/// Makes a Chrome Web Store URL from an extension ID and slug.
///
/// # Errors
///
/// Returns an error if the extension ID is invalid.
///
/// # Examples
///
/// ```
/// use nanocrx::webstore_url_from_id_and_slug;
///
/// webstore_url_from_id_and_slug("iginnfkhmmfhlkagcmpgofnjhanpmklb", "boxel-rebound").unwrap();
/// ```
pub fn webstore_url_from_id_and_slug(id: &str, slug: &str) -> Result<String, ConversionError> {
    if id.len() != 32 || !id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit()) {
        return Err(ConversionError::InvalidExtensionId);
    }

    Ok(format!(
        "https://chromewebstore.google.com/detail/{}/{}",
        slug, id
    ))
}

/// Makes an extension ID from a `CRX ID`.
///
/// # Errors
///
/// Returns an error if the CRX ID is invalid.
///
/// # Examples
///
/// ```
/// use nanocrx::extension_id_from_crx_id;
///
/// extension_id_from_crx_id(&[0; 16]).unwrap();
/// ```
pub fn extension_id_from_crx_id(crx_id: &[u8]) -> Result<String, ConversionError> {
    if crx_id.len() != 16 {
        return Err(ConversionError::InvalidCrxId);
    }

    let mut result = String::with_capacity(32);

    for &byte in crx_id {
        let high = byte >> 4;
        let low = byte & 0x0F;

        result.push((b'a' + high) as char);
        result.push((b'a' + low) as char);
    }

    Ok(result)
}