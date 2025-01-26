use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::PathBuf;


pub fn config_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "Stremio", "DesktopShell")
        .context("Couldn't determine config directory")?;
    Ok(proj_dirs.config_dir().to_path_buf())
}

pub fn config_file() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.json"))
}

pub fn webview_data_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "Stremio", "DesktopShell")
        .context("Couldn't determine data directory")?;
    Ok(proj_dirs.data_dir().join("webview_data"))
}