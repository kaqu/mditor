use rusqlite::Connection;
use std::{fs, sync::Mutex};
use tauri::{AppHandle, Manager};

const SCHEMA: &str = include_str!("../../../db/schema.sql");

/// Initialize the SQLite database and run migrations.
pub fn init_db(app: &AppHandle) -> tauri::Result<()> {
    let resolver = app.path();
    let mut path = resolver.app_data_dir()?;
    fs::create_dir_all(&path)?;
    path.push("editor.sqlite");

    let conn = Connection::open(&path).map_err(|e| {
        let err: Box<dyn std::error::Error> = e.into();
        tauri::Error::Setup(err.into())
    })?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(|e| {
            let err: Box<dyn std::error::Error> = e.into();
            tauri::Error::Setup(err.into())
        })?;
    let version: i64 = conn
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|e| {
            let err: Box<dyn std::error::Error> = e.into();
            tauri::Error::Setup(err.into())
        })?;
    if version == 0 {
        conn.execute_batch(SCHEMA).map_err(|e| {
            let err: Box<dyn std::error::Error> = e.into();
            tauri::Error::Setup(err.into())
        })?;
        conn.pragma_update(None, "user_version", 1).map_err(|e| {
            let err: Box<dyn std::error::Error> = e.into();
            tauri::Error::Setup(err.into())
        })?;
    }

    app.manage(Mutex::new(conn));
    Ok(())
}
