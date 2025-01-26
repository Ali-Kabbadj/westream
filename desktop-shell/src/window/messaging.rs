use anyhow::Result;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::{GetStockObject, HBRUSH, WHITE_BRUSH},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

use crate::webview::manager;

pub(super) unsafe fn create_window_instance(title: &str, width: i32, height: i32) -> Result<HWND> {
    let hinstance = GetModuleHandleW(None)?;
    let class_name = w!("StremioWindowClass");

    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        hInstance: hinstance.into(),
        lpszClassName: class_name,
        hbrBackground: HBRUSH(GetStockObject(WHITE_BRUSH).0),
        ..Default::default()
    };

    if RegisterClassW(&wc) == 0 {
        anyhow::bail!("Failed to register window class");
    }

    let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();

    let hwnd = CreateWindowExW(
        WS_EX_APPWINDOW,
        class_name,
        PCWSTR(title_wide.as_ptr()),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        width,
        height,
        None,
        None,
        Some(hinstance.into()),
        None,
    )?;

    Ok(hwnd)
}


extern "system" fn wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            // Proper cleanup sequence
            unsafe {
                let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut manager::WebViewManager;
                if !ptr.is_null() {
                    let _ = Box::from_raw(ptr);
                }
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        val if val == WM_USER + 1 => {
            // Handle web messages safely
            let response = unsafe { Box::from_raw(lparam.0 as *mut String) };
            log::debug!("Web response: {}", response);
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    }
}