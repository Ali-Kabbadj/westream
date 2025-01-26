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
function sendToNative(message) {
  try {
    const jsonMessage = JSON.stringify({
      cmd: message.command,
      args: message.data || {},
      timestamp: Date.now(),
    });
    window.chrome.webview.postMessage(jsonMessage);
  } catch (e) {
    console.error("Message serialization failed:", e);
  }
}
