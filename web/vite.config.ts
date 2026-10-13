import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  server: {
    // `npm run dev` talks to a running Akoyo backend (`cargo run`).
    proxy: { '/api': 'http://localhost:7700' },
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
});
