import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import { resolve } from "path";

export default defineConfig({
  plugins: [solid()],
  resolve: {
    alias: [
      { find: "@", replacement: resolve(__dirname, "./app") },
      { find: "~", replacement: resolve(__dirname, "./app/lib") },
    ],
  },
  server: {
    port: 3001,
  },
  build: {
    manifest: true,
    rollupOptions: {
      input: "app/main.tsx",
    },
  },
});
