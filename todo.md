# ✅ AI-MD-Editor — Detailed Implementation To-Do List

_(check items as you complete them)_

---

## 0 Repository & Meta

- [x] **Create GitHub repository** `ai-md-editor`
- [x] Add `LICENSE` (MIT)
- [x] Commit `SPEC.md` and `README.md` stubs
- [x] `.gitignore` for `target/`, `node_modules/`, `dist/`, `*.sqlite`, `*.env*`
- [x] Default branch `dev`; protect `main`
- [x] Conventional Commits tooling
  - [x] `commitlint`, `husky` pre-commit/push hooks
  - [x] `lint-staged` for format check

---

## 1 Monorepo Scaffolding

- [x] `pnpm init` at root
- [x] `pnpm-workspace.yaml` with:
  - `apps/desktop`
  - `packages/ui`
  - `packages/agent`
  - `packages/shared`
- [x] Root devDeps: `typescript`, `eslint`, `prettier`, `vitest`, `tsx`
- [x] Root `tsconfig.json` (`composite`, `paths` for shared types)

---

## 2 Continuous Integration

- [x] `.github/workflows/ci.yml`
  - [x] Cache pnpm & Rust toolchain
  - [x] Lint (`pnpm exec eslint .`)
  - [x] Unit tests (`pnpm -r test` & `cargo test`)
  - [x] Build desktop app (`cargo tauri build --debug`) on all OSes
- [x] `.github/workflows/release.yml`
  - [x] Trigger on tags `v*`
  - [x] Build signed installers
  - [x] Upload to GitHub Release

---

## 3 Tauri / Rust Core

- [x] `cargo install tauri-cli`
- [x] `tauri init` in `apps/desktop`
- [x] Update `tauri.conf.json`
  - [x] `distDir` → `../../packages/ui/dist`
  - [x] Application name, identifier
- [x] Add Rust deps:
  - `rusqlite`, `serde`, `serde_json`, `uuid`, `zip-rs`, `walkdir`, `diffy`
- [x] Set up Rust workspace (`Cargo.toml` at root)
- [x] Configure Clippy & rustfmt in CI

---

## 4 SQLite Layer

- [x] `db/schema.sql` — create `nodes`, `assets`, `op_log`
- [x] `db.rs`
  - [x] Open DB file in platform-specific app data dir
  - [x] Run migrations on startup
- [ ] CRUD helpers (`create_node`, `update_content`, `move_node`, `delete_node`)
- [ ] Op-log insert on each mutating call
- [ ] Unit tests: CRUD paths, cascade delete, retention trim

---

## 5 Tool Command API (Rust)

- [ ] `#[tauri::command] read_file(id)`
- [ ] `write_file(id, new_content)` → return unified diff
- [ ] `create_file(parent_id, name, content?)`
- [ ] `delete_node(id)`
- [ ] `move_node(id, new_parent)`
- [ ] `list_tree(parent_id?)` (shallow)
- [ ] `undo()` / `redo()`
- [ ] `export_zip(dest_path)` / `export_dir(dest)`
- [ ] `import_path(src)` merges & returns report
- [ ] `get_asset(uuid)` streaming handler

---

## 6 React UI Bootstrap (`packages/ui`)

- [ ] `pnpm create vite@latest ui -- --template react-ts`
- [ ] Install libs:
  - `@codemirror/basic-setup`, `@codemirror/lang-markdown`, `react-codemirror`
  - `react-sortable-tree`, `react-split-pane`
  - `markdown-it`, `highlight.js`, `mathjax-full`
  - `@tauri-apps/api`
- [ ] Global Tailwind CSS (optional)
- [ ] **Layout**
  - [ ] Left sidebar tree view
  - [ ] Split main pane: CodeMirror + Preview
  - [ ] Collapsible right sidebar container
  - [ ] Top bar with export / settings icons
- [ ] **State** (TanStack Query or Zustand)
  - [ ] Nodes cache via IPC
  - [ ] Current file selection
  - [ ] Dirty flag & autosave

---

## 7 IPC Bridges (`packages/shared`)

- [ ] Create `src/ipc.ts`
  - [ ] Typed wrappers for each Tauri command
- [ ] Re-export shared models (`NodeMeta`, `Diff`)

---

## 8 File-Tree UI

- [ ] Drag-drop reorder → `move_node`
- [ ] Context menu: new file/folder, rename, delete
- [ ] Search filter input
- [ ] Sync selection with editor tab

---

## 9 Editor & Preview

- [ ] CodeMirror 6 instance with Markdown language
- [ ] `diff-match-patch` decorations on unsaved changes
- [ ] Live preview panel using `markdown-it`
  - [ ] MathJax hook for `$$` blocks
  - [ ] Asset resolver: transform `assets://uuid` → `tauri://asset?uuid=`

---

## 10 Undo / Redo Integration

- [ ] Bind `Cmd/Ctrl+Z` → IPC `undo`
- [ ] Bind `Shift+Cmd/Ctrl+Z` → `redo`
- [ ] Update editor & tree after operation

---

## 11 Binary Asset Flow

- [ ] Detect `paste` / `drop` events with `image/*`
- [ ] IPC `save_asset(bytes, mime)` → returns `uuid`
- [ ] Insert Markdown: `![alt](assets://{uuid})` at cursor
- [ ] Preview fetches via Tauri asset endpoint

---

## 12 Export / Import UI

- [ ] “Export ▾” button
  - [ ] To Directory (OS picker)
  - [ ] To ZIP
- [ ] “Import” button (folder/zip)
  - [ ] Show merge summary dialog

---

## 13 Agent Subsystem (`packages/agent`)

- [ ] Install deps: `langchain`, `xstate`, `diff-match-patch`, `openai`
- [ ] `tools.ts` mapping → IPC calls
- [ ] `agentMachine.ts`
  - [ ] States: `planning`, `acting`, `waiting_user`, `done`, `aborted`
  - [ ] Limit checks (tool count, timeout)
- [ ] Web Worker wrapper (`comlink`)
- [ ] Progress store (`useAgentStore`)

### UI Integration

- [ ] Slash-command modal (`Cmd/Ctrl+K`)
- [ ] Right pane sections:
  - [ ] Prompt history (current loop)
  - [ ] Instructions editor (Markdown w/ assets)
  - [ ] Live task list / plan
- [ ] Ask-user modal (blocks editor)

---

## 14 Safety & Error UI

- [ ] Global error boundary in React
- [ ] Display agent abort reason (“limit exceeded”)
- [ ] Toast notifications for export/import success/fail

---

## 15 Keyboard Shortcuts

- [ ] Tauri global shortcuts:
  - [ ] `Cmd/Ctrl+N` new file
  - [ ] `Cmd/Ctrl+Shift+N` new folder
  - [ ] `Cmd/Ctrl+S` save
  - [ ] `Cmd/Ctrl+K` agent prompt
- [ ] In-editor keymap selection (default / Vim / Emacs)

---

## 16 Settings Panel

- [ ] React modal under top bar
- [ ] Fields:
  - [ ] OpenAI API key (stored via Tauri `@tauri-apps/plugin-store`)
  - [ ] Max tool calls / timeout sliders (advanced)
- [ ] “Clear DB” / “Open data folder” utilities

---

## 17 Packaging

- [ ] `tauri.conf.json` platform-specific identifiers
- [ ] Provide `.env` template for CI signing creds
- [ ] Verify `cargo tauri build --release` on macOS, Windows, Linux
- [ ] Generate SHA256 checksums

---

## 18 Tests

- **Rust**
  - [ ] Unit: each DB function, export/import parity
- **TypeScript**
  - [ ] Vitest for agent planner, tool wrappers
- **E2E** (Playwright)
  - [ ] Launch app → create file → run agent prompt → verify files created
  - [ ] Export dir → compare file contents against DB
- [ ] Add test steps to CI matrix

---

## 19 Documentation

- [ ] Expand `README.md`
  - [ ] Install prereqs (Rust, pnpm)
  - [ ] Dev workflow commands
  - [ ] Building installers
  - [ ] Usage quick-start (screencap GIF)
- [ ] `CONTRIBUTING.md` (branch naming, linting, release)
- [ ] Changelog template (`CHANGELOG.md`)

---

## 20 Demo Assets

- [ ] Sample workspace SQL dump in `/samples`
- [ ] Record 60-second demo GIF and place in `/media`

---

## 21 Release Process

- [ ] Bump versions (`pnpm version`, `cargo set-version`)
- [ ] Generate changelog (`pnpm changelog`)
- [ ] `git tag v0.1.0` → push tags
- [ ] GitHub Action builds & uploads installers
- [ ] Publish release notes with checksums & demo GIF

---
