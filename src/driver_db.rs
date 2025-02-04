use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct DriverManifest {
    pub drivers: HashMap<String, DriverEntry>,
}

#[derive(Debug, Deserialize)]
pub struct DriverEntry {
    pub url: String,
    pub checksum: String,
    pub min_version: String,
    pub compatible_hwids: Vec<String>,
}

pub struct DriverDB {
    manifest: DriverManifest,
}

impl DriverDB {
    pub async fn load() -> Result<Self, DbError> {
        let response = reqwest::get("https://drivers.intel.com/manifest.json")
            .await?
            .json()
            .await?;
        
        Ok(Self { manifest: response })
    }

    pub fn find_driver(&self, hwid: &str) -> Option<&DriverEntry> {
        self.manifest.drivers.values().find(|entry| 
            entry.compatible_hwids.iter().any(|id| hwid.starts_with(id))
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("Network error")]
    Reqwest(#[from] reqwest::Error),
}