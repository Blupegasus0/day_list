use sqlx::mysql::MySqlPool;
use dotenv::dotenv;
use std::env;
use chrono::{NaiveDateTime, Local};
use sqlx::FromRow;

use DayList::db::db;

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

//async fn search_todos(conn_pool: &MySqlPool, search_string: &String) -> Result<Vec<Todo>, sqlx::Error> {
    //let todos_found = sqlx::query_as!(Todo, "SELECT * FROM todo WHERE title LIKE %()")
//}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
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

    // Fetch and display all todos
    //match create_todo(&conn_pool, title, description, date_due, reminder_date, 
    //    parent_todo, priority, project_id).await 
    //{
    //    Ok(_) => println!("Todo created!"),
    //    Err(err) => eprintln!("Error fetching todos: {:?}", err),
    //}

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

    match db::get_all_todos(&conn_pool).await {
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
