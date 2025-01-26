/**
 * Run `build` or `dev` with `SKIP_ENV_VALIDATION` to skip env validation. This is especially useful
 * for Docker builds.
 */
import "./src/env.js";

/** @type {import("next").NextConfig} */
// const config = {

//   output: "export",
//   distDir: "../desktop-shell/src/resources/web",
//   images: { unoptimized: true },
//   // Enable WebView-compatible routing
//   trailingSlash: true,
//   // Disable runtime-intensive features
//   experimental: {
//     optimizePackageImports: ["@radix-ui/react"],
//   },
//   swcMinify: true,
// };

const config = {
  // Keep existing config but add dev server settings
  output: process.env.NODE_ENV === "production" ? "export" : undefined,
  distDir:
    process.env.NODE_ENV === "production"
      ? "../desktop-shell/src/resources/web"
      : undefined,
  // Enable CORS for WebView
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
