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
        for t in results {
            output_string.push_str(format!("\n{}\n", t.title).as_ref());
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
        output_string.push_str(format!("Found {} todos matching '{}'\n", results.len(), target).as_ref());
        for t in results {
            output_string.push_str(format!("\n{}\n", t.title).as_ref());
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

pub mod nav {
    pub enum Widget {
        Calendar,
        Main,
        Search,
        Upcoming,
    }

    impl Widget {
        pub fn up(&self) -> Widget {
            match &self {
                Widget::Calendar => Widget::Upcoming,
                Widget::Main => Widget::Search,
                Widget::Search => Widget::Search,
                Widget::Upcoming => Widget::Upcoming,
                _ => Widget::Main,
            }
        }
        pub fn down(&self) -> Widget {
            match &self {
                Widget::Calendar => Widget::Calendar,
                Widget::Main => Widget::Main,
                Widget::Search => Widget::Main,
                Widget::Upcoming => Widget::Calendar,
                _ => Widget::Main,
            }
        }
        pub fn left(&self) -> Widget {
            match &self {
                Widget::Calendar => Widget::Main,
                Widget::Main => Widget::Main, // Not implemented yet
                Widget::Search => Widget::Search, // Not implemented yet,
                Widget::Upcoming => Widget::Main,
                _ => Widget::Main,
            }
        }
        pub fn right(&self) -> Widget {
            match &self {
                Widget::Calendar => Widget::Calendar,
                Widget::Main => Widget::Calendar,
                Widget::Search => Widget::Upcoming,
                Widget::Upcoming => Widget::Upcoming,
                _ => Widget::Main,
            }
        }
    }
}
