import { Configuration } from 'electron-builder'

const config: Configuration = {
	productName: 'rt',
	appId: 'com.rt.app',
	artifactName: 'rt-${version}-${os}.${ext}',

	npmRebuild: false,
	publish: null,
	electronLanguages: ['en-US'],
	asarUnpack: ['resources/**'],
	directories: {
		buildResources: 'build'
	},
	files: [
		'!**/.vscode/*',
		'!src/*',
		'!{.env,.env.*,.npmrc}',
		'!{.gitignore,.gitattributes,README.md}',
		'!{electron.vite.config.ts,electron-builder-config.ts,svelte.config.mjs,tailwind.config.js}',
		'!{.prettierignore,.prettierrc,eslint.config.mjs}',
		'!{tsconfig.json,tsconfig.node.json,tsconfig.web.json}'
	],

	protocols: [
		{
			name: 'rt',
			schemes: ['rt'],
			role: 'Viewer'
		}
	],

	// Platforms

	win: {
		target: ['nsis']
	},
	nsis: {
		oneClick: false,
		shortcutName: '${productName}',
		uninstallDisplayName: '${productName}',
		createDesktopShortcut: 'always',
		deleteAppDataOnUninstall: true
	},

	mac: {
		entitlementsInherit: 'build/entitlements.mac.plist',
		identity: null,
		notarize: false,
		target: ['dmg']
	},

	linux: {
		target: ['deb'],
		category: 'Utility'
	}
}

export default config
