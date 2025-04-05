import { defineConfig } from "vite";

export default defineConfig({
  optimizeDeps: {
    exclude: ["ecies-wasm"], // otherwise wasm file will not be copied into .vite/deps
  },
  server: {
    fs: {
      // Allow serving files from one level up to the project root
      allow: [".."],
    },
  },
});
