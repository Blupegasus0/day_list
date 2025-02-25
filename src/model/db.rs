use sqlx::sqlite::SqlitePool;
use dotenv::dotenv;
use std::env;
use chrono::{NaiveDateTime, Local};

use crate::utils;
use crate::model::schema::Todo;

pub struct Db {
    pub conn_pool: SqlitePool,
}

impl Db {
    // Connect database to app runtime
    pub async fn new() -> Db {
        // Load environment variables - database related
        dotenv().ok(); 
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = SqlitePool::connect(&database_url).await.expect("Failed to connect to database"); 
        // ERROR handler more gracefully

        // Return connection pool for use throughout program
        Db {
            conn_pool: conn
        }
    }

    // Execute SELECT query on database to get todos
    pub async fn search(&self, search_string: &String) -> Result<Vec<Todo>, sqlx::Error> {
        // All database functions must return a Result<T>
        let search_string1 = format!("%{}%",search_string);
        let search_string2 = format!("%{}%",search_string);

        let rows = sqlx::query!(
            "SELECT * FROM todo WHERE todo.title LIKE ? OR todo.description LIKE ?;", 
            search_string1, search_string2)
            .fetch_all(&self.conn_pool)
        .await?;

        // Map the results to Todo
        let todos: Vec<Todo> = rows.into_iter().map(|row| {
            Todo {
                todo_id: row.todo_id,
                title: row.title,
                description: row.description,
                date_created: Db::string_to_naive_datetime(row.date_created),
                status: row.status as i8, // Assuming status is stored as BOOLEAN (0 or 1)
                date_due: Db::string_to_naive_datetime(row.date_due),
                reminder_date: Db::string_to_naive_datetime(row.reminder_date),
                parent_todo: row.parent_todo,
                priority: row.priority,
                project_id: row.project_id,
            }
        }).collect();

        Ok(todos)
    }
    // Execute SELECT query on database to get todos
    pub async fn fetch_todos(&self, offset: u32, limit: u32) -> Result<Vec<Todo>, sqlx::Error> {
        // All database functions must return a Result<T>
        let rows = sqlx::query!("SELECT * FROM todo")
            .fetch_all(&self.conn_pool)
        .await?;

        // Map the results to Todo
        let todos: Vec<Todo> = rows.into_iter().map(|row| {
            Todo {
                todo_id: row.todo_id,
                title: row.title,
                description: row.description,
                date_created: Db::string_to_naive_datetime(row.date_created),
                status: row.status as i8, // Assuming status is stored as BOOLEAN (0 or 1)
                date_due: Db::string_to_naive_datetime(row.date_due),
                reminder_date: Db::string_to_naive_datetime(row.reminder_date),
                parent_todo: row.parent_todo,
                priority: row.priority,
                project_id: row.project_id,
            }
        }).collect();

        Ok(todos)
    }

    pub async fn fetch_upcoming_todos(&self, offset: u32, limit: u32) -> Result<Vec<Todo>, sqlx::Error> {
        // All database functions must return a Result<T>
        let rows = sqlx::query!("SELECT * FROM todo WHERE date_due > CURRENT_DATE")
            .fetch_all(&self.conn_pool)
        .await?;

        // Map the results to Todo
        let todos: Vec<Todo> = rows.into_iter().map(|row| {
            Todo {
                todo_id: row.todo_id,
                title: row.title,
                description: row.description,
                date_created: Db::string_to_naive_datetime(row.date_created),
                status: row.status as i8, // Assuming status is stored as BOOLEAN (0 or 1)
                date_due: Db::string_to_naive_datetime(row.date_due),
                reminder_date: Db::string_to_naive_datetime(row.reminder_date),
                parent_todo: row.parent_todo,
                priority: row.priority,
                project_id: row.project_id,
            }
        }).collect();

        Ok(todos)
    }

    pub async fn create_todo(&self, todo: &Todo) -> Result<(), sqlx::Error> {
        let current_date = Some(Local::now().naive_local());

        sqlx::query!("INSERT INTO todo (title, description, date_created, status, date_due, reminder_date, parent_todo, priority, project_id) 
VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?);",
            todo.title, todo.description, current_date, todo.status, todo.date_due, 
            todo.reminder_date, todo.parent_todo, 
            todo.priority, todo.project_id
        )
            .execute(&self.conn_pool)
        .await?;
        Ok(())
    }

    pub async fn toggle_todo_status(&self, id: Option<i64>) -> Result<(), sqlx::Error> {
        // read todo status
        // set todo status to !status
        match id {
            Some(id) => {
                let record = sqlx::query!("SELECT status FROM todo WHERE todo_id = ?", id)
                    .fetch_optional(&self.conn_pool)
                .await?;

                if let Some(value) = record {
                    let mut status = value.status;
                    status = !status;
                    sqlx::query!("UPDATE todo SET status = ? WHERE todo_id = ?", status, id)
                        .execute(&self.conn_pool)
                    .await?;
                }
            }
            None =>  utils::alert("No valid todo item selected."),// TODO error popup "no"
        };
        Ok(())
    }

    pub async fn delete_todo(&self, id: Option<i64>) -> Result<(), sqlx::Error> {
        match id {
            Some(id) => {
                sqlx::query!("DELETE FROM todo WHERE todo_id = ?", id)
                    .execute(&self.conn_pool)
                .await?;
            }
            None => utils::alert("No valid todo item selected."),// TODO error popup "no"
        };
        Ok(())
    }

    pub async fn update_todo(&self, todo: &Todo) -> Result<(), sqlx::Error>{
        sqlx::query!("UPDATE todo SET title = ?, description = ? WHERE todo_id = ?", todo.title, todo.description, todo.todo_id)
            .execute(&self.conn_pool)
        .await?;
        Ok(())
    }


    fn string_to_naive_datetime(date_str: Option<String>) -> Option<NaiveDateTime> {
        date_str.and_then(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok())
    }

}
