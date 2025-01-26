// App entrypoint + core initialization
mod config;
mod utils;
mod window;
mod webview;
mod services;
mod ui;

use std::mem::ManuallyDrop;

use anyhow::{Context, Result};
use log::info;
use webview::manager::{self, WebViewManager};
use windows::Win32::{System::Com, UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, GWLP_USERDATA}};



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

      // Initialize ServiceManager and wrap in Arc
    let service_manager = std::sync::Arc::new(
        services::ServiceManager::init().context("Failed to create service manager")?
    );




     // Create WebViewManager with explicit Arc clone
    let webview_manager = manager::WebViewManager::create(
        hwnd,
        config.webview.user_data_path.to_str().context("Invalid user data path")?,
        config.webview.initial_url,
        config.webview.width,
        config.webview.height,
        service_manager.clone(), 
    )?;

    

    let webview_manager = ManuallyDrop::new(webview_manager); 

   // Store as a raw pointer in window user data
    unsafe {
        SetWindowLongPtrW(
            hwnd,
            GWLP_USERDATA,
            &*webview_manager as *const _ as isize
        );
    }

    //  // Add memory validation
    // #[cfg(target_os = "windows")]
    // unsafe {
    //     windows::Win32::System::Diagnostics::Debug::SetThreadErrorMode(
    //         windows::Win32::System::Diagnostics::Debug::SEM_FAILCRITICALERRORS,
    //         std::ptr::null_mut(),
    //     )?;
    // }
    // Run the main message loop
    window::run_message_loop(hwnd)?;


    // Cleanup
    unsafe { Com::CoUninitialize() };
    Ok(())
}