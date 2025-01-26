/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
// src/lib/rust.ts
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

// Extend Window interface properly
declare global {
  interface Window {
    chrome?: {
      webview?: {
        postMessage: (message: string) => void;
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

      // Safe timeout handling
      const timeoutId = window.setTimeout(() => {
        abortController.abort();
        reject(new Error(`Request timed out after 5000ms (${command})`));
      }, 5000);

      const messageHandler = (event: MessageEvent) => {
        try {
          const rawData =
            event.data instanceof Object
              ? JSON.stringify(event.data)
              : event.data;
          const data: unknown = JSON.parse(rawData);

          if (typeof data !== "object" || data === null) {
            throw new Error("Invalid response format");
          }

          const response = data as RustResponse<T>;

          if (response.requestId !== requestId) return;

          window.clearTimeout(timeoutId);
          window.removeEventListener("message", messageHandler);

          if (response.error) {
            throw new Error(response.error);
          }

          if (!response.success || !response.data) {
            throw new Error("Request failed without error message");
          }

          resolve(response.data);
        } catch (error) {
          window.clearTimeout(timeoutId);
          window.removeEventListener("message", messageHandler);
          reject(error instanceof Error ? error : new Error("Unknown error"));
        }
      };

      window.addEventListener("message", messageHandler, {
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
