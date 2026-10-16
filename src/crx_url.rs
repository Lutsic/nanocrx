use crate::error::CrxUrlError;


/// Builds download URLs for Chrome Web Store extensions.
///
/// `CrxUrl` allows configuring the extension ID, Chrome product version,
/// and accepted CRX formats before generating a download URL.
///
/// # Examples
///
/// ```
/// use nanocrx::CrxUrl;
///
/// let url = CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb")?
///     .prodversion("155.0.0.0")?
///     .allow_crx2(false)
///     .to_url();
///
/// assert!(url.contains("acceptformat=crx3"));
/// # Ok::<(), nanocrx::CrxUrlError>(())
/// ``` 
#[derive(Clone)]
pub struct CrxUrl {
    id: String,
    prodversion: String,
    allow_crx2: bool,
}

impl CrxUrl {
    /// Creates new `CrxUrl` using extension ID.
    ///
    /// The product version defaults to `9999.0.0.0`,
    /// and `CRX2` fallback is disabled by default.
    ///
    /// # Errors
    ///
    /// Return an error if extension ID has invalid format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nanocrx::CrxUrl;
    ///
    /// assert!(CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb").is_ok());
    /// assert!(CrxUrl::new("something").is_err());
    /// ```
    pub fn new(id: impl Into<String>) -> Result<Self, CrxUrlError> {
        let id = id.into();
        validate_id(&id)?;

        Ok(Self {
            id,
            prodversion: String::from("9999.0.0.0"),
            allow_crx2: false,
        })
    }

    /// Builds a Chrome Web Store update URL for downloading an extension.
    ///
    /// The generated URL uses the extension ID, product version,
    /// and `CRX` formats configured in this `CrxUrl`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nanocrx::CrxUrl;
    ///
    /// CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb").expect("REASON").to_url();
    /// ```
    pub fn to_url (&self) -> String {
        let acceptformat = if self.allow_crx2 {
            "crx2,crx3"
        } else {
            "crx3"
        };

        format!(
            "https://clients2.google.com/service/update2/crx?response=redirect&prodversion={}&acceptformat={}&x=id%3D{}%26installsource%3Dondemand%26uc",
            self.prodversion,
            acceptformat,
            self.id
        )
    }
    
    /// Sets a Chrome product version parameter.
    ///
    /// A product version must contain between 2 and 4 numeric components
    /// separated by dots (`.`).
    ///
    /// # Errors
    ///
    /// Returns an error if the product version has invalid format.
    ///
    /// # Examples
    ///
    /// ```
    /// use nanocrx::CrxUrl;
    ///
    /// assert!(CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb").expect("REASON").prodversion("155.0.0.0").is_ok());
    /// assert!(CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb").expect("REASON").prodversion("hi").is_err());
    /// ```
    pub fn prodversion(mut self, version: impl Into<String>) -> Result<Self, CrxUrlError> {
        let version = version.into();
        validate_prodversion(&version)?;

        self.prodversion = version;
        Ok(self)
    }
    
    /// Configures whether `CRX2` is included in the accepted formats.
    ///
    /// When enabled, the generated URL requests both `CRX2` and `CRX3`.
    /// Otherwise, it requests only `CRX3`.
    ///
    /// This setting does not guarantee which format the server will return.
    ///
    /// # Examples
    ///
    /// ```
    /// use nanocrx::CrxUrl;
    ///
    /// CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb").expect("REASON").allow_crx2(true);
    /// 
    /// ```
    pub fn allow_crx2(mut self, allow: bool) -> Self {
        self.allow_crx2 = allow;
        self
    }
}

/// Validates a Chrome Web Store extension ID.
///
/// An extension ID must contain exactly 32 characters,
/// each in the range `a` through `p`.
///
/// # Errors
///
/// Returns an error if the ID has an invalid length
/// or contains an invalid character.
///
/// # Examples
///
/// ```
/// use nanocrx::validate_id;
///
/// assert!(validate_id("abcdefghijklmnopabcdefghijklmnop").is_ok());
/// assert!(validate_id("abcdefghijklmnopabcdefghijkwxyzq").is_err());
/// assert!(validate_id("something").is_err());
/// assert!(validate_id("12345abcde").is_err());
/// assert!(validate_id("").is_err());
/// ```
pub fn validate_id (id: &str) -> Result<(), CrxUrlError> {
    if id.len() != 32 || !id.bytes().all(|b| (b'a'..=b'p').contains(&b)) {
        return Err(CrxUrlError::InvalidId);
    }
    Ok(())
}

/// Validates format of a Chrome product version parameter.
///
/// A product version must contain from 2 to 4 numeric components separated by `.`.
///
/// # Errors
///
/// Returns an error if the product version has invalid character, or does not contain any `.`.
///
/// # Examples
///
/// ```
/// use nanocrx::validate_prodversion;
///
/// assert!(validate_prodversion("155.0.8059.81").is_ok());
/// assert!(validate_prodversion("149.0").is_ok());
/// assert!(validate_prodversion("149").is_err());
/// assert!(validate_prodversion("hi").is_err());
/// assert!(validate_prodversion("").is_err());
/// ```
pub fn validate_prodversion (prodversion: &str) -> Result<(), CrxUrlError> {
    if prodversion.is_empty() || !prodversion.chars().all(|c| c.is_ascii_digit() || c == '.') || !prodversion.contains(".") {
        return Err(CrxUrlError::InvalidProdVersion);
    }
    Ok(())
}
