use std::error::Error;
use std::io;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent,
    KeyModifiers, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags,PopKeyboardEnhancementFlags
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, List, ListItem, Table, Row, Cell};
use tui::Terminal;
use sqlx::mysql::MySqlPool;

use DayList::db::db;
use DayList::nav::Widget;
use DayList::nav::Content;
use DayList::state::Todo_List;
use DayList::state::App_State;
use DayList::state::Layout_State;
use DayList::state::Edit_Selection;


#[tokio::main]
async fn main() {
    run().await.expect("Daylist encountered an error...");
}

async fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    //let state = state::init(); // Implement when the state is finalized

    // database conncection
    let conn_pool = db::establish_connection().await.expect("Failed to connect to db. Try Again.");

    // State
    let mut app = App_State::init();
    let mut layout = Layout_State::init();

    app.upcoming_list = db::fetch_upcoming_todos(&conn_pool, app.todo_items_offset, app.todo_items_limit).await?;
    app.todo_list = Todo_List::new(db::fetch_todos(&conn_pool, app.todo_items_offset, app.todo_items_limit).await?);
    let mut todo_list = Todo_List::new(db::fetch_todos(&conn_pool, app.todo_items_offset, app.todo_items_limit).await?); // ERROR redundant

    loop {
        // my ghetto way to exit the program, forgot the right way
        if !app.is_running() { break; } 

        terminal.draw(|frame| {
            layout.structure(frame.size());
            layout.update_bounds();

            // Define the block
            let projects_block = Block::default().title("Projects").borders(Borders::ALL);

            // State Assignments
            layout.search_box = Paragraph::new(app.search_string.clone()).block(Block::default().title("Search")
                .borders(Borders::ALL));

            // Edit todo... {
            let edit_string = format!("
                Title: {}
               
                Description: {}
              

                DD/MM/YY HH:MM

                Date Due: {}
             
                Reminder Date: {}
            
                Priority 1-9:  {}
                ",
                app.edit_name, app.edit_description,
                app.edit_date_due, app.edit_reminder_date,
                app.edit_priority
            );
            let edit_item = vec![ListItem::new(edit_string)];

            let edit_todo = List::new(edit_item)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default());
            // Edit todo... }

            let daylist_todos = List::new(
                todo_list.todos.iter()
                    .map(|todo| ListItem::new(todo.format()).style(Style::default().fg(Color::White)))
                    .collect::<Vec<ListItem<'_>>>()) // probably suboptimal
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(Style::default().fg(Color::Yellow).bg(Color::Black)); // Highlight the selected item

            // Initialize calendar content here
            // It is remaining yellow after selection because it is not being reset here

            layout.upcoming_content = List::new(
                app.upcoming_list.iter()
                    .map(|todo| ListItem::new(todo.format()).style(Style::default().fg(Color::White)))
                    .collect::<Vec<ListItem<'_>>>()) // probably suboptimal
                .block(Block::default().borders(Borders::ALL).title("Upcoming"))
                .highlight_style(Style::default().fg(Color::Yellow).bg(Color::Black)); // Highlight the selected item

            let mut search_results = app.search_results.iter()
                .map(|todo| ListItem::new(todo.format()).style(Style::default().fg(Color::White)))
                .collect::<Vec<ListItem<'_>>>(); // probably suboptimal
            let search_content = List::new(search_results)
                .block(Block::default().borders(Borders::ALL).title("Search Result"))
                .highlight_style(Style::default().fg(Color::Yellow).bg(Color::Black)); // Highlight the selected item);


            match app.main_content_shown {
                Content::Daylist => layout.main_content = daylist_todos,
                Content::Edit_Todo => layout.main_content = edit_todo,
                Content::Search_Results => layout.main_content = search_content,
                _ => {},
            }

            // A list for the bottom row showing keyboard shortcuts
            let default_keybinds = vec![
                Cell::from("q|Quit"),
                Cell::from("Esc|Home"),
                Cell::from("n|New"),
                Cell::from("d|Complete todo"),
                Cell::from("X|Delete todo"),
                Cell::from("L|List todos"),
                Cell::from("Tab|Navigate Todos"),
            ];
            
            let search_keybinds = vec![
                Cell::from("Esc|Home"),
                Cell::from("d|Complete todo"),
                Cell::from("X|Delete todo"),
                Cell::from("Tab|Navigate Todos"),
            ];
            
            let calendar_keybinds = vec![
                Cell::from("q|Quit"),
                Cell::from("Esc|Home"),
                Cell::from("n|New"),
                Cell::from("Tab|Navigate Todos"),
                Cell::from("Arrows|Navigate Calendar"),
            ];
            
            let upcoming_keybinds = vec![
                Cell::from("q|Quit"),
                Cell::from("Esc|Home"),
                Cell::from("n|New"),
                Cell::from("d|Complete todo"),
                Cell::from("X|Delete todo"),
                Cell::from("L|List todos"),
                Cell::from("Tab|Navigate Todos"),
            ];
            
            let project_keybinds = vec![
                Cell::from("q|Quit"),
                Cell::from("Esc|Home"),
                Cell::from("n|New"),
                Cell::from("d|Complete todo"),
                Cell::from("X|Delete todo"),
                Cell::from("L|List todos"),
                Cell::from("Tab|Navigate Todos"),
            ];

            let mut current_keybinds = default_keybinds;
            
            // Display the focused widget in the main content area
            match app.focused_widget {
                Widget::Main => {
                    layout.main_content = layout.main_content.clone().style(Style::default().fg(Color::Yellow));
                    // default keybinds
                }
                Widget::Search => {
                    layout.search_box = layout.search_box.clone().style(Style::default().fg(Color::Yellow));
                    current_keybinds = search_keybinds;
                }
                Widget::Calendar => {
                    layout.calendar_content = layout.calendar_content.clone().style(Style::default().fg(Color::Yellow));
                    current_keybinds = calendar_keybinds;
                }
                Widget::Upcoming => {
                    layout.upcoming_content = layout.upcoming_content.clone().style(Style::default().fg(Color::Yellow));
                    current_keybinds = upcoming_keybinds;
                }
                _ => {
                    layout.main_content = layout.main_content.clone().style(Style::default().fg(Color::Yellow));
                    // default keybinds
                }
            };

            let bottom_row_list = Row::new(current_keybinds).style(Style::default().fg(Color::Yellow));

            let days = ["sun", "mon", "tue", "wed", "thur", "fri", "sat"];
            let week = ["1","1","1","1","1","1","1"];
            let day_row = Row::new(days).style(Style::default().fg(Color::Yellow));
            let week_row = Row::new(week).style(Style::default().fg(Color::Yellow));
            layout.calendar_content = Table::new(vec![day_row, week_row.clone(), week_row.clone(), week_row.clone(), week_row.clone(), week_row])
                .block(Block::default().borders(Borders::ALL))
                .widths(&[
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                ]);

            layout.bottom_row_content = Table::new(vec![bottom_row_list])
                .block(Block::default().borders(Borders::ALL))
                .widths(&[
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                ]);


            frame.render_widget(layout.logo_block.clone(), layout.left_column[0]);
            frame.render_widget(projects_block, layout.left_column[1]);

            frame.render_widget(layout.bottom_row_content.clone(), layout.chunks[1]);

            frame.render_widget(layout.search_box.clone(), layout.center_column[0]);

            frame.render_stateful_widget(layout.main_content.clone(), layout.center_column[1], &mut todo_list.state);

            // TODO - Sacrifice rendering these if the terminal size becomes too small
            frame.render_stateful_widget(layout.upcoming_content.clone(), layout.right_column[0], &mut todo_list.state);
            // Supposed to be stateful but I need to create a struct for tables
            frame.render_widget(layout.calendar_content.clone(), layout.right_column[1]);

            //render_layout(layout, &mut f);
        })?;


        // Match on different types of events
        let user_event = event::read()?;
        //handle_user_events(user_event, app, todo_list, &conn_pool);
        match user_event {
            // Handle keyboard events
            Event::Key(key) => match app.focused_widget {
                // Search box is focused
                Widget::Search => handle_search_events(key, &mut app, &conn_pool).await?,

                Widget::Main => handle_list_events(key, &mut app, &mut todo_list, &conn_pool).await?,

                Widget::Edit_Todo => handle_edit_events(key, &mut app, &mut todo_list, &conn_pool).await?,

                // Default Key handling
                _ => handle_default_events(key, &mut app),
            },


            Event::Mouse(mouse_event) => match app.focused_widget {
                // Default Mouse handling
                _ => match mouse_event.kind {
                    crossterm::event::MouseEventKind::Down(button) => {
                        //button, mouse_event.column, mouse_event.row

                        // Check if the mouse click is within the bounds of the search bar
                        if mouse_event.column >= layout.search_bounds.x
                        && mouse_event.column < layout.search_bounds.x + layout.search_bounds.width
                        && mouse_event.row >= layout.search_bounds.y
                        && mouse_event.row < layout.search_bounds.y + layout.search_bounds.height
                        {
                            app.focused_widget = Widget::Search;
                        }

                        if mouse_event.column >= layout.main_bounds.x
                        && mouse_event.column < layout.main_bounds.x + layout.main_bounds.width
                        && mouse_event.row >= layout.main_bounds.y
                        && mouse_event.row < layout.main_bounds.y + layout.main_bounds.height
                        {
                            app.focused_widget = Widget::Main;
                        }
                    }
                    _ => {}

                }
            },

            // Handle terminal resize if needed
            Event::Resize(width, height) => {
                // Draw large resizing message
                println!("Terminal resized to {}x{}", width, height); // tmp
            }
            _ => {}
        }

    } //running loop


    // Cleanup
    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
} //main


fn render_layout<'a, B>(layout: Layout_State<'a>, f: tui::Frame<'a, B>) -> Layout_State<'a> where B: Backend {
    layout
}

fn handle_user_events(event: Event, app: App_State, todo_list: Todo_List, conn_pool: &MySqlPool) {
}

fn handle_keyboard_events(key: KeyCode, app: App_State, todo_list: Todo_List, conn_pool: &MySqlPool) {

}
async fn handle_search_events(key: KeyEvent, app: &mut App_State, conn_pool: &MySqlPool) -> Result<(), Box<dyn Error>> {
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
            app.search_results = db::search(&conn_pool, &app.search_string).await?;
            app.main_content_shown = Content::Search_Results;
        }

        KeyCode::Up => app.focused_widget = app.focused_widget.up(),
        KeyCode::Down => app.focused_widget = app.focused_widget.down(),
        KeyCode::Left => app.focused_widget = app.focused_widget.left(),
        KeyCode::Right => app.focused_widget = app.focused_widget.right(),
        _ => {} // Handle other keys as needed
    };
    Ok(())
}


fn handle_default_events(key: KeyEvent, app: &mut App_State) {
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

async fn handle_list_events(key: KeyEvent, app: &mut App_State, todo_list: &mut Todo_List, conn_pool: &MySqlPool) -> Result<(), Box<dyn Error>> {
    match key.code {
        KeyCode::Char('q') => app.exit(), // Quit on 'q' press
        KeyCode::Char('Q') => app.exit(), // Quit on 'Q' press
        KeyCode::Esc => app.main_content_shown = Content::Daylist,

        KeyCode::Char('L') => todo_list.set_todos(db::fetch_todos(&conn_pool, app.todo_items_offset, app.todo_items_limit).await?),

        KeyCode::Char('n') => {
            app.focused_widget = Widget::Edit_Todo; 
            app.main_content_shown = Content::Edit_Todo;
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
            db::toggle_todo_status(&conn_pool, todo_list.get_selected_id()).await?;
            todo_list.set_todos(db::fetch_todos(&conn_pool, app.todo_items_offset, app.todo_items_limit).await?);
        },

        KeyCode::Char('X') => {
            db::delete_todo(&conn_pool, todo_list.get_selected_id()).await?;
            todo_list.set_todos(db::fetch_todos(&conn_pool, app.todo_items_offset, app.todo_items_limit).await?);
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

async fn handle_edit_events(key: KeyEvent, app: &mut App_State, todo_list: &mut Todo_List, conn_pool: &MySqlPool) -> Result<(), Box<dyn Error>> {
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
                Edit_Selection::Name => app.edit_name.pop(),
                Edit_Selection::Description => app.edit_description.pop(),
                Edit_Selection::DateDue => app.edit_date_due.pop(),
                Edit_Selection::ReminderDate => app.edit_reminder_date.pop(),
                // That none is there to satisfy the compiler
                Edit_Selection::Priority => { app.edit_priority = 0; None}, 
            };
        }, // remove last character
        KeyCode::Enter => {
            match app.edit_selection {
                Edit_Selection::Name => app.edit_selection = Edit_Selection::Description,
                Edit_Selection::Description => app.edit_selection = Edit_Selection::DateDue,
                Edit_Selection::DateDue => app.edit_selection = Edit_Selection::ReminderDate,
                Edit_Selection::ReminderDate => app.edit_selection = Edit_Selection::Priority,
                Edit_Selection::Priority => {
                    // Add todo 
                    db::create_todo(
                        &conn_pool, app.edit_name.clone(), Some(app.edit_description.clone()),
                        app.parse_due(), app.parse_reminder(), None, app.edit_priority, None
                    ).await?;

                    app.main_content_shown = Content::Daylist;
                    app.focused_widget = Widget::Main;

                    app.edit_name.clear();
                    app.edit_description.clear();
                    app.edit_date_due.clear();
                    app.edit_reminder_date.clear();
                    app.edit_priority = 4; // Magic Number
                    app.edit_selection = Edit_Selection::Name;
                }
            }
            // Reload todos
            todo_list.set_todos(db::fetch_todos(&conn_pool, app.todo_items_offset, app.todo_items_limit).await?);
        },
        //  KeyCode::Tab // TODO add tab functionality
        KeyCode::Char(c) => {
            match app.edit_selection {
                Edit_Selection::Name => app.edit_name.push(c),
                Edit_Selection::Description => app.edit_description.push(c),
                Edit_Selection::DateDue => app.edit_date_due.push(c),
                Edit_Selection::ReminderDate => app.edit_reminder_date.push(c),
                Edit_Selection::Priority => match c {
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
