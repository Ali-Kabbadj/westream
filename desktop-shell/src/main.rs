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
    // Initialize logger and COM
    utils::logging::init_logger()?;
    info!("Starting desktop-shell");

    unsafe { Com::CoInitializeEx(Some(std::ptr::null()), Com::COINIT_APARTMENTTHREADED) }
        .ok()
        .context("COM initialization failed")?;

    // Load config
    let config = config::load().context("Failed to load config")?;

    // Create window
    let hwnd = window::create_window(&config.window).context("Window creation failed")?;

    // Create WebView on the main thread
    let _webview = webview::WebViewManager::create(
        hwnd,
        config.webview.user_data_path.to_str().context("Invalid user data path")?,
        config.webview.initial_url,
        config.webview.width,
        config.webview.height
    ).context("WebView initialization failed")?;

    // Run the main message loop
    window::run_message_loop(hwnd)?;

    // Cleanup
    unsafe { Com::CoUninitialize() };
    Ok(())
}