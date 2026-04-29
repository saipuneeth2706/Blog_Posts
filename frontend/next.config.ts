import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  turbopack: {
    // Keep Turbopack scoped to the frontend app to avoid scanning backend artifacts.
    root: ".",
  },
};

export default nextConfig;
