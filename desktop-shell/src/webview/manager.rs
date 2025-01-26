use anyhow::{Context, Result};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{E_FAIL, E_POINTER, HWND, RECT}, System::WinRT::EventRegistrationToken, UI::WindowsAndMessaging::{DispatchMessageW, IsWindow, PeekMessageW, SetWindowLongPtrW, TranslateMessage, GWLP_USERDATA, MSG, PM_REMOVE}
    },
};
use webview2_com::Microsoft::Web::WebView2::Win32::{
    CreateCoreWebView2EnvironmentWithOptions, ICoreWebView2, ICoreWebView2Controller, ICoreWebView2Environment, ICoreWebView2WebMessageReceivedEventArgs, ICoreWebView2WebMessageReceivedEventHandler, ICoreWebView2WebMessageReceivedEventHandler_Impl
};

use webview2_com::CreateCoreWebView2EnvironmentCompletedHandler;
use webview2_com::CreateCoreWebView2ControllerCompletedHandler;

use crate::services::ServiceManager;
// import json
use serde_json::json;

#[windows::core::implement(ICoreWebView2WebMessageReceivedEventHandler)]
struct WebMessageHandler {
    service_manager: std::sync::Arc<ServiceManager>,
    webview: ICoreWebView2,
}

// Removed duplicate Drop implementation

// impl ICoreWebView2WebMessageReceivedEventHandler_Impl for WebMessageHandler_Impl {
//     fn Invoke(
//         &self,
//         _sender: windows::core::Ref<'_, ICoreWebView2>,
//         args: windows::core::Ref<'_, ICoreWebView2WebMessageReceivedEventArgs>,
//     ) -> windows::core::Result<()> {
//         log::info!("ICoreWebView2WebMessageReceivedEventHandler_Impl for WebMessageHandler_Impl");
//         let args = args.as_ref().ok_or_else(|| windows::core::Error::new(E_POINTER, "Null args"))?;
        
//         let mut message = HSTRING::default();
//         if message.is_empty() {
//             log::warn!("Received empty WebMessage");
//             return Ok(()); // Early return instead of crashing
//         }
//         unsafe {
//             args.TryGetWebMessageAsString(&mut message as *mut _ as _)?;
//         }
        
//         // Handle JSON string
//         let response = self.service_manager.handle_web_message(&message.to_string())
//             .map_err(|e| windows::core::Error::new(E_FAIL, e.to_string()))?;

//         log::debug!("Posting response: {}", response);

//         // Send response back as string
//         unsafe { self.webview.PostWebMessageAsString(&HSTRING::from(response)) }?;
        
//         Ok(())
//     }
// }

impl ICoreWebView2WebMessageReceivedEventHandler_Impl for WebMessageHandler_Impl {
    fn Invoke(
        &self,
        _sender: windows::core::Ref<'_, ICoreWebView2>,
        args: windows::core::Ref<'_, ICoreWebView2WebMessageReceivedEventArgs>,
    ) -> windows::core::Result<()> {
        let args = args.as_ref().ok_or_else(|| windows::core::Error::new(E_POINTER, "Null args"))?;
        
        let mut message = HSTRING::default();
        unsafe {
            // 1. First retrieve the message
            args.TryGetWebMessageAsString(&mut message as *mut _ as _)?;
        }

        // 2. Validate message after retrieval
        if message.is_empty() {
            log::warn!("Received empty WebMessage");
            return Ok(());
        }

        let message_str = message.to_string();
        log::debug!("Received WebMessage: {}", message_str);

        // 3. Add JSON validation
        let response = match self.service_manager.handle_web_message(&message_str) {
            Ok(r) => r,
            Err(e) => {
                log::error!("Message handling failed: {}", e);
                // Return error response to frontend
                serde_json::to_string(&json!({
                    "success": false,
                    "error": e.to_string()
                })).unwrap_or_else(|_| r#"{"success":false,"error":"serialization failed"}"#.into())
            }
        };

        // 4. Validate response before sending
        if response.is_empty() {
            log::warn!("Empty response generated");
            return Ok(());
        }

        log::debug!("Posting response: {}", response);
        unsafe { 
            self.webview.PostWebMessageAsString(&HSTRING::from(response))?;
        }
        
        Ok(())
    }
}


pub struct WebViewManager {
    hwnd: HWND, // Add this field to store the window handle
    _controller: ICoreWebView2Controller,
    webview: ICoreWebView2,
    _message_token: EventRegistrationToken,
    service_manager: std::sync::Arc<ServiceManager>,
}

impl WebViewManager {
    pub fn create(
        hwnd: HWND,
        user_data_path: &str,
        initial_url: String,
        width: i32,
        height: i32,
        service_manager: std::sync::Arc<ServiceManager>, // Arc is retained here
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
        log::debug!("Created WebView2 controller {:?}", controller);


        // Configure WebView
       let webview = unsafe { controller.CoreWebView2() }
            .context("Failed to get WebView from controller")?;
        log::debug!("Created WebView2 instance {:?}", webview);


        // Navigate to initial URL
        unsafe { webview.Navigate(PCWSTR::from_raw(initial_url.as_ptr())) }
            .context("Failed to navigate to initial URL")?;

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
        }.into();

        unsafe {
            webview.add_WebMessageReceived(
                &handler,
                &mut message_token,
            )
        }?;


        // Set bounds and return
        let bounds = RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        };
        unsafe { controller.SetBounds(bounds) }
            .context("Failed to set WebView bounds")?;

      
        Ok(Self {
            hwnd,
            _controller: controller,
            webview,
            _message_token: message_token,
            service_manager, // Now actively referenced
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

impl Drop for WebViewManager {
    fn drop(&mut self) {
        log::info!("Dropping WebViewManager");
        unsafe {
            // Remove WebView message handler
            let _ = self.webview.remove_WebMessageReceived(self._message_token);

            // if let Some(controller) = self._controller.as_raw() {
            //     controller.Release();
            // }
            // if let Some(webview) = self.webview.as_raw() {
            //     webview.Release();
            // }

            // Clear window user data to prevent dangling pointers
             if IsWindow(Some(self.hwnd)).as_bool() {
                SetWindowLongPtrW(self.hwnd, GWLP_USERDATA, 0);
            }
        }
        // _controller and webview are automatically released by windows-rs
    }
}


