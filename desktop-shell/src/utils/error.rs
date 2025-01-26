use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Window error: {0}")]
    WindowError(String),

    #[error("WebView error: {0}")]
    WebViewError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
    
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

impl From<windows::core::Error> for AppError {
    fn from(e: windows::core::Error) -> Self {
        AppError::WebViewError(e.to_string())
    }
}