use anyhow::Result;
use log::LevelFilter;

pub fn init_logger() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .try_init()?;  // Remove map_err
    Ok(())
}