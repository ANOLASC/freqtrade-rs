import js from '@eslint/js';
import globals from 'globals';
import react from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';
import unusedImports from 'eslint-plugin-unused-imports';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  // Ignore patterns
  {
    ignores: ['dist/', 'node_modules/', '*.config.*', '**/*.css'],
  },

  // JavaScript/TypeScript files
  {
    files: ['**/*.{ts,tsx,js,jsx}'],
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
    },
    settings: {
      react: {
        version: '18.3',
      },
    },
    plugins: {
      react,
      'react-hooks': reactHooks,
      'unused-imports': unusedImports,
    },
    rules: {
      ...reactHooks.configs.recommended.rules,
      ...js.configs.recommended.rules,
      ...react.configs.recommended.rules,
      ...react.configs['jsx-runtime'].rules,
      'react/react-in-jsx-scope': 'off',
      'react-hooks/exhaustive-deps': 'off',
      'unused-imports/no-unused-imports': 'error',
      'no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
      '@typescript-eslint/no-explicit-any': 'off',
    },
  },

  // TypeScript specific
  ...tseslint.configs.recommended,

  // Global overrides
  {
    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
    },
  }
);
