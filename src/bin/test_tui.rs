use sqlx::mysql::MySqlPool;
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
async fn establish_connection() -> Result<MySqlPool, sqlx::Error> {
    // Load environment variables - database related
    dotenv().ok(); 
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Return connection pool for use throughout program
    MySqlPool::connect(&database_url).await
}

// Execure SELECT query on database to get users
async fn get_all_users(pool: &MySqlPool) -> Result<Vec<Todo>, sqlx::Error> {
// All database functions must return a Result<T>
    let users = sqlx::query_as!(Todo, "SELECT * FROM todo")
        .fetch_all(pool)
    .await?;
    Ok(users)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Set up the database connection
    let pool = establish_connection().await?;

    // Fetch and display all users
    match get_all_users(&pool).await {
        Ok(users) => {
            for user in &users {
                println!("{:?}", user);
            }
            if users.len() == 0 { println!("No data in table"); }
        }
        Err(err) => eprintln!("Error fetching users: {:?}", err),
    }

    match get_all_users(&pool).await {
        Ok(users) => {
            for user in &users {
                println!("{:?}", user);
            }
            if users.len() == 0 { println!("No data in table"); }
        }
        Err(err) => eprintln!("Error fetching users: {:?}", err),
    }

    match get_all_users(&pool).await {
        Ok(users) => {
            for user in &users {
                println!("{:?}", user);
            }
            if users.len() == 0 { println!("No data in table"); }
        }
        Err(err) => eprintln!("Error fetching users: {:?}", err),
    }

    Ok(())
}
