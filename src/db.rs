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
    pub async fn search(conn_pool: &MySqlPool, search_string: String) -> Result<Vec<Todo>, sqlx::Error> {
        // All database functions must return a Result<T>
        let search_string1 = format!("%{}%",search_string);
        let search_string2 = format!("%{}%",search_string);
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todo
WHERE todo.title LIKE ? OR todo.description LIKE ?;", search_string1, search_string2)
            .fetch_all(conn_pool)
        .await?;
        Ok(todos)
    }
    // Execure SELECT query on database to get todos
    pub async fn get_all_todos(conn_pool: &MySqlPool) -> Result<Vec<Todo>, sqlx::Error> {
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


    /*
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
    */

}
