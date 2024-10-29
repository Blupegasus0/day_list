pub mod db {
    use dotenvy::dotenv;
    use std::env;

    use crate::schema;
    
    pub fn establish_connection_pool() -> DbPool {
    }

    
    use tui::widgets::{List, ListItem};
    use tui::style::{Color, Style};
    pub fn fetch_todos(pool: DbPool, offset: i64, limit: i64) -> Vec<Todo> {
    }

    pub fn search(pool: DbPool, target: &String) -> Vec<Todo> {
    }

    pub fn format_todo(todo: &Todo) -> String {
    }

    // TODO
    fn format_todos(results: Vec<Todo>) -> Vec<ListItem<'static>> {
    }

    pub fn create(pool: DbPool, title: String, description: String) -> (String, Option<String>) {
    }

    pub fn complete_todo(pool: DbPool, id: Option<i32>) {
    }

    pub fn delete_todo(pool: DbPool, id: Option<i32>) {
    }

    pub fn update(pool: DbPool, id: Option<i32>, title: String, description: String) {
    }

}
