use anyhow::{Context, Result};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{HWND, RECT},
        UI::WindowsAndMessaging::{PeekMessageW, PM_REMOVE, TranslateMessage, DispatchMessageW, MSG},
    },
};
use webview2_com::Microsoft::Web::WebView2::Win32::{
    CreateCoreWebView2EnvironmentWithOptions, ICoreWebView2, ICoreWebView2Controller, ICoreWebView2Environment,
};

use webview2_com::CreateCoreWebView2EnvironmentCompletedHandler;
use webview2_com::CreateCoreWebView2ControllerCompletedHandler;



pub struct WebViewManager {
    _controller: ICoreWebView2Controller,
    _webview: ICoreWebView2,
}

impl WebViewManager {
    pub fn create(
        hwnd: HWND,
        user_data_path: &str,
        initial_url: String,
        width: i32,
        height: i32,
    ) -> Result<Self> {
        let user_data_path = HSTRING::from(user_data_path);
        let initial_url = HSTRING::from(initial_url);

        // Create environment
        let (env_sender, env_receiver) = std::sync::mpsc::channel();

        let env_handler = CreateCoreWebView2EnvironmentCompletedHandler::create(
            Box::new(move |result: windows::core::Result<()>, environment: Option<ICoreWebView2Environment>| {
                let result: Result<(), windows::core::Error> = result.map_err(|e| e.into());
                env_sender.send((result, environment.clone())).unwrap();
                Ok(())
            })
        );

        unsafe {
            CreateCoreWebView2EnvironmentWithOptions(
                None,
                PCWSTR::from_raw(user_data_path.as_ptr()),
                None,
                &env_handler,
            )
            .context("Failed to create WebView2 environment")?;
        }



        // Pump messages while waiting for environment
        let environment = loop {
            let mut msg = MSG::default();
            while unsafe { PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() } {
                unsafe {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }

            if let Ok((result, environment)) = env_receiver.try_recv() {
                result.context("Failed to create environment")?;
                break environment.context("No environment returned")?;
            }
        };

        // Create controller
        let (ctrl_sender, ctrl_receiver) = std::sync::mpsc::channel();
        // In the controller creation section:
        let ctrl_handler = CreateCoreWebView2ControllerCompletedHandler::create(
            Box::new(move |result: windows::core::Result<()>, controller: Option<ICoreWebView2Controller>| {
                ctrl_sender.send((result, controller.clone())).unwrap();
                Ok(())
            })
        );

        unsafe {
            environment.CreateCoreWebView2Controller(hwnd, &ctrl_handler)
                .context("Failed to create controller")?;
        }

        // Pump messages while waiting for controller
        let controller = loop {
            let mut msg = MSG::default();
            while unsafe { PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() } {
                unsafe {
                    if TranslateMessage(&msg).0 == 0 {
                        // Handle error if needed
                    }
                    DispatchMessageW(&msg);
                }
            }

            if let Ok((result, controller)) = ctrl_receiver.try_recv() {
                result.context("Failed to create controller")?;
                break controller.context("No controller returned")?;
            }
        };

        // Configure WebView
        let webview = unsafe { controller.CoreWebView2() }
            .context("Failed to get WebView from controller")?;


        let bounds = RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        };
        unsafe { controller.SetBounds(bounds) }
            .context("Failed to set WebView bounds")?;

        unsafe { webview.Navigate(PCWSTR::from_raw(initial_url.as_ptr())) }
            .context("Failed to navigate to initial URL")?;

        Ok(Self {
            _controller: controller,
            _webview: webview,
        })
    }

    pub fn resize(&self, width: i32, height: i32) -> Result<()> {
        let bounds = RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        };
        unsafe { self._controller.SetBounds(bounds) }
            .context("Failed to resize WebView bounds")?;
        Ok(())
    }
}