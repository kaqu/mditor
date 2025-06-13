# AI-MD-Editor — Unified Software Specification  
*Version 0.9 (Prototype-First Edition, 13 Jun 2025)*  

---

## 1 . Mission Statement
Build a cross-platform **Markdown editor with an embedded autonomous AI agent**.  

| Tier | Target | Description |
|------|--------|-------------|
|**Prototype-MVP**|Single-user desktop app | Offline-first, local SQLite, no auth/network. |

This document fully specifies the **Prototype-MVP**.

---

## 2 . Functional Scope (Prototype-MVP)

| Area | Must-have features |
|------|--------------------|
|Editing|Split code/preview ● Vim/Emacs keymaps optional ● Undo/redo (last 1000 ops)|
|Virtual file-tree|Add / rename / move / delete nodes ● Drag-drop reorder ● Search|
|AI Agent|Slash-command prompt (`⌘/Ctrl-K`) ● Autonomous plan–act loop ● `ask_user()` blocking calls|
|Instructions pane|Rich-text (Markdown) + drag-in media (stored as BLOB)|
|Export/Import|• One file → clipboard • Full tree → real directory or ZIP • Reverse import merges by `(path, hash)`|
|Binary assets|Images/audio stored as BLOB in SQLite; Markdown embeds use pseudo-URI `assets://{uuid}`|
|Safety limits|Max **200 tool calls** or **60 min** wall-clock per prompt|

---

## 3 . Non-Functional Requirements

* **Platforms:** Windows 10+, macOS 13+, Ubuntu 22+ (AppImage).  
* **Launch time:** ≤ 1.5 s. *Agent turn* ≤ 5 s for ≤ 100 tokens.  
* **Offline-first:** All CRUD works without internet; LLM calls fail gracefully.  
* **Accessibility:** WCAG 2.2 AA (keyboard nav, aria-labels).  
* **Security:** Agent whitelist, CSP inside WebView, signed updates.

---

## 4 . High-Level Architecture

```

Tauri Shell  ──┬──  Rust Core (SQLite, Exporter, Tool APIs)
│
└──  WebView (React 19 SPA)
├─ CodeMirror 6 editor
├─ markdown-it preview
└─ Web Worker → LangChainJS agent

````

*IPC* uses Tauri’s `invoke()` (synchronous) for tool calls and exports.

---

## 5 . Technology Stack

| Layer | Choice | Notes |
|-------|--------|-------|
|Shell | **Tauri 2.0** (Rust 1.78) | 6–12 MB binary |
|UI | React 19 + Vite 6 + TanStack Query | HMR for dev |
|Editor | CodeMirror 6.8 (`lang-markdown`) | Diff highlight plugin |
|Preview | markdown-it + highlight.js + MathJax | |
|DB | SQLite 3.46 via `rusqlite` (Rust) & `better-sqlite3` (UI tests) | |
|Agent | LangChainJS 0.2 + XState FSM + OpenAI o3 | |
|Diff | google diff-match-patch | line mode |
|Packaging | `cargo tauri build` → dmg / msi / AppImage | Codesign & notarize |

---

## 6 . Data Model (SQLite)

```sql
-- 1. Virtual nodes -----------------------------------------
CREATE TABLE nodes (
  id         INTEGER PRIMARY KEY,
  parent_id  INTEGER REFERENCES nodes(id) ON DELETE CASCADE,
  name       TEXT NOT NULL,
  kind       TEXT CHECK(kind IN ('file','folder')) NOT NULL,
  mime       TEXT,
  content    BLOB,               -- null for folders
  created_at INTEGER,            -- Unix ms
  updated_at INTEGER
);
CREATE UNIQUE INDEX idx_siblings ON nodes(parent_id, name);

-- 2. Embedded binary assets -------------------------------
-- Assets are referenced from Markdown via assets://<uuid>
CREATE TABLE assets (
  uuid       TEXT PRIMARY KEY,
  mime       TEXT,
  bytes      BLOB,
  created_at INTEGER
);

-- 3. Op-log for undo --------------------------------------
CREATE TABLE op_log (
  id       INTEGER PRIMARY KEY,
  ts       INTEGER,
  op       TEXT,          -- 'create','delete','update','move','asset'
  node_id  INTEGER,
  payload  BLOB           -- JSON delta
);
CREATE INDEX idx_op_ts ON op_log(ts DESC);
````

Retention policy: keep most-recent 1000 ops; vacuum older.

---

## 7 . Agent Tool Interface (Prototype Surface)

```ts
interface Tools {
  read_file(id: number): string;
  write_file(id: number, newContent: string): Diff;   // returns unified diff
  create_file(parentId: number, name: string, content?: string): number;
  delete_node(id: number): 'OK';
  move_node(id: number, newParent: number): 'OK';
  list_tree(parentId?: number): NodeMeta[];           // shallow list
  ask_user(prompt: string): Promise<string>;          // modal prompt
}
```

*All calls are atomic transactions in Rust; diffs streamed to op-log.*

---

## 8 . Agent Algorithm

```
init → plan(tasks[]) → loop {
    if exhausted limits → abort w/ summary
    step = planner.next()
    if step.needs_user:
        answer = ask_user()
        planner.record(answer)
    else:
        result = tools[step.tool](...args)
        memory.add(result)
    planner.update()
}
emit finish(summary)
```

*Memory trim after each tool call keeps scratchpad ≤ 2 KB.*

---

## 9 . UI Layout & Interactions

```
┌ Tree Sidebar ───────┐┌───────────── Editor Tab ──────────────┐
│  root/              ││ Code (CM6) │  Live Preview           │
│  ├ readme.md        ││ ───────────┼──────────────────────── │
│  └ docs/            ││ …                                     │
└─────────────────────┘└───────────────────────────────────────┘
⌘/Ctrl-K opens **Prompt Bar**  |  Right collapsible:  
                                |  • Instructions pane  
                                |  • Agent progress (plan + todo)  
```

*No multi-tab project selector in MVP; one window = one workspace.*

---

## 10 . Build, Run & Ship

```bash
# Dev mode
pnpm install
pnpm dev                # Vite + React
cargo tauri dev         # Rust hot-reload shell

# Release
pnpm build              # bundle UI → /dist
cargo tauri build       # dmg, msi, AppImage in /target/release/bundle
```

User settings (including `OPENAI_API_KEY`) stored via Tauri `store` plugin (encrypted in OS keychain).

---

## 11 . Known Gaps (reserved for SaaS)

* Roles & ACL
* Remote object storage (S3)
* Token/billing counters
* OpenTelemetry tracing
* Dependency vulnerability scanning

---

## 12 . Acceptance Tests (Prototype)

1. **CRUD**: create *README.md*, edit, move into folder, delete folder → expect cascading delete.
2. **Agent**: prompt: “Create README, LICENSE, TODO with boilerplate.” Agent finishes autonomously; files appear with non-empty content.
3. **Undo**: Perform 12 edits; `⌘-Z` rolls back each in order.
4. **Export**: Export tree to temp dir → open files identical to DB blobs.
5. **Import**: Modify an exported file, re-import → DB row updates, `updated_at` changes.
6. **Binary assets**: Paste PNG → appears in preview; export places file in `assets/` and rewrites Markdown link.

Passing all six equals MVP completion.

---
