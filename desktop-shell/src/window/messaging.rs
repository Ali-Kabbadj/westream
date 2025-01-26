// Window event processing
use super::WindowConfig;
use anyhow::Result;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        UI::WindowsAndMessaging::*,
        Graphics::Gdi::GetStockObject,
        System::LibraryLoader::GetModuleHandleW,
        Graphics::Gdi::WHITE_BRUSH
    },
};


pub unsafe fn create_window_instance(config: &WindowConfig) -> Result<HWND> {
    let instance = GetModuleHandleW(None)?;
    let class_name = w!("StremioWindowClass");

    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        hInstance: instance.into(),
        lpszClassName: class_name,
        hbrBackground: GetStockObject(WHITE_BRUSH),
        ..Default::default()
    };

    RegisterClassW(&wc);

    CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        class_name,
        HSTRING::from(config.title.as_str()).as_pcwstr(),
        WS_OVERLAPPEDWINDOW,
        config.position.0,
        config.position.1,
        config.width,
        config.height,
        None,
        None,
        instance,
        None,
    ).ok().context("Window creation failed")
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