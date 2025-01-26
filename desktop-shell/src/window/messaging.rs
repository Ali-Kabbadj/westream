// Window event processing
use anyhow::Result;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        UI::WindowsAndMessaging::*,
        Graphics::Gdi::{GetStockObject, HBRUSH, WHITE_BRUSH}, // Add HBRUSH here
        System::LibraryLoader::GetModuleHandleW,
    },
};


pub unsafe fn create_window_instance(config: &crate::config::WindowConfig) -> Result<HWND> {
    let instance = GetModuleHandleW(None)?;
    let class_name = w!("StremioWindowClass");

    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        hInstance: instance.into(),
        lpszClassName: class_name,
        hbrBackground: HBRUSH(GetStockObject(WHITE_BRUSH).0),
        ..Default::default()
    };

    RegisterClassW(&wc);
    let title_wide: Vec<u16> = config.title.encode_utf16().chain(std::iter::once(0)).collect();

    let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        class_name,
        PCWSTR(title_wide.as_ptr()),
        WS_OVERLAPPEDWINDOW,
        config.position.0,
        config.position.1,
        config.width,
        config.height,
        None,
        None,
        instance,
        None,
    );
    if hwnd.0 == 0 {
        return Err(anyhow::anyhow!("Window creation failed"));
    }

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