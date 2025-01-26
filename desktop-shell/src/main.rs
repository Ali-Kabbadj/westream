// App entrypoint + core initialization
mod config;
mod utils;
mod window;
mod webview;
mod services;
mod ui;

use anyhow::{Context, Result};
use log::info;
use windows::Win32::System::Com;



fn main() -> Result<()> {
    let _ui = ui::UiManager::new();

    utils::logging::init_logger()?;
    info!("Starting desktop-shell");

    unsafe { Com::CoInitializeEx(Some(std::ptr::null()), Com::COINIT_APARTMENTTHREADED) }
    .ok() 
    .context("COM initialization failed")?;

    let config = config::load().context("Failed to load config")?;
    let hwnd = window::create_window(&config.window).context("Window creation failed")?;

    // Add underscores:
    let _webview = webview::WebViewManager::create(hwnd, &config.webview)
        .context("WebView initialization failed")?;

    let _services = services::ServiceManager::init()
        .context("Service initialization failed")?;

    window::run_message_loop(hwnd)?;

    unsafe { Com::CoUninitialize() };
    Ok(())
}