// use std::sync::Arc;

// use anyhow::Result;
// use windows::{
//     core::*,
//     Win32::{
//         Foundation::*, Graphics::Gdi::{GetStockObject, HBRUSH, WHITE_BRUSH}, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::*
//     },
//     Win32::UI::WindowsAndMessaging::WM_USER,
// };


// pub unsafe fn create_window_instance(title: &str, width: i32, height: i32) -> Result<HWND> {
//     let hinstance = GetModuleHandleW(None)?;
//     let class_name = w!("StremioWindowClass");

//     let wc = WNDCLASSW {
//         style: CS_HREDRAW | CS_VREDRAW,
//         lpfnWndProc: Some(wndproc),
//         hInstance: HINSTANCE(hinstance.0),
//         lpszClassName: class_name,
//         hbrBackground: HBRUSH(GetStockObject(WHITE_BRUSH).0),
//         ..Default::default()
//     };

//     let atom = RegisterClassW(&wc);
//     if atom == 0 {
//         anyhow::bail!("Failed to register window class");
//     }

//     let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();

//     let hwnd = match CreateWindowExW(
//         WS_EX_APPWINDOW,
//         class_name,
//         PCWSTR(title_wide.as_ptr()),
//         WS_OVERLAPPEDWINDOW,
//         CW_USEDEFAULT,
//         CW_USEDEFAULT,
//         width,
//         height,
//         None,
//         None,
//         Some(HINSTANCE(hinstance.0)),
//         None,
//     ) {
//         Ok(hwnd) => hwnd,
//         Err(e) => return Err(e.into()),
//     };

//     Ok(hwnd)
// }

// use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, WM_SIZE};
// use windows::Win32::Foundation::RECT;

// use crate::{services::ServiceManager, webview::manager::{self, WebViewManager}};

// #[allow(unused_variables)]
// extern "system" fn wndproc(
//     hwnd: HWND,
//     msg: u32,
//     wparam: WPARAM,
//     lparam: LPARAM,
// ) -> LRESULT {
//     match msg {
//         val if val == WM_USER + 1 => {
//         let response = unsafe { Box::from_raw(lparam.0 as *mut String) };
//         log::debug!("Received response: {}", response);
        
//         // Handle the response here (e.g., send to webview)
//         LRESULT(0)
//     }
//        WM_DESTROY => {
//             // Safely clean up WebViewManager
//             log::info!("WM_DESTROY received for HWND {:?}", hwnd);
//             let webview_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut WebViewManager;

//             if !webview_ptr.is_null() {
//                 unsafe {
//                     // Reconstruct the Box to invoke Drop
//                     let _ = Box::from_raw(webview_ptr);
//                     SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
//                 }
//             }
//             unsafe { PostQuitMessage(0) };
//             LRESULT(0)
//         }
//         WM_SIZE => {
//             // Get client area dimensions
//             log::debug!("WM_SIZE - HWND validity: {}", unsafe { IsWindow(Some(hwnd)).as_bool() });

//             let mut client_rect = RECT::default();
//             unsafe { let _ = GetClientRect(hwnd, &mut client_rect); };
            
//             let width = client_rect.right - client_rect.left;
//             let height = client_rect.bottom - client_rect.top;

//             // Retrieve WebViewManager from window user data
//             let webview_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut manager::WebViewManager;
//             if !webview_ptr.is_null() {
//                 let webview_manager = unsafe { &*webview_ptr };
//                 let _ = webview_manager.resize(width, height);
//             }

//             let managers_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } 
//                 as *mut (WebViewManager, Arc<ServiceManager>);
            
//             if !managers_ptr.is_null() {
//                 let (webview_manager, service_manager) = unsafe { &*managers_ptr };
//                 // Handle async operations here
//             }

//             LRESULT(0)
//         }
//         _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
//     }
// }


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
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}