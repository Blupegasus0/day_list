use sqlx::mysql::MySqlPool;
use dotenv::dotenv;
use std::env;
use chrono::NaiveDateTime;
use sqlx::FromRow;

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

async fn establish_connection() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok(); // Load .env variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Establish the database connection pool
    MySqlPool::connect(&database_url).await
}

async fn get_all_users(pool: &MySqlPool) -> Result<Vec<Todo>, sqlx::Error> {
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
            for user in users {
                println!("{:?}", user);
            }
        }
        Err(err) => eprintln!("Error fetching users: {:?}", err),
    }

    Ok(())
}
