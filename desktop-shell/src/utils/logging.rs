use anyhow::Result;
use log::LevelFilter;

pub fn init_logger() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .format_timestamp_millis()
        .format_module_path(true)
        .try_init()?;
    Ok(())
}
