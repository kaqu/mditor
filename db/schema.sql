-- SQLite schema for AI-MD-Editor

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
