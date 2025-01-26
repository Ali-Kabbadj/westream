type RustCommand = {
  cmd: "search" | "play" | "getConfig";
  args: unknown;
};

export const rustBridge = {
  invoke: <T>(command: RustCommand): Promise<T> => {
    return new Promise((resolve, reject) => {
      if (!window.__RUST_BRIDGE) {
        return reject("Rust bridge not initialized");
      }

      const channel = new MessageChannel();
      channel.port1.onmessage = ({ data }) => {
        if (data.error) reject(data.error);
        else resolve(data.payload);
      };

      window.postMessage(command, "*", [channel.port2]);
    });
  },
};
