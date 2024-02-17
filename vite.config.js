import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite"
import Components from "unplugin-vue-components/vite"
import { quasar, transformAssetUrls } from '@quasar/vite-plugin'

export default defineConfig(async () => ({
  plugins: [
    vue({
      template: { transformAssetUrls }
    }),
    quasar()
  ],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/Rust/**"],
    },
  }
}));
