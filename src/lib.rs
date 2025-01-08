pub mod schema;
pub mod db;

//https://patorjk.com/software/taag/#p=display&f=Tmplr&t=Daylist
pub const LOGO: &str = "
  ____          _ _     _   
 |    \\ ___ _ _| |_|___| |_ 
 |  |  | .'| | | | |_ -|  _|
 |____/|__,|_  |_|_|___|_|  
           |___|            

";
pub const LOGO2: &str = "
   ____              _ _     _   
  |  _ \\  __ _ _   _| (_)___| |_ 
  | | | |/ _` | | | | | / __| __|
  | |_| | (_| | |_| | | \\__ \\ |_ 
  |____/ \\__,_|\\__, |_|_|___/\\__|
               |___/             
";
pub const LOGO3: &str = "
┳┓    ┓•  
┃┃┏┓┓┏┃┓┏╋
┻┛┗┻┗┫┗┗┛┗
     ┛    
";
pub const LOGO4: &str = "
  ___            _ _    _   
 |   \\ __ _ _  _| (_)__| |_ 
 | |) / _` | || | | (_-<  _|
 |___/\\__,_|\\_, |_|_/__/\\__|
            |__/            
";


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
    use tui::layout::Rect;

    use crate::nav::Widget;
    use crate::nav::Content;


    pub struct App_State {
        running: bool,

        pub search_string: String,
        pub main_context_string: String,
        pub search_results: Vec<Todo>,

        pub edit_name: String,
        pub edit_description: String,
        pub edit_selection: Edit_Selection,

        pub focused_widget: Widget,
        pub main_content_shown: Content,

        pub todo_items_limit: u32,
        pub todo_items_offset: u32,

        pub upcoming_list: Vec<Todo>,
        // ...
    }

    impl App_State {
        pub fn init() -> App_State {
            // TODO
            // do a bunch of calculations...

            App_State {
                running: true,

                search_string: String::new(),
                main_context_string: String::new(),
                search_results: vec![],

                edit_name: String::new(),
                edit_description: String::new(),
                edit_selection: Edit_Selection::Name,

                focused_widget: Widget::Main,
                main_content_shown: Content::Daylist,

                todo_items_limit: 10,
                todo_items_offset: 0,
                
                upcoming_list: vec![],
            }
        }

        pub fn is_running(&self) -> bool {
            self.running
        } 
        pub fn exit(&mut self) {
            self.running = false;
        }
    }

    pub enum Edit_Selection {
        Name,
        Description,
    }

    pub struct Layout_State {
        pub chunks: Rect,
        pub columns: Rect,
        pub left_column: Rect,
        pub center_column: Rect,
        pub right_column: Rect,

        pub logo_block: Rect,
        pub search_box: Rect,
        pub main_content: Rect,
        pub upcoming_content: Rect,
        pub calendar_content: Rect,
        
        pub row: Rect,
        pub bottom_row_list: Rect,
    }

    impl Layout_State {
        pub fn init() -> Layout_State {
            Layout_State {
                chunks: Rect::default(),
                columns: Rect::default(),
                left_column: Rect::default(),
                center_column: Rect::default(),
                right_column: Rect::default(),
                
                logo_block: Rect::default(),
                search_box: Rect::default(),
                main_content: Rect::default(),
                upcoming_content: Rect::default(),
                calendar_content: Rect::default(),
                
                row: Rect::default(),
                bottom_row_list: Rect::default(),
            }
        }
    }

    use tui::widgets::{ListItem, ListState};
    use crate::schema::schema::Todo;
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
                    Some(self.todos[i].todo_id)
                }
                None => None
            }
        }
    }
}

pub mod utils {
    // TODO display popup message for user
    pub fn alert(message: &str) {}
}
