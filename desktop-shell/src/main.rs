mod config;
mod utils;
mod window;
mod webview;
mod services;
mod ui;

use anyhow::{Context, Result};
use log::info;
use webview::manager;
use windows::Win32::{
    System::Com,
    UI::WindowsAndMessaging::{SetWindowLongPtrW, GWLP_USERDATA},
};

fn main() -> Result<()> {
    utils::logging::init_logger()?;
    info!("Starting desktop-shell");
    
    unsafe { Com::CoInitializeEx(None, Com::COINIT_APARTMENTTHREADED).ok().context("COM init failed")? };

    let config = config::load().context("Config load failed")?;
    let hwnd = window::create_window(&config.window)?;

    let service_manager = std::sync::Arc::new(
        services::ServiceManager::init().context("Service manager init failed")?
    );

    let webview_manager = manager::WebViewManager::create(
        hwnd,
        config.webview.user_data_path.to_str().context("Invalid data path")?,
        config.webview.initial_url,
        config.webview.width,
        config.webview.height,
        service_manager.clone(),
    )?;

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