import tseslint from '@electron-toolkit/eslint-config-ts'
import eslintConfigPrettier from '@electron-toolkit/eslint-config-prettier'
import eslintPluginSvelte from 'eslint-plugin-svelte'

export default tseslint.config(
	{ ignores: ['**/node_modules', '**/dist', '**/out', '**/build'] },
	tseslint.configs.recommended,
	eslintPluginSvelte.configs['flat/recommended'],
	{
		files: ['**/*.svelte', '**/*.svelte.ts'],
		languageOptions: {
			parserOptions: {
				parser: tseslint.parser
			}
		}
	},
	{
		files: ['**/*.svelte', '**/*.svelte.ts'],
		rules: {
			'no-undef': 'off'
		}
	},
	{
		files: ['**/*.svelte'],
		rules: {
			'@typescript-eslint/explicit-function-return-type': 'off'
		}
	},
	eslintConfigPrettier
)
