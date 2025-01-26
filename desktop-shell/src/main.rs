mod config;
mod services;
mod ui;
mod utils;
mod webview;
mod window;

use anyhow::{Context, Result};
use webview::manager;
use windows::Win32::{
    System::Com,
    UI::WindowsAndMessaging::{SetWindowLongPtrW, GWLP_USERDATA},
};

fn main() -> Result<()> {
    utils::logging::init_logger()?;
    log::info!("Starting application initialization");

    unsafe {
        Com::CoInitializeEx(None, Com::COINIT_APARTMENTTHREADED)
            .ok()
            .context("COM init failed")?
    };

    let config = config::load().context("Config load failed")?;
    let hwnd = window::create_window(&config.window)?;

    let service_manager = std::sync::Arc::new(
        services::ServiceManager::init().context("Service manager init failed")?,
    );

    log::info!("Window created successfully");
    log::info!("WebView manager creation starting");
    let webview_manager = manager::WebViewManager::create(
        hwnd,
        config
            .webview
            .user_data_path
            .to_str()
            .context("Invalid data path")?,
        config.webview.initial_url,
        config.webview.width,
        config.webview.height,
        service_manager.clone(),
    )
    .context("WebView creation failed")
    .map_err(|e| {
        log::error!("WebView creation error: {}", e);
        e
    })?;
    log::info!("WebView manager created successfully");

    // Store in window user data
    let boxed = Box::new(webview_manager);
    unsafe {
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(boxed) as isize);
    }

    window::run_message_loop(hwnd)?;

    // COM cleanup
    unsafe { Com::CoUninitialize() };
    Ok(())
}
