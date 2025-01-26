use anyhow::Result;
mod playback;
mod addons;

#[allow(dead_code)]
pub struct ServiceManager {
    pub playback: playback::PlaybackService,
    pub addons: addons::AddonManager,
}

#[allow(dead_code)]
impl ServiceManager {
    pub fn init() -> Result<Self> {
        Ok(Self {
            playback: playback::PlaybackService::new(),
            addons: addons::AddonManager::new(),
        })
    }
}