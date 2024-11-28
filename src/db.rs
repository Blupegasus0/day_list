pub mod db {
    use crate::schema::schema::Todo;
    
    use tui::widgets::{List, ListItem};
    use tui::style::{Color, Style};

    use sqlx::mysql::MySqlPool;
    use dotenv::dotenv;
    use std::env;
    use chrono::{NaiveDateTime, Local};

    use crate::utils;

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

    pub async fn toggle_todo_status(conn_pool: &MySqlPool, id: Option<i32>) -> Result<(), sqlx::Error> {
        // read todo status
        // set todo status to !status
        match id {
            Some(id) => {
                let record = sqlx::query!("SELECT status FROM todo WHERE todo_id = ?", id)
                    .fetch_optional(conn_pool)
                .await?;

                match record {
                    Some(value) => {
                        let mut status = value.status;
                        if status == 0 {status = 1} else {status = 0;}
                        sqlx::query!("UPDATE todo SET status = ? WHERE todo_id = ?", status, id)
                            .execute(conn_pool)
                        .await?;

                    }
                    None => {},
                }
            }
            None =>  utils::alert("No valid todo item selected."),// TODO error popup "no"
        };
        Ok(())
    }

    pub async fn delete_todo(conn_pool: &MySqlPool, id: Option<i32>) -> Result<(), sqlx::Error> {
        match id {
            Some(id) => {
                sqlx::query!("DELETE FROM todo WHERE todo_id = ?", id)
                    .execute(conn_pool)
                .await?;
            }
            None => print!("NO ID SELECTED")//utils::alert("No valid todo item selected."),// TODO error popup "no"
        };
        Ok(())
    }

    pub async fn update_todo(conn_pool: &MySqlPool, id: Option<i32>, title: String, description: String) -> Result<(), sqlx::Error>{
        match id {
            Some(id) => {
                sqlx::query!("UPDATE todo SET title = ?, description = ? WHERE todo_id = ?", title, description, id)
                    .execute(conn_pool)
                .await?;
            }
            None =>  utils::alert("No valid todo item selected."),// TODO error popup "no"
        };
        Ok(())
    }

}
