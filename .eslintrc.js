// .eslintrc.js

module.exports = {
  parser: '@typescript-eslint/parser', // Specifies the ESLint parser
  parserOptions: {
    ecmaVersion: 2018, // Allows for parsing modern ECMAScript features
    sourceType: 'module', // Allows for the use of imports
  },
  plugins: ['@typescript-eslint'],
  extends: [
    'eslint:recommended', // Uses the recommended rules from ESLint
    'plugin:@typescript-eslint/recommended', // Uses the recommended rules from @typescript-eslint
    'prettier', // Enables eslint-plugin-prettier and displays prettier errors as ESLint errors
  ],
  rules: {
    // Place to specify ESLint rules
    '@typescript-eslint/no-explicit-any': 'off', // Allow usage of 'any' type
    '@typescript-eslint/explicit-module-boundary-types': 'off', // Allow implicit return types
    '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }], // Ignore unused variables that start with '_'
    // Add more custom rules as needed
  },
};
