use crate::error::ConversionError;

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
/// assert!(id_from_webstore_url("https://chromewebstore.google.com/detail/boxel-rebound/iginnfkhmmfhlkagcmpgofnjhanpmklb?utm_source=chatgpt.com").is_ok());
/// assert!(id_from_webstore_url("chromewebstore.google.com/detail/boxel-rebound/iginnfkhmmfhlkagcmpgofnjhanpmklb").is_ok());
/// assert!(id_from_webstore_url("https://chromewebstore.google.com/detail/cool-extension/invalid-id").is_err());
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
/// assert!(webstore_url_from_id_and_slug("iginnfkhmmfhlkagcmpgofnjhanpmklb", "boxel-rebound").is_ok());
/// assert!(webstore_url_from_id_and_slug("invalid-id", "cool_extension").is_err());
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