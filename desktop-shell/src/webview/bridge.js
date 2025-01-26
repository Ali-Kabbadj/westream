window.__RUST_BRIDGE = {
  invoke: (command, args) =>
    new Promise((resolve, reject) => {
      const channel = new MessageChannel();
      channel.port1.onmessage = ({ data }) => {
        if (data.error) {
          reject(new Error(data.error));
        } else {
          resolve(data.data);
        }
      };

      window.chrome.webview.postMessage(
        {
          cmd: command,
          args: args,
        },
        [channel.port2],
      );
    }),
};
function sendToNative(message) {
  // BAD: window.chrome.webview.postMessage({ data: message });
  // GOOD: Send as JSON string
  window.chrome.webview.postMessage(JSON.stringify(message));
}
