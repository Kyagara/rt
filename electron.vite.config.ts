import { defineConfig, externalizeDepsPlugin } from 'electron-vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'
import { vite as vidstack } from 'vidstack/plugins'
import path from 'node:path'

export default defineConfig({
	main: {
		plugins: [externalizeDepsPlugin()],
		resolve: {
			alias: {
				$shared: path.resolve(__dirname, './src/shared')
			}
		}
	},
	preload: {
		plugins: [externalizeDepsPlugin()],
		resolve: {
			alias: {
				$shared: path.resolve(__dirname, './src/shared')
			}
		}
	},
	renderer: {
		plugins: [vidstack(), svelte(), tailwindcss()],
		resolve: {
			alias: {
				$lib: path.resolve(__dirname, './src/renderer/src/lib'),
				$shared: path.resolve(__dirname, './src/shared')
			}
		}
	}
})
