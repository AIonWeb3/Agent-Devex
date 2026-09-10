import tseslint from "typescript-eslint";

export default tseslint.config(
  { ignores: ["dist/**"] },
  {
    files: ["src/**/*.ts"],
    languageOptions: {
      parser: tseslint.parser,
      parserOptions: { project: false },
    },
  },
);
