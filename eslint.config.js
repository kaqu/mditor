const js = require("@eslint/js");
const globals = require("globals");

/** @type {import('eslint').Linter.FlatConfig[]} */
module.exports = [
  {
    ignores: ["node_modules/**"],
  },
  {
    files: ["**/*.{js,ts,tsx}"],
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: {
        ...globals.node,
        ...globals.es2021,
      },
    },
    rules: {
      ...js.configs.recommended.rules,
    },
  },
];
