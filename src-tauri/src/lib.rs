mod database;

use database::{Database, Todo};
use std::sync::Mutex;
use tauri::State;

struct DbState(Mutex<Database>);

#[tauri::command]
fn get_todos(state: State<'_, DbState>) -> Result<Vec<Todo>, String> {
    let db = state.0.lock().map_err(|_| "Failed to lock database")?;
    db.get_todos().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let database = Database::new().expect("Database initialization failed");

    tauri::Builder::default()
        .manage(DbState(Mutex::new(database)))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
