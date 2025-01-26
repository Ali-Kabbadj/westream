use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;


mod paths;
mod defaults;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub window: WindowConfig,
    pub webview: WebViewConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: i32,
    pub height: i32,
    pub title: String,
    #[serde(default = "defaults::window_position")]
    pub position: (i32, i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebViewConfig {
    pub initial_url: String,
    pub user_data_path: PathBuf,
    pub width: i32,
    pub height: i32,
}

pub fn load() -> Result<AppConfig> {
    let config_path = paths::config_file()?;
    
    if config_path.exists() {
        let content = std::fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        let default_config = AppConfig {
            window: WindowConfig {
                width: defaults::window_width(),
                height: defaults::window_height(),
                title: defaults::window_title(),
                position: defaults::window_position(),
            },
            webview: WebViewConfig {
                initial_url: defaults::webview_initial_url(),
                user_data_path: paths::webview_data_dir()?,
                width: defaults::webview_width(),
                height: defaults::webview_height(),
            },
        };
        save(&default_config)?;
        Ok(default_config)
    }
}

// In config/mod.rs update the save() function:
pub fn save(config: &AppConfig) -> Result<()> {
    let config_path = paths::config_file()?;
    let config_dir = config_path.parent()
        .ok_or_else(|| anyhow!("Invalid config path"))?;
    
    // Create directory if missing
    std::fs::create_dir_all(config_dir)?;
    
    let content = serde_json::to_string_pretty(config)?;
    std::fs::write(config_path, content)?;
    Ok(())
}