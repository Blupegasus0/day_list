pub mod schema;
pub mod models;

pub mod db {
    use diesel::sqlite::SqliteConnection;
    use diesel::r2d2::{self, ConnectionManager, Pool};
    use diesel::prelude::*;
    use dotenvy::dotenv;
    use std::env;

    use crate::schema;
    use crate::models::NewTodo;
    use crate::models::Todo;

    type DbPool = Pool<ConnectionManager<SqliteConnection>>;
    
    pub fn establish_connection_pool() -> DbPool {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    
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
        
        format_todos(results)
    }

    pub fn search(connection: &mut SqliteConnection, target: &String) -> Vec<ListItem<'static>> {
        // -- Read
        let pattern = format!("%{}%", target);

        let results = schema::todo::table
            .filter(schema::todo::title.like(pattern))
            .load::<Todo>(connection)
            .expect("Error loading todos");

        let results_found = format!("Found {} todos matching '{}'\n", results.len(), target);
        let mut search_results = format_todos(results);
        
        search_results.insert(0,ListItem::new(results_found));
        search_results
    }

    fn format_todos(results: Vec<Todo>) -> Vec<ListItem<'static>> {
        let mut list: Vec<ListItem> = Vec::new();
        for todo in results {
            let todo_item = format!("\n   {}\n   {}\n", todo.title, 
                match todo.description {
                    Some(s) => s,
                    None => "--".to_string(),
                } 
            );
            list.push(ListItem::new(todo_item))
        }
        list
    }


    pub fn create(connection: &mut SqliteConnection, title: String, description: String) -> (String, Option<String>) {
        // -- Create
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

        (new_todo.title,new_todo.description)
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
        Daylist,
        Edit_Todo,
        Search_Results,
    }

    pub enum Widget {
        Calendar,
        Edit_Todo,
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


    use tui::widgets::ListState;
    pub struct Todo_List {
        todos: Vec<String>,
        state: ListState,
    }

    impl Todo_List {
        pub fn new(todos: Vec<String>) -> Todo_List {
            Todo_List {
                todos,
                state: ListState::default(),
            }
        }

        pub fn set_todos(&mut self, todos: Vec<String>) {
            self.todos = todos;
            self.state = ListState::default(); // Reset the state since the items have changed
        }

        // Select the next item. This will not be reflected until the widget is drawn in the
        // `Terminal::draw` callback using `Frame::render_stateful_widget`.
        pub fn next(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.todos.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }

        // Select the previous item. This will not be reflected until the widget is drawn in the
        // `Terminal::draw` callback using `Frame::render_stateful_widget`.
        pub fn previous(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.todos.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }

        // Unselect the currently selected item if any. The implementation of `ListState` makes
        // sure that the stored offset is also reset.
        pub fn unselect(&mut self) {
            self.state.select(None);
        }
    }
}
