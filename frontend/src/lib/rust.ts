type RustRequest<T = unknown> = {
  requestId: string;
  cmd: string;
  args: T;
};

type RustResponse<T = unknown> = {
  requestId: string;
  success: boolean;
  data?: T;
  error?: string;
};

declare global {
  interface Window {
    chrome?: {
      webview?: {
        postMessage: (message: string) => void;
        addEventListener: (
          type: "message",
          listener: (event: { data: string }) => void,
          options?: { signal?: AbortSignal },
        ) => void;
        removeEventListener: (
          type: "message",
          listener: (event: { data: string }) => void,
        ) => void;
      };
    };
  }
}

export const rustBridge = {
  invoke: async <T>(command: string, args?: unknown): Promise<T> => {
    return new Promise((resolve, reject) => {
      if (!window.chrome?.webview) {
        reject(new Error("WebView bridge not available"));
        return;
      }

      const requestId = crypto.randomUUID();
      const abortController = new AbortController();

      const timeoutId = window.setTimeout(() => {
        abortController.abort();
        reject(new Error(`Request timed out after 5000ms (${command})`));
      }, 5000);

      const messageHandler = (event: { data: string | object }) => {
        try {
          console.debug("[WebView] Raw message received:", event.data);

          // Handle both string and object data
          const rawData =
            typeof event.data === "string"
              ? event.data
              : JSON.stringify(event.data);

          // Parse the message envelope
          const envelope: unknown = JSON.parse(rawData);

          if (typeof envelope !== "object" || envelope === null) {
            throw new Error("Invalid response format");
          }

          // Type guard for RustResponse
          const response = envelope as RustResponse<T>;

          if (response.requestId !== requestId) return;

          window.clearTimeout(timeoutId);
          window.chrome?.webview?.removeEventListener(
            "message",
            messageHandler,
          );

          if (!response.success || !response.data) {
            throw new Error(response.error ?? "Request failed");
          }

          // Directly use the already-parsed data
          resolve(response.data);
        } catch (error) {
          window.clearTimeout(timeoutId);
          window.chrome?.webview?.removeEventListener(
            "message",
            messageHandler,
          );
          reject(error instanceof Error ? error : new Error("Unknown error"));
        }
      };

      window.chrome.webview.addEventListener("message", messageHandler, {
        signal: abortController.signal,
      });

      try {
        const request: RustRequest = {
          requestId,
          cmd: command,
          args: args ?? null,
        };
        window.chrome.webview.postMessage(JSON.stringify(request));
      } catch (error) {
        abortController.abort();
        reject(
          error instanceof Error ? error : new Error("Failed to send message"),
        );
      }
    });
  },
};
