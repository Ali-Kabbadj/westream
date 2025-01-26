// Window creation, resize handlersuse anyhow::{Context, Result};
use anyhow::{Context, Result};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::*,
    Graphics::Gdi::UpdateWindow,
};
use crate::config::WindowConfig;


mod styling;
mod messaging;



pub fn create_window(config: &WindowConfig) -> Result<HWND> {

    let hwnd = unsafe {
        messaging::create_window_instance(&config.title, config.width, config.height)
            .context("Win32 window creation failed")?
    };


    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = UpdateWindow(hwnd);
    }

    Ok(hwnd)
}


pub fn run_message_loop(hwnd: HWND) -> Result<()> {
    let mut msg = MSG::default();
    while unsafe { GetMessageW(&mut msg, None, 0, 0) }.into() {
        unsafe {
            
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}