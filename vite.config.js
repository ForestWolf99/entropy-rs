import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
  // prevent vite from obscuring rust errors
  //clearScreen: false,
  // Tauri expects a fixed port
  server: {
    strictPort: true,
  },

  envPrefix: ["VITE_", "TAURI_"],
  build: {
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  plugins: [vue()],
})
