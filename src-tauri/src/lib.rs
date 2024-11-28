mod database;

use database::{Database, Todo};
use serde::Deserialize;
use std::sync::Mutex;
use tauri::State;

struct DbState(Mutex<Database>);

#[derive(Debug, Deserialize)]
pub struct NewTodo {
    title: String,
    description: Option<String>,
}

#[tauri::command]
fn add_todo(state: State<'_, DbState>, new_todo: NewTodo) -> Result<Todo, String> {
    let db = state.0.lock().map_err(|_| "Failed to lock database")?;
    db.add_todo(new_todo.title, new_todo.description)
        .map_err(|e| e.to_string())
}

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
        .invoke_handler(tauri::generate_handler![get_todos, add_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
