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

    pub fn read(connection: &mut SqliteConnection) -> String {
        // -- Read
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

    use tui::widgets::{List, ListItem};
    pub fn fetch_todos(conn: &mut SqliteConnection, offset: i64, limit: i64) -> Vec<ListItem> {
        let results = schema::todo::table
            .limit(limit)
            .offset(offset)
            .load::<Todo>(conn)
            .expect("Error loading items");

        let mut list: Vec<ListItem> = Vec::new();

        for todo in results {
            let todo_item = format!("{}\n{}\n", todo.title, 
                match todo.description {
                    Some(s) => s,
                    None => "--".to_string(),
                } 
            );
            list.push(ListItem::new(todo_item))
        }
        list
    }

    pub fn search(connection: &mut SqliteConnection, target: &String) -> String {
        // -- Read
        let pattern = format!("%{}%", target);

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


    pub fn create(connection: &mut SqliteConnection) {
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

        diesel::insert_into(schema::todo::table)
            .values(&new_todo)
            .execute(connection)
            .expect("Error saving new todo");
    }

    pub fn update(connection: &mut SqliteConnection) {
        // -- Update
        let id = 1;

        let todo = diesel::update(schema::todo::table.find(id))
            .set(schema::todo::completed.eq(true))
            .execute(connection)
            .unwrap();
        // println!("Completed '{}'", todo.title);


    }

    pub fn delete(connection: &mut SqliteConnection) {
        // -- Delete
        let target = String::from("test");
        let pattern = format!("%{}%", target);

        let num_deleted = diesel::delete(schema::todo::table.filter(schema::todo::title.like(pattern)))
            .execute(connection)
            .expect("Error deleting posts");

        println!("Deleted {} todos", num_deleted);

    }

}

pub mod nav {
    pub enum Content {
        Search_Results,
        Daylist,
    }

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


pub mod state {
    struct Daylist_State {
        search_string: String,
        main_context_string: String,
        // ...
    }
    
    impl Daylist_State {
        pub fn init() -> Daylist_State {
            // do a bunch of calculations...

            Daylist_State {
                search_string: String::new(),
                main_context_string: String::new(),
            }
        }
    }
}
