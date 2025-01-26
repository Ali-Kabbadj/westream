use anyhow::Result;
use log::LevelFilter;

pub fn init_logger() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug) 
        .try_init()?;
    Ok(())
}