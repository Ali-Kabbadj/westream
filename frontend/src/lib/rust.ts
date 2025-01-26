type RustCommand = {
  cmd: string;
  args?: unknown;
};

type RustResponse<T = unknown> = {
  success: boolean;
  data?: T;
  error?: string;
};

declare global {
  interface Window {
    chrome?: {
      webview?: {
        postMessage: (message: string) => void; // Changed to only accept strings
      };
    };
  }
}

export const rustBridge = {
  invoke: async <T>(command: string, args?: unknown): Promise<T> => {
    return new Promise((resolve, reject) => {
      if (!window.chrome?.webview) {
        reject("WebView bridge not available");
        return;
      }

      // Create a unique ID for this request
      const requestId = Math.random().toString(36).substring(2, 15);

      // Set up message handler for response
      const messageHandler = (event: MessageEvent) => {
        try {
          const data: RustResponse<T> = JSON.parse(event.data);

          // Check if this is the response for our request
          if (data.requestId === requestId) {
            window.removeEventListener("message", messageHandler);

            if (data.error) {
              reject(new Error(data.error));
            } else if (data.success && data.data !== undefined) {
              resolve(data.data);
            } else {
              reject(new Error("Invalid response from Rust"));
            }
          }
        } catch (e) {
          window.removeEventListener("message", messageHandler);
          reject(e);
        }
      };

      window.addEventListener("message", messageHandler);

      // Send command as JSON string
      window.chrome.webview.postMessage(
        JSON.stringify({
          requestId,
          cmd: command,
          args,
        }),
      );
    });
  },

  // Add event listener support
  onMessage: (callback: (message: any) => void) => {
    if (window.chrome?.webview) {
      window.addEventListener("message", (event) => {
        try {
          // Parse the message data
          const data = JSON.parse(event.data);
          callback(data);
        } catch (e) {
          console.error("Failed to parse message:", e);
        }
      });
    }
  },
};
