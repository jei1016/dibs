import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import path from "path";

export default defineConfig({
  plugins: [svelte(), tailwindcss()],
  resolve: {
    alias: {
      // Resolve $lib for dibs-admin's shadcn components
      "$lib": path.resolve(__dirname, "../../packages/dibs-admin/src/lib"),
    },
  },
});
