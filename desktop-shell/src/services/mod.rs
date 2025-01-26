use anyhow::Result;
mod playback;
mod addons;
mod metadata;

#[allow(dead_code)]
// src/services/mod.rs
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
}