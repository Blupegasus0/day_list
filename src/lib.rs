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
    use tui::widgets::{Block, Borders, Paragraph, List, ListItem, Table, Row, Cell};
    use tui::style::{Color, Style};
    use chrono::{NaiveDateTime, Local};

    use crate::nav::Widget;
    use crate::nav::Content;
    use crate::{LOGO, LOGO2, LOGO3, LOGO4};


    pub struct App_State {
        running: bool,
        pub todo_list: Todo_List,

        pub search_string: String,
        pub main_context_string: String,
        pub search_results: Vec<Todo>,

        // Might need to extract to a new struct
        pub edit_selection: Edit_Selection,
        pub edit_name: String,
        pub edit_description: String,
        pub edit_date_due: String,
        pub edit_reminder_date: String,
        pub edit_priority: i32,

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
                todo_list: Todo_List::new(Vec::new()),

                search_string: String::new(),
                main_context_string: String::new(),
                search_results: vec![],

                // Might need to extract to a new struct
                edit_selection: Edit_Selection::Name,
                edit_name: String::new(),
                edit_description: String::new(),
                edit_date_due: String::new(),
                edit_reminder_date: String::new(),
                edit_priority: 4,

                focused_widget: Widget::Main,
                main_content_shown: Content::Daylist,

                todo_items_limit: 10,
                todo_items_offset: 0,
                
                upcoming_list: vec![],
            }
        }

        pub fn parse_due(&self) -> Option<NaiveDateTime> {
            NaiveDateTime::parse_from_str(self.edit_date_due.as_str(), "%d/%m/%y %H:%M").ok()
        }
        pub fn parse_reminder(&self) -> Option<NaiveDateTime> {
            NaiveDateTime::parse_from_str(self.edit_reminder_date.as_str(), "%d/%m/%y %H:%M").ok()
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
        DateDue,
        ReminderDate,
        Priority,
    }

    pub struct Layout_State<'a> {
        pub chunks: Vec<Rect>,
        pub columns: Vec<Rect>,
        pub left_column: Vec<Rect>,
        pub center_column: Vec<Rect>,
        pub right_column: Vec<Rect>,

        pub logo_block: Paragraph<'a>,
        pub search_box: Paragraph<'a>,
        pub main_content: List<'a>,
        pub upcoming_content: List<'a>,
        pub calendar_content: List<'a>,
        
        pub row: Rect,
        pub bottom_row_list: Rect,
    }

    impl Layout_State<'_> {
        pub fn init() -> Layout_State<'static> {
            Layout_State {
                chunks: vec![],
                columns: vec![],
                left_column: vec![],
                center_column: vec![],
                right_column: vec![],

                logo_block: Paragraph::new(LOGO4).block(Block::default()).style(Style::default().fg(Color::Yellow)),

                search_box: Paragraph::new(String::from("")).block(Block::default().title("Search")
                    .borders(Borders::ALL)),

                main_content: List::new([ListItem::new("")].to_vec()).block(Block::default().title("Daylist")
                    .borders(Borders::ALL)),

                upcoming_content: List::new([ListItem::new("")].to_vec()).block(Block::default().title("Upcoming")
                    .borders(Borders::ALL)),

                calendar_content: List::new([ListItem::new("")].to_vec()).block(Block::default().title("Calendar")
                    .borders(Borders::ALL)),

                row: Rect::default(),
                bottom_row_list: Rect::default(),
            }
        }
    }

    use tui::widgets::ListState;
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
