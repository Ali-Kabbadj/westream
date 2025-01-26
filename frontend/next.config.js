/**
 * Run `build` or `dev` with `SKIP_ENV_VALIDATION` to skip env validation. This is especially useful
 * for Docker builds.
 */
import "./src/env.js";

/** @type {import("next").NextConfig} */
const config = {
  output: "export",
  distDir: "../desktop-shell/src/resources/web",
  images: { unoptimized: true },
  // Enable WebView-compatible routing
  trailingSlash: true,
  // Disable runtime-intensive features
  experimental: {
    optimizePackageImports: ["@radix-ui/react"],
  },
  swcMinify: true,
};

export default config;
