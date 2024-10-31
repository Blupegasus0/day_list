use sqlx::mysql::MySqlconn_pool;
use dotenv::dotenv;
use std::env;
use chrono::NaiveDateTime;
use sqlx::FromRow;

// Define rust object
#[derive(Debug,sqlx::FromRow)]
pub struct Todo {
    pub todo_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date_created: Option<NaiveDateTime>,
    pub status: i8,
    pub date_due: Option<NaiveDateTime>,
    pub reminder_date: Option<NaiveDateTime>,
    pub parent_todo: Option<i32>,
    pub priority: i32,
    pub project_id: Option<i32>,
}

// Connect database to app runtime
async fn establish_connection() -> Result<MySqlconn_pool, sqlx::Error> {
    // Load environment variables - database related
    dotenv().ok(); 
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Return connection conn_pool for use throughout program
    MySqlconn_pool::connect(&database_url).await
}

// Execure SELECT query on database to get todos
async fn get_all_todos(conn_pool: &MySqlconn_pool) -> Result<Vec<Todo>, sqlx::Error> {
// All database functions must return a Result<T>
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todo")
        .fetch_all(conn_pool)
    .await?;
    Ok(todos)
}

async fn search_todos(conn_pool: &MySqlconn_pool, search_string: &String) -> Result<Vec<Todo>, sqlx::Error> {
    //let todos_found = sqlx::query_as!(Todo, "SELECT * FROM todo WHERE title LIKE %()")
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Set up the database connection
    let conn_pool = establish_connection().await?;

    // Fetch and display all todos
    match search_todos(&conn_pool).await {
        Ok(todos) => {
            for todo in &todos {
                println!("{}", todo.title);
            }
            if todos.len() == 0 { println!("No data in table"); }
        }
        Err(err) => eprintln!("Error fetching todos: {:?}", err),
    }

    match get_all_todos(&conn_pool).await {
        Ok(todos) => {
            for todo in &todos {
                println!("{}", todo.todo_id);
            }
            if todos.len() == 0 { println!("No data in table"); }
        }
        Err(err) => eprintln!("Error fetching todos: {:?}", err),
    }


    Ok(())
}
