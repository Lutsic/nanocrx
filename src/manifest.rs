#![allow(missing_docs)]

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use crate::ManifestError;
use std::io::{Cursor, Read};
use zip::ZipArchive;

/// Represents both v2 and v3 manifests.
///
/// Provides access to the package's manifest data.
///
/// # Examples
///
/// ```no_run
/// use nanocrx::Crx3;
///
/// let crx = Crx3::open("extension.crx")?;
/// println!("Extension name: {}", crx.manifest().name);
/// # Ok::<(), nanocrx::CrxError>(())
/// ```
#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub manifest_version: u8,

    pub name: String,
    pub version: String,

    pub description: Option<String>,
    pub version_name: Option<String>,
    pub default_locale: Option<String>,

    #[serde(default)]
    pub permissions: Vec<String>,

    #[serde(default)]
    pub optional_permissions: Vec<String>,

    #[serde(default)]
    pub host_permissions: Vec<String>,

    #[serde(default)]
    pub optional_host_permissions: Vec<String>,

    #[serde(default)]
    pub icons: HashMap<String, String>,

    #[serde(default)]
    pub content_scripts: Vec<ContentScript>,

    pub background: Option<Background>,

    pub action: Option<ActionDetails>,
    pub browser_action: Option<ActionDetails>,
    pub page_action: Option<ActionDetails>,

    pub options_page: Option<String>,
    pub options_ui: Option<OptionsUi>,

    // different fields across manifest versions.
    pub content_security_policy: Option<Value>,
    pub web_accessible_resources: Option<Value>,

    // some extra bullshit 
    /// Extra fields that are not explicitly defined in the `Manifest` struct.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct ContentScript {
    pub matches: Vec<String>,

    #[serde(default)]
    pub js: Vec<String>,

    #[serde(default)]
    pub css: Vec<String>,

    #[serde(default)]
    pub exclude_matches: Vec<String>,

    pub run_at: Option<String>,
    pub all_frames: Option<bool>,
    pub world: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Background {
    #[serde(default)]
    pub scripts: Vec<String>,

    pub service_worker: Option<String>,
    pub page: Option<String>,
    pub persistent: Option<bool>,
    pub type_: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ActionDetails {
    pub default_title: Option<String>,
    pub default_popup: Option<String>,

    #[serde(default)]
    pub default_icon: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct OptionsUi {
    pub page: String,
    pub open_in_tab: Option<bool>,
}

pub fn parse_manifest(zip_data: Vec<u8>) -> Result<Manifest, ManifestError> {
    let cursor = Cursor::new(zip_data);
    let mut archive = ZipArchive::new(cursor)?;

    let mut file = archive.by_name("manifest.json")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let manifest = serde_json::from_str(&contents)?;
    Ok(manifest)
}