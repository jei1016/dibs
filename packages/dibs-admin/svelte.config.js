import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/** @type {import('@sveltejs/vite-plugin-svelte').SvelteConfig} */
export default {
  compilerOptions: {
    // Add any compiler options here
  },
  kit: undefined, // Not using SvelteKit
  vitePlugin: {
    // Configure the vite plugin
  }
};
