use windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;

pub fn window_width() -> i32 { 800 }
pub fn window_height() -> i32 { 600 }
pub fn window_title() -> String { "Stremio Shell".into() }
pub fn window_position() -> (i32, i32) { (CW_USEDEFAULT, CW_USEDEFAULT) }
pub fn webview_initial_url() -> String { "about:blank".into() }