pub mod schema;
pub mod models;

pub mod db {
    use diesel::sqlite::SqliteConnection;
    use diesel::prelude::*;
    use dotenvy::dotenv;
    use std::env;

    use crate::schema;
    use crate::models::NewTodo;
    use crate::models::Todo;

    pub fn establish_connection() -> SqliteConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    pub fn read() -> String {
        // -- Read
        let connection = &mut establish_connection();
        let results = schema::todo::table
            .select(Todo::as_select())
            .load(connection)
            .expect("Error loading todos");

        let mut output_string = String::new();
        output_string.push_str(format!("Displaying {} todos", results.len()).as_ref());
        for t in results {
            output_string.push_str(format!("{}\n", t.title).as_ref());
            output_string.push_str(format!("-----------\n").as_ref());
            output_string.push_str(format!("{}\n", t.description.unwrap()).as_ref());
        }
        output_string
    }

    pub fn search(target: &String) -> String {
        // -- Read
        let pattern = format!("%{}%", target);

        let connection = &mut establish_connection();
        let results = schema::todo::table
            .filter(schema::todo::title.like(pattern))
            .load::<Todo>(connection)
            .expect("Error loading todos");

        let mut output_string = String::new();
        output_string.push_str(format!("Found {} todos matching {}", results.len(), target).as_ref());
        for t in results {
            output_string.push_str(format!("{}\n", t.title).as_ref());
            output_string.push_str(format!("{}\n", t.description.unwrap()).as_ref());
        }
        output_string
    }


    pub fn create() {
        // -- Create
        let mut title = String::from("New todo");
        let mut description = String::from("I am testing the db");
        let mut completed = false;

        let new_todo = NewTodo { 
            title: title, 
            description: Some(description), 
            completed: false, 
            parent_todo_id: None 
        };
        let connection = &mut establish_connection();

        diesel::insert_into(schema::todo::table)
            .values(&new_todo)
            .execute(connection)
            .expect("Error saving new todo");
    }

    pub fn update() {
        // -- Update
        let connection = &mut establish_connection();
        let id = 1;

        let todo = diesel::update(schema::todo::table.find(id))
            .set(schema::todo::completed.eq(true))
            .execute(connection)
            .unwrap();
        // println!("Completed '{}'", todo.title);


    }

    pub fn delete() {
        // -- Delete
        let connection = &mut establish_connection();
        let target = String::from("test");
        let pattern = format!("%{}%", target);

        let num_deleted = diesel::delete(schema::todo::table.filter(schema::todo::title.like(pattern)))
            .execute(connection)
            .expect("Error deleting posts");

        println!("Deleted {} todos", num_deleted);

    }

}
