// window.__RUST_BRIDGE = {
//   invoke: (command, args) =>
//     new Promise((resolve, reject) => {
//       const channel = new MessageChannel();
//       channel.port1.onmessage = ({ data }) => {
//         if (data.error) {
//           reject(new Error(data.error));
//         } else {
//           resolve(data.data);
//         }
//       };

//       window.chrome.webview.postMessage(
//         {
//           cmd: command,
//           args: args,
//         },
//         [channel.port2],
//       );
//     }),
// };
// Modify sendToNative to match Rust expectations:
function sendToNative(message) {
  const msg = {
    cmd: message.command,
    args: message.data || {},
    __timestamp: Date.now(),
  };
  window.chrome.webview.postMessage(JSON.stringify(msg));
}
