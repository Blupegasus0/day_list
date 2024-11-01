use sqlx::mysql::MySqlPool;
use sqlx::FromRow;
use dotenv::dotenv;
use std::env;
use chrono::{NaiveDateTime, Local};

use DayList::db::db;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    /*
    // Set up the database connection
    let conn_pool = db::establish_connection().await?;

    // state test data
    let title = String::from("Real from rust again");
    let description = Some(String::from("used the real create_todo"));
    let date_due: Option<NaiveDateTime> = None;
    let reminder_date: Option<NaiveDateTime> = None;
    let parent_todo: Option<i32> = None;
    let priority = 4;
    let project_id: Option<i32> = None;

    match db::search(&conn_pool, String::from("rust")).await {
        Ok(todos) => {
            println!("Search Results: ");
            for todo in &todos {
                println!("{}", todo.title);
            }
            print!("\n");
            if todos.len() == 0 { println!("No data in table"); }
        }
        Err(err) => eprintln!("Error fetching todos: {:?}", err),
    }

    match db::fetch_todos(&conn_pool, 0, 1).await {
        Ok(todos) => {
            for todo in &todos {
                println!("{}", todo.title);
            }
            if todos.len() == 0 { println!("No data in table"); }
        }
        Err(err) => eprintln!("Error fetching todos: {:?}", err),
    }

*/
    Ok(())
}
