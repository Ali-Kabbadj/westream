
use anyhow::{Context, Result};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{E_POINTER, HWND, LPARAM, RECT, WPARAM},
        System::{Com, WinRT::EventRegistrationToken},
        UI::WindowsAndMessaging::{
            DispatchMessageW, IsWindow, PeekMessageW, PostMessageW, SetWindowLongPtrW, TranslateMessage,
            GWLP_USERDATA, MSG, PM_REMOVE, WM_USER,
        },
    },
};
use webview2_com::Microsoft::Web::WebView2::Win32::{
    CreateCoreWebView2EnvironmentWithOptions, ICoreWebView2, ICoreWebView2Controller,
    ICoreWebView2Environment, ICoreWebView2WebMessageReceivedEventArgs,
    ICoreWebView2WebMessageReceivedEventHandler, ICoreWebView2WebMessageReceivedEventHandler_Impl,
};
use webview2_com::{CreateCoreWebView2EnvironmentCompletedHandler, CreateCoreWebView2ControllerCompletedHandler};
use serde_json::json;
use crate::services::ServiceManager;


#[windows::core::implement(ICoreWebView2WebMessageReceivedEventHandler)] // Explicitly specify the macro's crate
struct WebMessageHandler {
    service_manager: std::sync::Arc<ServiceManager>,
    webview: ICoreWebView2,
    parent_hwnd: HWND,
}

impl WebMessageHandler {
    fn handle_message(&self, message: String) -> String {
        match self.service_manager.handle_web_message(&message) {
            Ok(r) => r,
            Err(e) => serde_json::to_string(&json!({
                "success": false,
                "error": e.to_string()
            })).unwrap_or_else(|_| r#"{"success":false}"#.into())
        }
    }
}


impl ICoreWebView2WebMessageReceivedEventHandler_Impl for WebMessageHandler_Impl {
    fn Invoke(
        &self,
        _sender: windows::core::Ref<'_, ICoreWebView2>,
        args: windows::core::Ref<'_, ICoreWebView2WebMessageReceivedEventArgs>,
    ) -> windows::core::Result<()> {
        let args = args.as_ref().ok_or_else(|| windows::core::Error::new(E_POINTER, "Null args"))?;
        
        let mut message = HSTRING::default();
        unsafe { args.TryGetWebMessageAsString(&mut message as *mut _ as _)?; }

        if message.is_empty() {
            log::warn!("Received empty WebMessage");
            return Ok(());
        }

        // Access fields via self.this instead of self.0
        let response = self.this.handle_message(message.to_string());

        unsafe {
            let lparam = Box::into_raw(Box::new(response)) as _;
            PostMessageW(
                Some(self.this.parent_hwnd), // Use self.this here
                WM_USER + 1,
                WPARAM(0),
                LPARAM(lparam)
            ).ok();
        }

        Ok(())
    }
}

pub struct WebViewManager {
    hwnd: HWND,
    _controller: ICoreWebView2Controller,
    webview: ICoreWebView2,
    _message_token: EventRegistrationToken,
    _handler: ICoreWebView2WebMessageReceivedEventHandler,
    service_manager: std::sync::Arc<ServiceManager>,
}



impl WebViewManager {
    pub fn create(
        hwnd: HWND,
        user_data_path: &str,
        initial_url: String,
        width: i32,
        height: i32,
        service_manager: std::sync::Arc<ServiceManager>,
    ) -> Result<Self> {
        let user_data_path = HSTRING::from(user_data_path);
        let initial_url = HSTRING::from(initial_url);

        // Create environment
        let (env_sender, env_receiver) = std::sync::mpsc::channel();
        let env_handler = CreateCoreWebView2EnvironmentCompletedHandler::create(
            Box::new(move |result: windows::core::Result<()>, environment: Option<ICoreWebView2Environment>| {
                unsafe { let _ = Com::CoInitializeEx(None, Com::COINIT_APARTMENTTHREADED).ok(); };
                let result: Result<(), windows::core::Error> = result.map_err(|e| e.into());
                let _ = env_sender.send((result, environment));
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

        let environment = loop {
            let mut msg = MSG::default();
            while unsafe { PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() } {
                unsafe {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }

            if let Ok((result, env)) = env_receiver.try_recv() {
                result.context("Environment creation failed")?;
                break env.context("Environment not returned")?;
            }
        };

        // Create controller
        let (ctrl_sender, ctrl_receiver) = std::sync::mpsc::channel();
        let ctrl_handler = CreateCoreWebView2ControllerCompletedHandler::create(
            Box::new(move |result: windows::core::Result<()>, controller: Option<ICoreWebView2Controller>| {
                let _ = ctrl_sender.send((result, controller));
                Ok(())
            })
        );

        unsafe {
            environment.CreateCoreWebView2Controller(hwnd, &ctrl_handler)
                .context("Failed to create controller")?;
        }

        let controller = loop {
            let mut msg = MSG::default();
            while unsafe { PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() } {
                unsafe {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }

            if let Ok((result, ctrl)) = ctrl_receiver.try_recv() {
                result.context("Controller creation failed")?;
                break ctrl.context("Controller not returned")?;
            }
        };

        let webview = unsafe { controller.CoreWebView2() }
            .context("Failed to get WebView")?;

        // Navigate to initial URL
        unsafe { webview.Navigate(PCWSTR::from_raw(initial_url.as_ptr())) }
            .context("Failed to navigate")?;

        // Inject JavaScript bridge
        let js_bridge = include_str!("./bridge.js");
        unsafe {
            webview.AddScriptToExecuteOnDocumentCreated(
                &HSTRING::from(js_bridge),
                None,
            )?;
        }

        let mut message_token = EventRegistrationToken::default();
        let handler: ICoreWebView2WebMessageReceivedEventHandler = WebMessageHandler {
            service_manager: service_manager.clone(),
            webview: webview.clone(),
            parent_hwnd: hwnd,
        }.into();

        unsafe {
            webview.add_WebMessageReceived(&handler, &mut message_token)?;
        }

        // Set initial bounds
        let bounds = RECT { left: 0, top: 0, right: width, bottom: height };
        unsafe { controller.SetBounds(bounds) }
            .context("Failed to set bounds")?;

        Ok(Self {
            hwnd,
            _controller: controller,
            webview,
            _message_token: message_token,
            _handler: handler,
            service_manager,
        })
    }

    pub fn resize(&self, width: i32, height: i32) -> Result<()> {
        let bounds = RECT { left: 0, top: 0, right: width, bottom: height };
        unsafe { self._controller.SetBounds(bounds) }
            .context("Resize failed")
    }
}

impl Drop for WebViewManager {
    fn drop(&mut self) {
        log::info!("Dropping WebViewManager");
        unsafe {
            let _ = self.webview.remove_WebMessageReceived(self._message_token);
            let _ = self._controller.Close();
            if IsWindow(Some(self.hwnd)).as_bool() {
                SetWindowLongPtrW(self.hwnd, GWLP_USERDATA, 0);
            }
        }
    }
}