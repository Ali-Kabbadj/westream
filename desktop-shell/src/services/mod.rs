use anyhow::Result;
mod playback;
mod addons;

pub struct ServiceManager {
    pub playback: playback::PlaybackService,
    pub addons: addons::AddonManager,
}

impl ServiceManager {
    pub fn init() -> Result<Self> {
        Ok(Self {
            playback: playback::PlaybackService::new(),
            addons: addons::AddonManager::new(),
        })
    }
}