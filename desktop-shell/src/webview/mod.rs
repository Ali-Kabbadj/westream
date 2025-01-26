// WebView lifecycle management
use anyhow::{Context, Result};
use webview2::EnvironmentBuilder;
use windows::Win32::Foundation::HWND;

pub struct WebViewManager {
    environment: webview2::Environment,
    controller: webview2::Controller,
    webview: webview2::WebView,
}

impl WebViewManager {
    pub fn create(hwnd: HWND, config: &crate::config::WebViewConfig) -> Result<Self> {
        let (env_sender, env_receiver) = std::sync::mpsc::channel();
        
        EnvironmentBuilder::new()
            .with_user_data_folder(config.user_data_path.as_path())
            .build(move |result| {
                let _ = env_sender.send(result);
                Ok(())
            })
            .context("Failed to build environment")?;

        let environment = env_receiver.recv()??;
        
        let (ctrl_sender, ctrl_receiver) = std::sync::mpsc::channel();
        environment.create_controller(hwnd.0 as _, move |result| {
            let _ = ctrl_sender.send(result);
            Ok(())
        })?;
        
        let controller = ctrl_receiver.recv()??;
        let webview = controller.get_webview()?;
        webview.navigate(&config.initial_url)?;

        Ok(Self {
            environment,
            controller,
            webview,
        })
    }

    pub fn navigate(&self, url: &str) -> Result<()> {
        self.webview.navigate(url)?;
        Ok(())
    }
}