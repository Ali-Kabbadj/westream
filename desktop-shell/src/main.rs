// App entrypoint + core initialization
mod config;
mod utils;
mod window;
mod webview;
mod services;

use anyhow::{Context, Result};
use log::info;
use windows::Win32::System::Com;


fn main() -> Result<()> {
    utils::logging::init_logger()?;
    info!("Starting desktop-shell");

    unsafe { Com::CoInitializeEx(None, Com::COINIT_APARTMENTTHREADED) }
        .context("COM initialization failed")?;

    let config = config::load().context("Failed to load config")?;
    let hwnd = window::create_window(&config.window).context("Window creation failed")?;

    let webview = webview::WebViewManager::create(hwnd, &config.webview)
        .context("WebView initialization failed")?;

    let services = services::ServiceManager::init()
        .context("Service initialization failed")?;

    window::run_message_loop(hwnd)?;

    unsafe { Com::CoUninitialize() };
    Ok(())
}