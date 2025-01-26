use anyhow::{Context, Result};
use serde_json::{from_str, json, to_string, Value};
mod playback;
mod addons;
mod metadata;


pub struct ServiceManager {
    pub playback: playback::PlaybackService,
    pub addons: addons::AddonManager,
    pub mock_metadata: metadata::MockMetadataService, // Add this
}

impl ServiceManager {
    pub fn init() -> Result<Self> {
        Ok(Self {
            playback: playback::PlaybackService::new(),
            addons: addons::AddonManager::new(),
            mock_metadata: metadata::MockMetadataService::new(), // Add this
        })
    }

   pub fn handle_web_message(&self, message: &str) -> Result<String> {
        let value: Value = from_str(message)?;
        let cmd = value["cmd"].as_str().context("Missing command")?;
        let args = &value["args"];

        let response = match cmd {
            "search" => {
                let query = args.as_str().unwrap_or_default();
                let results = self.mock_metadata.search(query);
                serde_json::to_value(results)?
            }
            "getCatalog" => {
                let catalog = self.mock_metadata.get_catalog();
                serde_json::to_value(catalog)?
            }
            _ => return Err(anyhow::anyhow!("Unknown command: {}", cmd)),
        };

        Ok(to_string(&json!({
            "success": true,
            "data": response
        }))?)
    }
}