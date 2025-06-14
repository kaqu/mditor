use std::{path::PathBuf, sync::Mutex};

#[cfg(feature = "tauri")]
use std::fs;

use rusqlite::{params, Connection, OptionalExtension};

#[cfg(feature = "tauri")]
use tauri::{AppHandle, Manager};

const SCHEMA: &str = include_str!("../../../db/schema.sql");

#[derive(Debug)]
pub struct Db {
    conn: Mutex<Connection>,
}

impl Db {
    /// Open a SQLite connection and run migrations.
    pub fn open(path: PathBuf) -> rusqlite::Result<Self> {
        let create = !path.exists();
        let conn = Connection::open(&path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        let version: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
        if version == 0 && create {
            conn.execute_batch(SCHEMA)?;
            conn.pragma_update(None, "user_version", 1)?;
        }
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Initialize the DB in the platform data directory and register it with Tauri.
    #[cfg(feature = "tauri")]
    pub fn init(app: &AppHandle) -> tauri::Result<()> {
        let resolver = app.path();
        let mut path = resolver.app_data_dir()?;
        fs::create_dir_all(&path)?;
        path.push("mditor.sqlite");
        let db = Self::open(path).map_err(|e| {
            let err: Box<dyn std::error::Error> = e.into();
            tauri::Error::Setup(err.into())
        })?;
        app.manage(db);
        Ok(())
    }

    pub fn read_file(&self, id: i64) -> rusqlite::Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT content FROM nodes WHERE id=?1 AND kind='file'",
            params![id],
            |row| row.get(0),
        )
        .optional()
    }

    pub fn write_file(&self, id: i64, content: &str) -> rusqlite::Result<()> {
        let now = chrono::Utc::now().timestamp_millis();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE nodes SET content=?1, updated_at=?2 WHERE id=?3 AND kind='file'",
            params![content, now, id],
        )?;
        Ok(())
    }

    pub fn create_file(
        &self,
        parent_id: Option<i64>,
        name: &str,
        content: Option<&str>,
    ) -> rusqlite::Result<i64> {
        let now = chrono::Utc::now().timestamp_millis();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO nodes(parent_id, name, kind, mime, content, created_at, updated_at) VALUES (?1, ?2, 'file', 'text/markdown', ?3, ?4, ?4)",
            params![parent_id, name, content, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn delete_node(&self, id: i64) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM nodes WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn move_node(&self, id: i64, new_parent: Option<i64>) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE nodes SET parent_id=?1 WHERE id=?2",
            params![new_parent, id],
        )?;
        Ok(())
    }

    pub fn list_tree(&self, parent_id: Option<i64>) -> rusqlite::Result<Vec<NodeMeta>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, kind FROM nodes WHERE parent_id IS ?1")?;
        let rows = stmt
            .query_map(params![parent_id], |row| {
                Ok(NodeMeta {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    kind: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }
}

#[derive(serde::Serialize)]
pub struct NodeMeta {
    pub id: i64,
    pub name: String,
    pub kind: String,
}
