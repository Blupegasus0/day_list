use std::error::Error;

use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };

use crate::model::db::Db;
use crate::model::schema::Todo;
use crate::controller::nav::Content;
use crate::controller::nav::Widget;
use crate::controller::state::AppState;
use crate::controller::state::EditSelection;
use crate::controller::state::TodoList;

//fn user_events(event: Event, app: AppState, todo_list: TodoList, list_db: &Db) {}

//fn keyboard_events(key: KeyCode, app: AppState, todo_list: TodoList, list_db: &Db) {}


pub async fn search_events(key: 
    KeyEvent, app: &mut AppState, list_db: &Db) -> Result<(), Box<dyn Error>> {
    match key.code {
        KeyCode::Esc => {
            app.focused_widget = Widget::Main;
            app.main_content_shown = Content::Daylist;
        }, 
        KeyCode::Char(c) => app.search_string.push(c), // append character to search string
        KeyCode::Backspace => {app.search_string.pop();}, // remove last character
        KeyCode::Enter => {
            // SUBMIT SEARCH STRING...
            // TODO update to lazy loading
            app.search_results = list_db.search(&app.search_string).await?;
            app.main_content_shown = Content::SearchResults;
        }

        KeyCode::Up => app.focused_widget = app.focused_widget.up(),
        KeyCode::Down => app.focused_widget = app.focused_widget.down(),
        KeyCode::Left => app.focused_widget = app.focused_widget.left(),
        KeyCode::Right => app.focused_widget = app.focused_widget.right(),
        _ => {} // Handle other keys as needed
    };
    Ok(())
}


pub fn default_events(key: KeyEvent, app: &mut AppState) {
    match key.code {
        KeyCode::Char('q') => app.exit(), // Quit on 'q' press
        KeyCode::Char('Q') => app.exit(), // Quit on 'Q' press
        KeyCode::Esc => app.exit(), // Exit on Escape key - We'll see if this is kept

        KeyCode::Char('k') => app.focused_widget = app.focused_widget.up(),
        KeyCode::Char('j') => app.focused_widget = app.focused_widget.down(),
        KeyCode::Char('h') => app.focused_widget = app.focused_widget.left(),
        KeyCode::Char('l') => app.focused_widget = app.focused_widget.right(),
        KeyCode::Up => app.focused_widget = app.focused_widget.up(),
        KeyCode::Down => app.focused_widget = app.focused_widget.down(),
        KeyCode::Left => app.focused_widget = app.focused_widget.left(),
        KeyCode::Right => app.focused_widget = app.focused_widget.right(),
        _ => {}, // Handle other keys as needed
    }
}

pub async fn list_events(key: KeyEvent, app: &mut AppState, todo_list: &mut TodoList, list_db: &Db) -> Result<(), Box<dyn Error>> {
    match key.code {
        KeyCode::Char('q') => app.exit(), // Quit on 'q' press
        KeyCode::Char('Q') => app.exit(), // Quit on 'Q' press
        KeyCode::Esc => app.main_content_shown = Content::Daylist,

        KeyCode::Char('L') => todo_list.set_todos(list_db.fetch_todos(app.todo_items_offset, app.todo_items_limit).await?),

        KeyCode::Char('n') => {
            app.focused_widget = Widget::EditTodo; 
            app.main_content_shown = Content::EditTodo;
        }

        KeyCode::Tab => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                print!("Going up captain!");
                todo_list.previous();
            } else {
                todo_list.next();
            }
        }

        KeyCode::Char('d') => {
            list_db.toggle_todo_status(todo_list.get_selected_id()).await?;
            todo_list.set_todos(list_db.fetch_todos(app.todo_items_offset, app.todo_items_limit).await?);
        },

        KeyCode::Char('X') => {
            list_db.delete_todo(todo_list.get_selected_id()).await?;
            todo_list.set_todos(list_db.fetch_todos(app.todo_items_offset, app.todo_items_limit).await?);
        },

        KeyCode::Char('k') => app.focused_widget = app.focused_widget.up(),
        KeyCode::Char('j') => app.focused_widget = app.focused_widget.down(),
        KeyCode::Char('h') => app.focused_widget = app.focused_widget.left(),
        KeyCode::Char('l') => app.focused_widget = app.focused_widget.right(),
        KeyCode::Up => app.focused_widget = app.focused_widget.up(),
        KeyCode::Down => app.focused_widget = app.focused_widget.down(),
        KeyCode::Left => app.focused_widget = app.focused_widget.left(),
        KeyCode::Right => app.focused_widget = app.focused_widget.right(),
        _ => {}, // Handle other keys as needed
    };
    Ok(())
}

pub async fn edit_events(key: KeyEvent, app: &mut AppState, todo_list: &mut TodoList, list_db: &Db) -> Result<(), Box<dyn Error>> {
    match key.code {
        KeyCode::Esc => {
            app.main_content_shown = Content::Daylist;
            app.focused_widget = Widget::Main;
            app.edit_name.clear();
            app.edit_description.clear();
            app.edit_date_due.clear();
            app.edit_reminder_date.clear();
            app.edit_priority = 4; // Magic Number
        },
        KeyCode::Backspace => {
            match app.edit_selection {
                EditSelection::Name => app.edit_name.pop(),
                EditSelection::Description => app.edit_description.pop(),
                EditSelection::DateDue => app.edit_date_due.pop(),
                EditSelection::ReminderDate => app.edit_reminder_date.pop(),
                // That none is there to satisfy the compiler
                EditSelection::Priority => { app.edit_priority = 0; None}, 
            };
        }, // remove last character
        KeyCode::Enter => {
            match app.edit_selection {
                EditSelection::Name => app.edit_selection = EditSelection::Description,
                EditSelection::Description => app.edit_selection = EditSelection::DateDue,
                EditSelection::DateDue => app.edit_selection = EditSelection::ReminderDate,
                EditSelection::ReminderDate => app.edit_selection = EditSelection::Priority,
                EditSelection::Priority => {
                    // Add todo 
                    let new_todo = Todo{
                        todo_id: 0,
                        title: app.edit_name.clone(), 
                        description: Some(app.edit_description.clone()),
                        date_created: None,
                        status: 0,
                        date_due: app.parse_due(), 
                        reminder_date: app.parse_reminder(), 
                        parent_todo: None, 
                        priority: app.edit_priority, 
                        project_id: None,
                    };
                    list_db.create_todo(&new_todo).await?;

                    app.main_content_shown = Content::Daylist;
                    app.focused_widget = Widget::Main;

                    app.edit_name.clear();
                    app.edit_description.clear();
                    app.edit_date_due.clear();
                    app.edit_reminder_date.clear();
                    app.edit_priority = 4; // Magic Number
                    app.edit_selection = EditSelection::Name;
                }
            }
            // Reload todos
            todo_list.set_todos(list_db.fetch_todos(app.todo_items_offset, app.todo_items_limit).await?);
        },
        //  KeyCode::Tab // TODO add tab functionality
        KeyCode::Char(c) => {
            match app.edit_selection {
                EditSelection::Name => app.edit_name.push(c),
                EditSelection::Description => app.edit_description.push(c),
                EditSelection::DateDue => app.edit_date_due.push(c),
                EditSelection::ReminderDate => app.edit_reminder_date.push(c),
                EditSelection::Priority => match c {
                    '1' => app.edit_priority = 1,
                    '2' => app.edit_priority = 2,
                    '3' => app.edit_priority = 3,
                    '4' => app.edit_priority = 4,
                    '5' => app.edit_priority = 5,
                    '6' => app.edit_priority = 6,
                    '7' => app.edit_priority = 7,
                    '8' => app.edit_priority = 8,
                    '9' => app.edit_priority = 9,
                    _ => app.edit_priority = 4,
                }
            }
        }, 

        KeyCode::Up => app.focused_widget = app.focused_widget.up(),
        KeyCode::Down => app.focused_widget = app.focused_widget.down(),
        KeyCode::Left => app.focused_widget = app.focused_widget.left(),
        KeyCode::Right => app.focused_widget = app.focused_widget.right(),
        _ => {},
    }
    Ok(())
}
