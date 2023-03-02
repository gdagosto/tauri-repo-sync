import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { join } from 'path';
import { loadEnv } from 'vite';
import { ENV_DIR, ENV_PREFIX } from './constants.js';

// /analises
export default defineConfig(({ mode }) => {
	const envFolder = join(process.cwd(), ENV_DIR);

	// Salva a informação para o svelte.config.js
	process.env = {
		...process.env,
		...loadEnv(mode, envFolder, ENV_PREFIX),
		MODE: mode
	};

	return {
		plugins: [sveltekit()],
		test: {
			include: ['src/**/*.{test,spec}.{js,ts}']
		}
	};
});
