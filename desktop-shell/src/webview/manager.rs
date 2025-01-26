use anyhow::{Context, Result};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{E_FAIL, E_POINTER, HWND, RECT}, System::WinRT::EventRegistrationToken, UI::WindowsAndMessaging::{DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE}
    },
};
use webview2_com::Microsoft::Web::WebView2::Win32::{
    CreateCoreWebView2EnvironmentWithOptions, ICoreWebView2, ICoreWebView2Controller, ICoreWebView2Environment, ICoreWebView2WebMessageReceivedEventArgs, ICoreWebView2WebMessageReceivedEventHandler, ICoreWebView2WebMessageReceivedEventHandler_Impl
};

use webview2_com::CreateCoreWebView2EnvironmentCompletedHandler;
use webview2_com::CreateCoreWebView2ControllerCompletedHandler;

use crate::services::ServiceManager;

#[windows::core::implement(ICoreWebView2WebMessageReceivedEventHandler)]
struct WebMessageHandler {
    service_manager: std::sync::Arc<ServiceManager>,
    webview: ICoreWebView2,
}


// impl ICoreWebView2WebMessageReceivedEventHandler_Impl for WebMessageHandler_Impl {
//     fn Invoke(
//         &self,
//         _sender: windows::core::Ref<'_, ICoreWebView2>,
//         args: windows::core::Ref<'_, ICoreWebView2WebMessageReceivedEventArgs>,
//     ) -> windows::core::Result<()> {
//         let args = args.as_ref().ok_or_else(|| windows::core::Error::new(E_POINTER, "Null args"))?;
        
//         let mut message = HSTRING::default();
//         unsafe {
//             args.TryGetWebMessageAsString(&mut message as *mut _ as _)?;
//         }
        
//         let response = self.service_manager.handle_web_message(&message.to_string())
//             .map_err(|e| windows::core::Error::new(E_FAIL, e.to_string()))?;
        
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
            args.TryGetWebMessageAsString(&mut message as *mut _ as _)?;
        }
        
        // Handle JSON string
        let response = self.service_manager.handle_web_message(&message.to_string())
            .map_err(|e| windows::core::Error::new(E_FAIL, e.to_string()))?;

        // Send response back as string
        unsafe { self.webview.PostWebMessageAsString(&HSTRING::from(response)) }?;
        
        Ok(())
    }
}


pub struct WebViewManager {
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
        service_manager: std::sync::Arc<ServiceManager>,
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
            _controller: controller,
            webview,
            _message_token: message_token,
            service_manager,
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