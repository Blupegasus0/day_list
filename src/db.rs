pub mod db {
    use crate::schema::schema::Todo;
    
    use tui::widgets::{List, ListItem};
    use tui::style::{Color, Style};

    use sqlx::mysql::MySqlPool;
    use dotenv::dotenv;
    use std::env;
    use chrono::{NaiveDateTime, Local};
    use sqlx::FromRow;

    // Connect database to app runtime
    pub async fn establish_connection() -> Result<MySqlPool, sqlx::Error> {
        // Load environment variables - database related
        dotenv().ok(); 
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // Return connection pool for use throughout program
        MySqlPool::connect(&database_url).await
    }

    // Execure SELECT query on database to get todos
    pub async fn search(conn_pool: &MySqlPool, search_string: &String) -> Result<Vec<Todo>, sqlx::Error> {
        // All database functions must return a Result<T>
        let search_string1 = format!("%{}%",search_string);
        let search_string2 = format!("%{}%",search_string);
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todo
WHERE todo.title LIKE ? OR todo.description LIKE ?;", search_string1, search_string2)
            .fetch_all(conn_pool)
        .await?;
        Ok(todos)
    }
    // Execute SELECT query on database to get todos
    pub async fn fetch_todos(conn_pool: &MySqlPool, offset: i32, limit: i32) -> Result<Vec<Todo>, sqlx::Error> {
        // All database functions must return a Result<T>
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todo")
            .fetch_all(conn_pool)
        .await?;
        Ok(todos)
    }

    pub async fn create_todo(conn_pool: &MySqlPool, title: String, 
        description: Option<String>, date_due: Option<NaiveDateTime>, 
        reminder_date: Option<NaiveDateTime>, parent_todo: Option<i32>, 
        priority: i32, project_id: Option<i32>
    ) -> Result<(), sqlx::Error> {
        let current_date = Some(Local::now().naive_local());
        let status = false;

        sqlx::query!("INSERT INTO todo (title, description, date_created, status, date_due, reminder_date, parent_todo, priority, project_id) 
VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?);",
            title, description, current_date, status, date_due, 
            reminder_date, parent_todo, 
            priority, project_id
        )
            .execute(conn_pool)
        .await?;
        Ok(())
    }

    pub fn format_todo(todo: &Todo) -> String {
        let mut todo_status = "[ ]";
        if todo.status == 1 {
            todo_status = "[]";
        }

        format!("\n   {} {}\n       {}\n",todo_status, todo.title, 
            match todo.description.clone() {
                Some(s) => s,
                None => "--".to_string(),
            } 
        )
    }

    /*
    pub fn format_todo(todo: &Todo) -> String {
        format!("{}\n{}", todo.title, todo.description)
    }

    // TODO
    fn format_todos(results: Vec<Todo>) -> Vec<ListItem<'static>> {
        vec![ListItem::new(results)]
    }
    */

    pub fn toggle_todo_status(pool: &MySqlPool, id: Option<i32>) {
    // read todo status
    // set todo status to !status
    ()
    }

    pub fn delete_todo(pool: &MySqlPool, id: Option<i32>) {
    ()
    }

    pub fn update(pool: &MySqlPool, id: Option<i32>, title: String, description: String) {
    ()
    }

}
