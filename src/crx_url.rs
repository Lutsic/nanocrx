use crate::crx_url_error::CrxUrlError;
use crate::validation::{validate_prodversion, validate_id};

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
pub struct CrxUrl {
    extension_id: String,
    prodversion: String,
    allow_crx2: bool,
}


impl CrxUrl {
    /// Creates new `CrxUrl` using extension ID.
    ///
    /// The product version defaults to `9999.0.0.0`,
    /// and `CRX2` fallback is enabled by default.
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
    /// CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb");
    /// ```
    pub fn new(extension_id: &str) -> Result<Self, CrxUrlError>{
        if !validate_id(extension_id) {
            return Err(CrxUrlError::InvalidId);
        }
        Ok(Self {
            extension_id: extension_id.to_string(),
            prodversion: String::from("9999.0.0.0"),
            allow_crx2: true,
        })
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
    /// CrxUrl::new("iginnfkhmmfhlkagcmpgofnjhanpmklb").expect("REASON").prodversion("155.0.0.0");
    /// ```
    pub fn prodversion(mut self, prodversion: &str) -> Result<Self, CrxUrlError> {
        if !validate_prodversion(prodversion) {
            return Err(CrxUrlError::InvalidProdVersion);
        }
        self.prodversion = prodversion.to_string();
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
            self.extension_id
        )
    }
}