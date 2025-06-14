#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use desktop::{commands, db};

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            db::Db::init(&app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::read_file,
            commands::write_file,
            commands::create_file,
            commands::delete_node,
            commands::move_node,
            commands::list_tree,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
