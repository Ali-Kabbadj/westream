
// use anyhow::Result;
// use windows::{
//     core::*,
//     Win32::{
//         Foundation::*,
//         UI::WindowsAndMessaging::*,
//         Graphics::Gdi::{GetStockObject, HBRUSH, WHITE_BRUSH},
//         System::LibraryLoader::GetModuleHandleW,
//     },
// };

// pub unsafe fn create_window_instance(config: &crate::config::WindowConfig) -> Result<HWND> {
//     let hmodule = GetModuleHandleW(None)?;
//     let hinstance = HINSTANCE(hmodule.0); // Convert HMODULE to HINSTANCE
//     let class_name = w!("StremioWindowClass");

//     let wc = WNDCLASSW {
//         style: CS_HREDRAW | CS_VREDRAW,
//         lpfnWndProc: Some(wndproc),
//         hInstance: hinstance,
//         lpszClassName: class_name,
//         hbrBackground: HBRUSH(GetStockObject(WHITE_BRUSH).0),
//         ..Default::default()
//     };

//     RegisterClassW(&wc);
//     let title_wide: Vec<u16> = config.title.encode_utf16().chain(std::iter::once(0)).collect();

//     let hwnd = CreateWindowExW(
//         WINDOW_EX_STYLE::default(),
//         class_name,
//         PCWSTR(title_wide.as_ptr()),
//         WS_OVERLAPPEDWINDOW | WS_THICKFRAME,
//         config.position.0,
//         config.position.1,
//         config.width,
//         config.height,
//         None,
//         None,
//         Some(hinstance),
//         None,
//     )?;

//     Ok(hwnd)
// }

// extern "system" fn wndproc(
//     hwnd: HWND,
//     msg: u32,
//     wparam: WPARAM,
//     lparam: LPARAM,
// ) -> LRESULT {
//     match msg {
//         WM_DESTROY => {
//             unsafe { PostQuitMessage(0) };
//             LRESULT(0)
//         }
//         _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
//     }
// }

use anyhow::Result;
use windows::{
    core::*,
    Win32::{
        Foundation::*, Graphics::Gdi::{GetStockObject, HBRUSH, WHITE_BRUSH}, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::*
    },
};

pub unsafe fn create_window_instance(title: &str, width: i32, height: i32) -> Result<HWND> {
    let hinstance = GetModuleHandleW(None)?;
    let class_name = w!("StremioWindowClass");

    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        hInstance: HINSTANCE(hinstance.0),
        lpszClassName: class_name,
        hbrBackground: HBRUSH(GetStockObject(WHITE_BRUSH).0),
        ..Default::default()
    };

    let atom = RegisterClassW(&wc);
    if atom == 0 {
        anyhow::bail!("Failed to register window class");
    }

    let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();

    let hwnd = match CreateWindowExW(
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
        Some(HINSTANCE(hinstance.0)),
        None,
    ) {
        Ok(hwnd) => hwnd,
        Err(e) => return Err(e.into()),
    };

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
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}