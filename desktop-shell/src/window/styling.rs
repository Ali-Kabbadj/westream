// Window chrome customization
use windows::Win32::Foundation::HWND;

use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongW, SetWindowLongW, GWL_STYLE, WS_OVERLAPPEDWINDOW};



pub fn apply_window_styles(hwnd: HWND) {

}


 // Remove thickframe for borderless style (optional)

 #[allow(dead_code)]
 pub fn apply_borderless_style(hwnd: HWND) {
    unsafe {
        // Remove thickframe for borderless style (optional)
        let style = GetWindowLongW(hwnd, GWL_STYLE);
        SetWindowLongW(hwnd, GWL_STYLE, style & !(WS_OVERLAPPEDWINDOW.0 as i32));
    }
}