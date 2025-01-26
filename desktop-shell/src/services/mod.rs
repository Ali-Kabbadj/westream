use anyhow::Result;
use log::warn;

pub struct ServiceManager;

impl ServiceManager {
    pub fn init() -> Result<Self> {
        warn!("Services module not fully implemented");
        Ok(Self)
    }
}