function sendToNative(message) {
  const msg = {
    cmd: message.command,
    args: message.data || {},
    __timestamp: Date.now(),
  };
  window.chrome.webview.postMessage(JSON.stringify(msg));
}
