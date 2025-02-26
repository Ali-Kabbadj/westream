// Window chrome customization
use windows::Win32::Foundation::HWND;

use windows::Win32::UI::HiDpi::{
    SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongW, SetWindowLongW, GWL_STYLE, WS_CAPTION, WS_OVERLAPPED, WS_OVERLAPPEDWINDOW,
    WS_SYSMENU, WS_VISIBLE,
};

#[allow(dead_code)]
pub fn configure_webview_window(hwnd: HWND) {
    unsafe {
        // Enable WebView-compatible styles
        SetWindowLongW(
            hwnd,
            GWL_STYLE,
            (WS_VISIBLE | WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU).0 as i32,
        );

        // Set DPI awareness
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
}

#[allow(dead_code)]
pub fn apply_borderless_style(hwnd: HWND) {
    unsafe {
        // Remove thickframe for borderless style (optional)
        let style = GetWindowLongW(hwnd, GWL_STYLE);
        SetWindowLongW(hwnd, GWL_STYLE, style & !(WS_OVERLAPPEDWINDOW.0 as i32));
    }
}
