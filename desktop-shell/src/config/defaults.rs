use windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;

pub fn window_width() -> i32 { 800 }
pub fn window_height() -> i32 { 600 }
pub fn window_title() -> String { "Stremio Shell".into() }
pub fn window_position() -> (i32, i32) { (CW_USEDEFAULT, CW_USEDEFAULT) }
#[cfg(debug_assertions)] // Use dev server in debug mode
pub fn webview_initial_url() -> String {
    "http://localhost:3000".into()
}

#[cfg(not(debug_assertions))] // Use built files in release
pub fn webview_initial_url() -> String {
    format!("file://{}/resources/web/index.html", env!("CARGO_MANIFEST_DIR"))
}
pub fn webview_width() -> i32 {800 }
pub fn webview_height() -> i32 {600}
