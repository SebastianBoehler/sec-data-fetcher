import typescriptParser from '@typescript-eslint/parser'; // Import the parser
import typescriptPlugin from '@typescript-eslint/eslint-plugin'; // Import the plugin

export default [
  {
    ignores: ['node_modules/**', 'dist/**'],
  },
  {
    files: ['**/*.ts'],
    languageOptions: {
      ecmaVersion: 2018,
      sourceType: 'module',
      parser: typescriptParser, // Use the imported parser object
    },
    plugins: {
      '@typescript-eslint': typescriptPlugin, // Use the plugin as an object
    },
    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/explicit-module-boundary-types': 'off',
      '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
    },
  },
];
