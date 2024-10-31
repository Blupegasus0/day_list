use sqlx::mysql::MySqlPool;
use dotenv::dotenv;
use std::env;
use chrono::{NaiveDateTime, Local};
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
async fn establish_connection() -> Result<MySqlPool, sqlx::Error> {
    // Load environment variables - database related
    dotenv().ok(); 
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Return connection pool for use throughout program
    MySqlPool::connect(&database_url).await
}

// Execure SELECT query on database to get todos
async fn get_all_todos(conn_pool: &MySqlPool) -> Result<Vec<Todo>, sqlx::Error> {
// All database functions must return a Result<T>
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todo")
        .fetch_all(conn_pool)
    .await?;
    Ok(todos)
}

// In this mf we using query! to perform functions that do not return results
// Not query_as! because that expects results
async fn test_create_todo(conn_pool: &MySqlPool) -> Result<(), sqlx::Error> {
    //let current_date = ifaoeihf;
    sqlx::query!("INSERT INTO todo (title, description, date_created, status, date_due, reminder_date, parent_todo, priority, project_id) 
        VALUES('test create', 'from rust',  CURRENT_DATE(),0, CURRENT_DATE(), CURRENT_DATE, 1, 1, NULL);")
        .execute(conn_pool)
        .await?;
    Ok(())
}

async fn create_todo(conn_pool: &MySqlPool, title: String, 
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

//async fn search_todos(conn_pool: &MySqlPool, search_string: &String) -> Result<Vec<Todo>, sqlx::Error> {
    //let todos_found = sqlx::query_as!(Todo, "SELECT * FROM todo WHERE title LIKE %()")
//}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Set up the database connection
    let conn_pool = establish_connection().await?;

    // state test data
    let title = String::from("Real from rust again");
    let description = Some(String::from("used the real create_todo"));
    let date_due = None;
    let reminder_date = None;
    let parent_todo = None;
    let priority = 4;
    let project_id = None;

    // Fetch and display all todos
    match create_todo(&conn_pool, title, description, date_due, reminder_date, 
        parent_todo, priority, project_id).await 
    {
        Ok(_) => println!("Todo created!"),
        Err(err) => eprintln!("Error fetching todos: {:?}", err),
    }

    match get_all_todos(&conn_pool).await {
        Ok(todos) => {
            for todo in &todos {
                println!("{}", todo.title);
            }
            if todos.len() == 0 { println!("No data in table"); }
        }
        Err(err) => eprintln!("Error fetching todos: {:?}", err),
    }


    Ok(())
}
