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
    use crate::nav::Widget;
    use crate::nav::Content;


    pub struct App_State {
        pub search_string: String,
        pub main_context_string: String,
        pub search_results: Vec<Todo>,

        pub todo_name: String,
        pub todo_description: String,
        pub edit_selection: Edit_Selection,

        pub focused_widget: Widget,
        pub main_content_shown: Content,

        pub todo_items_limit: u32,
        pub todo_items_offset: u32,
        // ...
    }

    impl App_State {
        pub fn init() -> App_State {
            // TODO
            // do a bunch of calculations...

            App_State {
                search_string: String::new(),
                main_context_string: String::new(),
                search_results: vec![],

                todo_name: String::new(),
                todo_description: String::new(),
                edit_selection: Edit_Selection::Name,

                focused_widget: Widget::Main,
                main_content_shown: Content::Daylist,

                todo_items_limit: 10,
                todo_items_offset: 0,
            }
        }
    }

    pub enum Edit_Selection {
        Name,
        Description,
    }

    pub struct Layout_State {
        pub chunks: tui::layout::Rect,
        pub columns: tui::layout::Rect,
        pub left_column: tui::layout::Rect,
        pub center_column: tui::layout::Rect,
        pub right_column: tui::layout::Rect,

        pub logo_block: tui::layout::Rect,
        pub search_box: tui::layout::Rect,
        pub main_content: tui::layout::Rect,
        pub right_top_block: tui::layout::Rect, // Upcoming
        pub right_bottom_block: tui::layout::Rect, // Calendar
        
        pub row: tui::layout::Rect,
        pub bottom_row_list: tui::layout::Rect,
    }

    impl Layout_State {
        pub fn init(&mut self) {
            self.chunks = tui::layout::Rect::default();
            self.columns = tui::layout::Rect::default();
            self.left_column = tui::layout::Rect::default();
            self.center_column = tui::layout::Rect::default();
            self.right_column = tui::layout::Rect::default();

            self.logo_block = tui::layout::Rect::default();
            self.search_box = tui::layout::Rect::default();
            self.main_content = tui::layout::Rect::default();
            self.right_top_block = tui::layout::Rect::default();
            self.right_bottom_block = tui::layout::Rect::default();

            self.row = tui::layout::Rect::default();
            self.bottom_row_list = tui::layout::Rect::default();
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
