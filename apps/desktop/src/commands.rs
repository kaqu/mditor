#![cfg(feature = "tauri")]

use tauri::State;

use crate::db::{Db, NodeMeta};

#[tauri::command]
pub async fn read_file(id: i64, db: State<'_, Db>) -> Result<Option<String>, String> {
    db.read_file(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_file(id: i64, new_content: String, db: State<'_, Db>) -> Result<(), String> {
    db.write_file(id, &new_content).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_file(
    parent_id: Option<i64>,
    name: String,
    content: Option<String>,
    db: State<'_, Db>,
) -> Result<i64, String> {
    db.create_file(parent_id, &name, content.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_node(id: i64, db: State<'_, Db>) -> Result<(), String> {
    db.delete_node(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn move_node(id: i64, new_parent: Option<i64>, db: State<'_, Db>) -> Result<(), String> {
    db.move_node(id, new_parent).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_tree(parent_id: Option<i64>, db: State<'_, Db>) -> Result<Vec<NodeMeta>, String> {
    db.list_tree(parent_id).map_err(|e| e.to_string())
}
