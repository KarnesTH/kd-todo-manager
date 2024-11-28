pub mod components;

pub use components::todo_card::TodoCard;

use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
