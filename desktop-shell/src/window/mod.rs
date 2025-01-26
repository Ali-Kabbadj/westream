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
        messaging::create_window_instance(config)
            .context("Win32 window creation failed")?
    };

   styling::apply_window_styles(hwnd);

    unsafe {
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
    }

    Ok(hwnd)
}

pub fn run_message_loop(_hwnd: HWND) -> Result<()> {
    let mut msg = MSG::default();
    while unsafe { GetMessageW(&mut msg, HWND(0), 0, 0) }.into() {
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}