import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
	plugins: [wasm()],
	optimizeDeps: {
		exclude: ['complex-visualizer'], // ← prevent Vite from prebundling it
	},
});
