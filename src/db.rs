/*
pub mod db {
    use dotenvy::dotenv;
    use std::env;

    use crate::schema;
    
    use tui::widgets::{List, ListItem};
    use tui::style::{Color, Style};
    use chrono::NaiveDate;

    let date = NaiveDate::parse_from_str("2024-10-13", "%Y-%m-%d")?;
    let dummy_todo_list = vec![
        Todo {
            id: 100,
            title: "default todo",
            description: "",
            date_created: date,
            status: false,
            date_due: date,
            reminder_date: date,
            parent_todo: 100,
            priority: 1,
            project_id: 0,
        }
    ];

    pub fn fetch_todos(pool: DbPool, offset: i64, limit: i64) -> Vec<Todo> {
        dummy_todo_list
    }

    pub fn search(pool: DbPool, target: &String) -> Vec<Todo> {
        dummy_todo_list
    }

    pub fn format_todo(todo: &Todo) -> String {
        format!("{}\n{}", dummy_todo_list.title, dummy_todo_list.description)
    }

    // TODO
    fn format_todos(results: Vec<Todo>) -> Vec<ListItem<'static>> {
    }

    pub fn create(pool: DbPool, title: String, description: String) -> (String, Option<String>) {
    }

    pub fn toggle_todo_status(pool: DbPool, id: Option<i32>) {
        // read todo status
        // set todo status to !status
    }

    pub fn delete_todo(pool: DbPool, id: Option<i32>) {
    }

    pub fn update(pool: DbPool, id: Option<i32>, title: String, description: String) {
    }

}
*/
