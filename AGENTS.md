# Project Agents Guide

This file defines rules for OpenAI Codex and other AI agents working in this repository.

## Reference Documents
- `specification.md` describes the application requirements and architecture.
- `todo.md` lists implementation tasks. Always work on the **first unchecked item** and mark it complete when finished.

## Commit Guidelines
- Use **Conventional Commits** style messages (`feat:`, `fix:`, `docs:`, `chore:`...).
- Keep pull requests focused on a single task from `todo.md`.

## Development Workflow
1. Format code before committing:
   - TypeScript: `pnpm exec prettier --write .`
   - Rust: `cargo fmt`
2. Run linters and unit tests:
   - TypeScript: `pnpm exec eslint .` and `pnpm -r test`
   - Rust: `cargo clippy -- -D warnings` and `cargo test`
3. Build & run in development:
   - `pnpm dev` to start the React UI
   - `cargo tauri dev` to run the Tauri shell

Update this file with additional rules (tests, linting, style) as the project grows.
