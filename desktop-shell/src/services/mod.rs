use std::sync::Mutex;

use anyhow::{Context, Result};
use serde_json::{from_str, json, to_string, Value};
mod playback;
mod addons;
mod metadata;


#[allow(dead_code)]
pub struct ServiceManager {
    playback: Mutex<playback::PlaybackService>,
    addons: Mutex<addons::AddonManager>,
    mock_metadata: Mutex<metadata::MockMetadataService>,
}

impl ServiceManager {
    pub fn init() -> Result<Self> {
        Ok(Self {
            playback: playback::PlaybackService::new().into(),
            addons: addons::AddonManager::new().into(),
            mock_metadata: metadata::MockMetadataService::new().into(), // Add this
        })
    }

 pub fn handle_web_message(&self, message: &str) -> Result<String> {
    let value: Value = from_str(message)?;
    let cmd = value["cmd"].as_str().context("Missing command")?;
    let args = &value["args"];

    let guard = self.mock_metadata.lock().unwrap();

        let response = match cmd {
            "search" => {
                let query = args.as_str().unwrap_or_default();
                let results = guard.search(query);
                serde_json::to_value(results)?
            }
            "getCatalog" => {
                let catalog = guard.get_catalog();
                serde_json::to_value(catalog)?
            }
            _ => return Err(anyhow::anyhow!("Unknown command: {}", cmd)),
        };
        log::debug!("Sending response: {}", response);
        Ok(to_string(&json!({
            "success": true,
            "data": response
        }))?)
    }
}