use dirs;
use rusqlite::{Connection, Result};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

pub struct Database {
    pub connection: Connection,
}

#[derive(Debug, Serialize)]
pub struct Todo {
    id: u64,
    title: String,
    description: Option<String>,
    created_at: String,
    updated_at: String,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::ensure_database_dir()?;
        let connection = Connection::open(db_path)?;
        Self::init_database(&connection)?;
        Ok(Self { connection })
    }

    fn ensure_database_dir() -> Result<PathBuf> {
        if let Some(home_dir) = dirs::home_dir() {
            let db_dir = home_dir.join("kd-todo-manager/data");
            fs::create_dir_all(&db_dir).unwrap();
            Ok(db_dir.join("todo.db"))
        } else {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1),
                Some("Could not determine home directory".into()),
            ))
        }
    }

    fn init_database(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                todo_id INTEGER,
                title TEXT NOT NULL,
                description TEXT,
                completed BOOLEAN DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE
            )",
            [],
        )?;

        Ok(())
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, title, description, created_at, updated_at FROM todos")?;

        let todos = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get::<_, Option<String>>(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        todos.collect()
    }

    pub fn add_todo(&self, title: String, description: Option<String>) -> Result<Todo> {
        let mut stmt = self.connection.prepare("INSERT INTO todos (title, description) VALUES (?1, ?2) RETURNING id, title, description, created_at, updated_at")?;

        stmt.query_row(&[&title, &description as &dyn rusqlite::ToSql], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get::<_, Option<String>>(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
    }

    pub fn get_todo(&self, id: u64) -> Result<Todo> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, description, created_at, updated_at FROM todos WHERE id = ?1",
        )?;

        stmt.query_row(&[&id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
    }

    pub fn update_todo(&self, title: String, description: Option<String>) -> Result<Todo> {
        let mut stmt = self.connection.prepare("UPDATE todos SET title = ?1, description = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3 RETURNING id, title, description, created_at, updated_at")?;

        stmt.query_row(&[&title, &description as &dyn rusqlite::ToSql], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get::<_, Option<String>>(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
    }
}
