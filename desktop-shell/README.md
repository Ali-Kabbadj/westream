Here's how to restructure the `desktop-shell` to align with the architecture doc and ensure scalability:

### **Directory Structure**
```rust
desktop-shell/
├── src/
│   ├── main.rs          // App entrypoint + core initialization
│   ├── window/          // Window management module
│   │   ├── mod.rs       // Window creation, resize handlers
│   │   ├── styling.rs   // Window chrome customization
│   │   └── messaging.rs // Window event processing
│   ├── webview/         // WebView2 integration
│   │   ├── mod.rs       // WebView lifecycle management
│   │   ├── bindings.rs  // JS-Rust interop
│   │   └── router.rs    // URL routing/navigation
│   ├── services/        // Core subsystems
│   │   ├── mod.rs       // Service coordinator
│   │   ├── playback/    // Stream engine integration
│   │   │   ├── mod.rs
│   │   │   ├── mpv.rs   // MPV player control
│   │   │   └── adaptive.rs
│   │   ├── addons/      // Addon UI management
│   │   │   ├── mod.rs
│   │   │   ├── store.rs // Addon marketplace
│   │   │   └── sandbox.rs
│   │   └── user/        // Auth/profile management
│   │       ├── mod.rs
│   │       ├── auth.rs  // OAuth flows
│   │       └── sync.rs  // Watch history sync
│   ├── ui/              // Visual components
│   │   ├── mod.rs       // UI root component
│   │   ├── components/  // Reusable widgets
│   │   └── layout/      // Window layout managers
│   ├── config/          // Persistent configuration
│   │   ├── mod.rs       // Settings manager
│   │   ├── paths.rs     // Data/cache locations
│   │   └── defaults.rs  // Fallback values
│   └── utils/
│       ├── error.rs     // Unified error handling
│       └── logging.rs   // Tracing setup
├── resources/           // Static assets
│   ├── web/            // Web resources for WebView
│   └── icons/          
└── Cargo.toml
```

### **Key Module Responsibilities**

1. **Window Module (`window/`)**
```rust
// window/mod.rs
pub struct WindowConfig {
    pub size: (i32, i32),
    pub position: (i32, i32),
    pub transparency: bool
}

pub fn create_window(config: WindowConfig) -> Result<HWND> {
    // Win32 window creation logic
}

// Handles DPI scaling and resize events
pub fn handle_resize(hwnd: HWND, new_size: (i32, i32)) {
    // Update WebView bounds
    // Adjust UI layout
}
```

2. **WebView Module (`webview/`)**
```rust
// webview/mod.rs
pub struct WebViewManager {
    controller: ICoreWebView2Controller,
    webview: ICoreWebView2
}

impl WebViewManager {
    pub fn navigate(&self, url: &str) -> Result<()> {
        // Implement navigation logic
    }
    
    pub fn register_js_handler(&self, name: &str, handler: Callback) {
        // Expose Rust functions to JS
    }
}
```

3. **Services Layer (`services/`)**
```rust
// services/mod.rs
pub struct ServiceManager {
    pub playback: Arc<playback::PlaybackService>,
    pub addons: Arc<addons::AddonManager>,
    pub user: Arc<user::UserService>
}

impl ServiceManager {
    pub fn init() -> Result<Self> {
        // Initialize all subsystems
    }
}
```

### **Critical Integration Points**

1. **Window-WebView Binding**
```rust
// main.rs
let webview_manager = WebViewManager::create(hwnd)?;
webview_manager.navigate("https://app.stremio")?;

// Connect window resize to WebView bounds
window::register_resize_handler(hwnd, move |size| {
    webview_manager.update_bounds(size);
});
```

2. **Service Initialization**
```rust
// main.rs
let services = ServiceManager::init()
    .context("Failed to initialize services")?;

// Inject services into WebView JS context
webview_manager.register_js_handler("getAddons", {
    let addon_service = services.addons.clone();
    move || addon_service.list_addons()
});
```

### **Cargo.toml Updates**
```toml
[dependencies]
# Existing dependencies
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
parking_lot = "0.12" # For thread-safe service access
reqwest = { version = "0.11", features = ["json"] } # For service HTTP calls
```

### **Development Workflow**

1. **Core Window System First**
```bash
cargo run --features window-basic # Work on window management
```

2. **Layer Services Gradually**
```bash
cargo run --features window,webview # Add WebView integration
cargo run --features full # Enable all services
```

### **Key Architectural Benefits**

1. **Vertical Isolation**
- Window management knows nothing about addons
- WebView layer only handles presentation
- Services are completely decoupled

2. **Horizontal Scalability**
- New service? Add under `services/` and register in `ServiceManager`
- New UI component? Add to `ui/components/`
- New window type? Create `window/specialized.rs`

3. **Consistent Error Handling**
```rust
// utils/error.rs
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Window error: {0}")]
    Window(#[from] window::WindowError),
    
    #[error("WebView error: {0}")]
    WebView(#[from] webview::WebViewError),
    
    // Unified error type across all modules
}
```

