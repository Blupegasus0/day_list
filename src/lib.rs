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

    use tui::widgets::{List, ListItem};
    use tui::style::{Color, Style};
    pub fn fetch_todos<'a>(pool: DbPool, offset: i64, limit: i64) -> Vec<ListItem<'a>> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool.");
        let results = schema::todo::table
            .limit(limit)
            .offset(offset)
            .load::<Todo>(&mut conn)
            .expect("Error loading items");
        
        format_todos(results)
    }

    pub fn fetch_todos2(pool: DbPool, offset: i64, limit: i64) -> Vec<Todo> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool.");
        let results = schema::todo::table
            .limit(limit)
            .offset(offset)
            .load::<Todo>(&mut conn)
            .expect("Error loading items");
        results   
    }

    pub fn search(pool: DbPool, target: &String) -> Vec<ListItem<'static>> {
        // -- Read
        let pattern = format!("%{}%", target);

        let mut conn = pool.get().expect("Failed to get a connection from the pool.");
        let results = schema::todo::table
            .filter(schema::todo::title.like(pattern))
            .load::<Todo>(&mut conn)
            .expect("Error loading todos");

        let results_found = format!("Found {} todos matching '{}'\n", results.len(), target);
        let mut search_results = format_todos(results);
        
        search_results.insert(0,ListItem::new(results_found));
        search_results
    }

    pub fn format_todo(todo: &Todo) -> String {
        let mut todo_status = "[ ]";
        if todo.completed {
            todo_status = "[]";
        }

        format!("\n   {} {}\n       {}\n",todo_status, todo.title, 
            match todo.description.clone() {
                Some(s) => s,
                None => "--".to_string(),
            } 
        )
    }

    // TODO
    fn format_todos(results: Vec<Todo>) -> Vec<ListItem<'static>> {
        let mut list: Vec<ListItem> = Vec::new();
        for todo in results {
            let todo_item = format!("\n   {}\n   {}\n", todo.title, 
                match todo.description {
                    Some(s) => s,
                    None => "--".to_string(),
                } 
            );
            list.push(
                ListItem::new(todo_item).style(Style::default().fg(Color::DarkGray))
            )
        }
        list
    }

    pub fn create(pool: DbPool, title: String, description: String) -> (String, Option<String>) {
        // -- Create
        let new_todo = NewTodo { 
            title: title, 
            description: Some(description), 
            completed: false, 
            parent_todo_id: None 
        };

        let mut conn = pool.get().expect("Failed to get a connection from the pool.");
        diesel::insert_into(schema::todo::table)
            .values(&new_todo)
            .execute(&mut conn)
            .expect("Error saving new todo");

        (new_todo.title,new_todo.description)
    }

    pub fn complete_todo(pool: DbPool, id: Option<i32>) {
        match id {
            Some(id) => {
                let mut conn = pool.get().expect("Failed to get a connection from the pool.");
                let todo = diesel::update(schema::todo::table.find(id))
                    .set(schema::todo::completed.eq(true))
                    .execute(&mut conn)
                    .unwrap();

            }
            None => {} 
        }
    }

    pub fn delete_todo(pool: DbPool, id: Option<i32>) {
        match id {
            Some(id) => {
                let mut conn = pool.get().expect("Failed to get a connection from the pool.");
                let num_deleted = diesel::delete(schema::todo::table.find(id))
                    .execute(&mut conn)
                    .expect("Error deleting posts");

            }
            None => {} 
        }
    }

    pub fn update(pool: DbPool, id: Option<i32>, title: String, description: String) {
        match id {
            Some(id) => {
                let mut conn = pool.get().expect("Failed to get a connection from the pool.");
                let todo = diesel::update(schema::todo::table.find(id))
                    .set((
                            schema::todo::title.eq(title),
                            schema::todo::description.eq(description),
                        ))
                    .execute(&mut conn)
                    .unwrap();
                }
            None => {} 
        }
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


    use tui::widgets::{ListItem, ListState};
    use crate::models::Todo;
    pub struct Todo_List {
        pub todos: Vec<Todo>,
        pub state: ListState,
    }

    impl Todo_List {
        pub fn new(todos: Vec<Todo>) -> Todo_List {
            Todo_List {
                todos,
                state: ListState::default(),
            }
        }

        pub fn set_todos(&mut self, todos: Vec<Todo>) {
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

        pub fn get_selected_id(&self) -> Option<i32> {
            match self.state.selected() {
                Some(i) => {
                    Some(self.todos[i].id)
                }
                None => None
            }
        }
    }
}
