# AI-MD-Editor

A local-first Markdown editor with an embedded AI agent. This repository contains the monorepo for both the Rust/Tauri core and the React UI.

More details can be found in `specification.md`.

## Branching Model

Active development happens on the `dev` branch. The `main` branch is protected
and only updated from `dev` when releasing. Feature branches should be created
from `dev` and pull requests should target `dev`.

## Developer Setup

Install the Tauri CLI so you can run and build the desktop shell:

```bash
cargo install tauri-cli --locked
```

Then install the Node dependencies:

```bash
pnpm install
```

Use `pnpm dev` to start the React UI and `cargo tauri dev` to launch the Tauri
application during development.
