use tui::widgets::{Block, Borders, Paragraph, List, ListItem, Table, Row, Cell};
use tui::style::{Color, Style};
use tui::layout::{Constraint, Direction, Layout, Rect};
use chrono::{NaiveDateTime, Local};

use crate::controller::nav::Widget;
use crate::controller::nav::Content;
use crate::{LOGO, LOGO2, LOGO3, LOGO4};


pub struct AppState {
    running: bool,
    pub todo_list: TodoList,

    pub search_string: String,
    pub main_context_string: String,
    pub search_results: Vec<Todo>,

    // Might need to extract to a new struct
    pub edit_selection: EditSelection, // component state
    pub edit_name: String, // component state
    pub edit_description: String, // component state
    pub edit_date_due: String, // component state
    pub edit_reminder_date: String, // component state
    pub edit_priority: i64, // component state

    pub focused_widget: Widget,
    pub main_content_shown: Content, // component state

    pub todo_items_limit: u32, // component state
    pub todo_items_offset: u32, // component state

    pub upcoming_list: Vec<Todo>,
    // ...
}

impl AppState {
    pub fn init() -> AppState {
        // TODO
        // do a bunch of calculations...

        AppState {
            running: true,
            todo_list: TodoList::new(Vec::new()),

            search_string: String::new(),
            main_context_string: String::new(),
            search_results: vec![],

            // Might need to extract to a new struct
            edit_selection: EditSelection::Name,
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

pub enum EditSelection {
    Name,
    Description,
    DateDue,
    ReminderDate,
    Priority,
}

pub struct LayoutState<'a> {
    pub chunks: Vec<Rect>,
    pub columns: Vec<Rect>,
    pub left_column: Vec<Rect>,
    pub center_column: Vec<Rect>,
    pub right_column: Vec<Rect>,

    pub logo_block: Paragraph<'a>,
    pub search_box: Paragraph<'a>,
    pub main_content: List<'a>,
    pub upcoming_content: List<'a>,
    pub calendar_content: Table<'a>,
    pub projects_content: List<'a>,
    pub bottom_row_content: Table<'a>,

    pub search_bounds: Rect,
    pub main_bounds: Rect,
    pub calendar_bounds: Rect,
    pub upcoming_bounds: Rect,

}

impl LayoutState<'_> {
    pub fn init() -> LayoutState<'static> {
        LayoutState {
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

            calendar_content: Table::new(vec![])
                .block(Block::default().borders(Borders::ALL))
                .widths(&[
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                ]),

            bottom_row_content: Table::new(vec![])
                .block(Block::default().borders(Borders::ALL))
                .widths(&[
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                ]),

            projects_content: List::new(vec![])
                .block(Block::default().title("Projects").borders(Borders::ALL))
                .highlight_style(Style::default()),


            search_bounds: Rect::default(),
            main_bounds: Rect::default(),
            calendar_bounds: Rect::default(),
            upcoming_bounds: Rect::default(),
        }
    }

    pub fn update_bounds(&mut self) {
        self.search_bounds = self.center_column[0];
        self.main_bounds = self.center_column[1];
        self.upcoming_bounds = self.right_column[0];
        self.calendar_bounds = self.right_column[1];
    }

    pub fn structure(&mut self, frame_size: Rect) {
        // Split the screen into vertical chunks
        self.chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(0), // main section
                    Constraint::Length(3), // Bottom row for keyboard shortcuts
                ]
                    .as_ref(),
            )
            .split(frame_size);

        // Split the main area into 3 columns
        self.columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]
                    .as_ref(),
            )
            .split(self.chunks[0]);

        // Left column split into 20% and 80% vertically
        self.left_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(75),
                ]
                    .as_ref(),
            )
            .split(self.columns[0]);

        // Center column split with a search bar at the top
        self.center_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3), // Space for a search bar
                    Constraint::Min(0),    // The rest of the space
                ]
                    .as_ref(),
            )
            .split(self.columns[1]);

        // Right column split into 2 equal parts vertically
        self.right_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]
                    .as_ref(),
            )
            .split(self.columns[2]);


    } 
}

use tui::widgets::ListState;
use crate::model::schema::Todo;
pub struct TodoList {
    pub todos: Vec<Todo>,
    pub state: ListState,
}

impl TodoList {
    pub fn new(todos: Vec<Todo>) -> TodoList {
        TodoList {
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

    pub fn get_selected_id(&self) -> Option<i64> {
        match self.state.selected() {
            Some(i) => {
                Some(self.todos[i].todo_id)
            }
            None => None
        }
    }
}
