use std::sync::Mutex;

use anyhow::{Context, Result};
use serde_json::{from_str, json, to_string, Value};
mod addons;
mod metadata;
mod playback;

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
        log::debug!("Received message: {}", message);
        let value: Value = from_str(message).map_err(|e| {
            log::error!("JSON parse error: {}", e);
            e
        })?;

        let cmd = value["cmd"]
            .as_str()
            .context("Missing command")
            .map_err(|e| {
                log::error!("Command error: {}", e);
                e
            })?;
        let request_id = value["requestId"].as_str().unwrap_or_default();
        let args = &value["args"];
        log::info!(
            "Handling command '{}' (Request ID: {}) (Args: {})",
            cmd,
            request_id,
            args
        );

        log::info!("Handling command: {}", cmd);

        let guard = match self.mock_metadata.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                log::error!("Mutex poisoned! Attempting recovery");
                poisoned.into_inner()
            }
        };

        let response = match cmd {
            "getCatalog" => {
                log::debug!("Processing getCatalog command");
                let catalog = guard.get_catalog();
                serde_json::to_value(catalog)?
            }
            _ => return Err(anyhow::anyhow!("Unknown command: {}", cmd)),
        };
        log::debug!("Sending response: {}", response);
        Ok(to_string(&json!({
                "requestId": value["requestId"].as_str().unwrap_or_default(),
                "success": true,
                "data": response
        }))?)
    }
}
