import "./src/env.js";

/** @type {import("next").NextConfig} */

const config = {
  // Keep existing config but add dev server settings
  output: process.env.NODE_ENV === "production" ? "export" : undefined,
  distDir:
    process.env.NODE_ENV === "production"
      ? "../desktop-shell/src/resources/web"
      : undefined,
  // Enable CORS for WebView
  images: {
    domains: ["m.media-amazon.com"],
  },
  async headers() {
    return [
      {
        source: "/(.*)",
        headers: [
          {
            key: "Cross-Origin-Embedder-Policy",
            value: "require-corp",
          },
          {
            key: "Cross-Origin-Opener-Policy",
            value: "same-origin",
          },
        ],
      },
    ];
  },
};

export default config;
