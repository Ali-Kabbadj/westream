// src/webview/bindings.rs
use crate::services::ServiceManager;
use serde_json::json;

pub struct JsBridge {
    services: ServiceManager,
}

#[derive(serde::Deserialize)]
struct BridgeMessage {
    cmd: String,
    args: Option<String>,
}

impl JsBridge {
    pub fn handle_message(&self, message: String) -> String {
        let msg: BridgeMessage = match serde_json::from_str(&message) {
            Ok(m) => m,
            Err(_) => return json!({"error": "Invalid message format"}).to_string(),
        };

        match msg.cmd.as_str() {
            "search" => {
                let query = msg.args.unwrap_or_default();
                let results = self.services.mock_metadata.search(&query);
                json!({"data": results}).to_string()
            },
            "getCatalog" => {
                let results = self.services.mock_metadata.get_catalog();
                json!({"data": results}).to_string()
            },
            _ => json!({"error": "Unknown command"}).to_string(),
        }
    }
}