/*
 * Project:    ci-test-ts-simple
 * File:       eslint.config.js
 * License:    FSL-1.1-ALv2
 * Copyright:  (c) 2026 HYPERI PTY LIMITED
 */

import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ['src/**/*.ts'],
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  {
    ignores: ['dist/', 'node_modules/', 'coverage/'],
  },
);
