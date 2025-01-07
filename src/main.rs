use std::error::Error;
use std::io;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, 
    KeyModifiers, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags,PopKeyboardEnhancementFlags
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, List, ListItem, Table, Row, Cell};
use tui::Terminal;

use DayList::db::db;
use DayList::nav::Widget;
use DayList::nav::Content;
use DayList::state::Todo_List;
use DayList::state::App_State;
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

    let mut app = App_State::init();

    // State
    
    // Widget Boundaries
    let mut search_bounds = Rect::default();
    let mut main_bounds = Rect::default();
    let mut calendar_bounds = Rect::default();
    let mut upcoming_bounds = Rect::default();
    
    
    // testing daylist state 
    let mut todo_list = Todo_List::new(db::fetch_todos(&conn_pool, app.todo_items_offset, app.todo_items_limit).await?);

    loop {
        terminal.draw(|f| {
            // Split the screen into vertical chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Min(0), // main section
                        Constraint::Length(3), // Bottom row for keyboard shortcuts
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // Split the main area into 3 columns
            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(chunks[0]);

            // Left column split into 20% and 80% vertically
            let left_column = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(75),
                    ]
                    .as_ref(),
                )
                .split(columns[0]);

            // Center column split with a search bar at the top
            let center_column = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3), // Space for a search bar
                        Constraint::Min(0),    // The rest of the space
                    ]
                    .as_ref(),
                )
                .split(columns[1]);

            // Right column split into 2 equal parts vertically
            let right_column = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(columns[2]);


            // Define the block
            let logo_block = Paragraph::new(DayList::LOGO4).block(Block::default()).style(Style::default().fg(Color::Yellow));
            let labels_block = Block::default().title("Projects").borders(Borders::ALL);

            // State Assignments
            let mut search_box = Paragraph::new(app.search_string.as_ref()).block(Block::default().title("Search")
                .borders(Borders::ALL));

            let mut main_content = List::new([ListItem::new("")].to_vec()).block(Block::default().title("Daylist")
                .borders(Borders::ALL));

            let mut right_top_block = Block::default().title("Upcoming").borders(Borders::ALL);
            let mut right_bottom_block = Block::default().title("Calendar").borders(Borders::ALL);

            // A list for the bottom row showing keyboard shortcuts
            let row = Row::new(vec![
                Cell::from("q|Quit"),
                Cell::from("Esc|Home"),
                Cell::from("n|New"),
                Cell::from("d|Complete todo"),
                Cell::from("X|Delete todo"),
                Cell::from("L|List todos"),
                Cell::from("Tab|Select todo"),
            ]).style(Style::default().fg(Color::Yellow));

            let bottom_row_list = Table::new(vec![row])
                .block(Block::default().borders(Borders::ALL))
                .widths(&[
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
            ]);

            let edit_string = format!("Title: {}\n-----\nDescription: {}",app.todo_name, app.todo_description);
            let edit_item = vec![ListItem::new(edit_string)];

            let edit_todo = List::new(edit_item)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default());

            let daylist_todos = List::new(
                todo_list.todos.iter()
                    .map(|todo| ListItem::new(db::format_todo(todo)).style(Style::default().fg(Color::White)))
                    .collect::<Vec<ListItem<'_>>>()) // probably suboptimal
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(Style::default().fg(Color::Yellow).bg(Color::Black)); // Highlight the selected item

           let mut search_results = app.search_results.iter()
                    .map(|todo| ListItem::new(db::format_todo(todo)).style(Style::default().fg(Color::White)))
                    .collect::<Vec<ListItem<'_>>>(); // probably suboptimal
            let search_content = List::new(search_results)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default());


            match app.main_content_shown {
                Content::Daylist => main_content = daylist_todos,
                Content::Edit_Todo => main_content = edit_todo,
                Content::Search_Results => main_content = search_content,
                _ => {},
            }

            // Display the focused widget
            match app.focused_widget {
                Widget::Main => main_content = main_content.style(Style::default().fg(Color::Yellow)),
                Widget::Search => search_box = search_box.style(Style::default().fg(Color::Yellow)),
                Widget::Calendar => right_bottom_block = right_bottom_block.style(Style::default().fg(Color::Yellow)),
                Widget::Upcoming => right_top_block = right_top_block.style(Style::default().fg(Color::Yellow)),
                _ => main_content = main_content.style(Style::default().fg(Color::Yellow)),
            };

            // Update boundaries
            search_bounds = center_column[0];
            main_bounds = center_column[1];
            upcoming_bounds = right_column[0];
            calendar_bounds = right_column[1];


            f.render_widget(logo_block, left_column[0]);
            f.render_widget(labels_block, left_column[1]);
            
            // TODO - Sacrifice rendering these if the terminal size becomes too small
            f.render_widget(right_top_block, right_column[0]);
            f.render_widget(right_bottom_block, right_column[1]);

            f.render_widget(bottom_row_list, chunks[1]);

            f.render_widget(search_box, center_column[0]);

            f.render_stateful_widget(main_content, center_column[1], &mut todo_list.state);
        })?;


        // Match on different types of events
        match event::read()? {
            // Handle keyboard events
            Event::Key(key) => match app.focused_widget {
                // Search box is focused
                Widget::Search => match key.code {
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
                },

                Widget::Main =>  match key.code {
                    KeyCode::Char('q') => break, // Quit on 'q' press
                    KeyCode::Char('Q') => break, // Quit on 'Q' press
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
                },

                Widget::Edit_Todo => match key.code {
                    KeyCode::Esc => {
                        app.main_content_shown = Content::Daylist;
                        app.focused_widget = Widget::Main;
                        app.todo_name.clear();
                        app.todo_description.clear();
                    },
                    KeyCode::Backspace => {
                        match app.edit_selection {
                            Edit_Selection::Name => app.todo_name.pop(),
                            Edit_Selection::Description => app.todo_description.pop(),
                        };
                    }, // remove last character
                    KeyCode::Enter => {
                        match app.edit_selection {
                            Edit_Selection::Name => app.edit_selection = Edit_Selection::Description,
                            Edit_Selection::Description => {
                                // Add todo 
                                db::create_todo(
                                    &conn_pool, app.todo_name.clone(), Some(app.todo_description.clone()),
                                    None, None, None, 4, None
                                ).await?;

                                app.main_content_shown = Content::Daylist;
                                app.focused_widget = Widget::Main;

                                app.todo_name.clear();
                                app.todo_description.clear();
                                app.edit_selection = Edit_Selection::Name;
                            }
                        }
                        // Reload todos
                        todo_list.set_todos(db::fetch_todos(&conn_pool, app.todo_items_offset, app.todo_items_limit).await?);
                    },
                    //  KeyCode::Tab // TODO add tab functionality
                    KeyCode::Char(c) => {
                        match app.edit_selection {
                            Edit_Selection::Name => app.todo_name.push(c),
                            Edit_Selection::Description => app.todo_description.push(c),
                        }
                    }, 

                    KeyCode::Up => app.focused_widget = app.focused_widget.up(),
                    KeyCode::Down => app.focused_widget = app.focused_widget.down(),
                    KeyCode::Left => app.focused_widget = app.focused_widget.left(),
                    KeyCode::Right => app.focused_widget = app.focused_widget.right(),
                    _ => {},
                },

                // Default Key handling
                _ =>  match key.code {
                    KeyCode::Char('q') => break, // Quit on 'q' press
                    KeyCode::Char('Q') => break, // Quit on 'Q' press
                    KeyCode::Esc => break, // Exit on Escape key - We'll see if this is kept

                    KeyCode::Char('k') => app.focused_widget = app.focused_widget.up(),
                    KeyCode::Char('j') => app.focused_widget = app.focused_widget.down(),
                    KeyCode::Char('h') => app.focused_widget = app.focused_widget.left(),
                    KeyCode::Char('l') => app.focused_widget = app.focused_widget.right(),
                    KeyCode::Up => app.focused_widget = app.focused_widget.up(),
                    KeyCode::Down => app.focused_widget = app.focused_widget.down(),
                    KeyCode::Left => app.focused_widget = app.focused_widget.left(),
                    KeyCode::Right => app.focused_widget = app.focused_widget.right(),
                    _ => {}, // Handle other keys as needed
                },

            }


            Event::Mouse(mouse_event) => match app.focused_widget {
                // Default Mouse handling
                _ => match mouse_event.kind {
                    crossterm::event::MouseEventKind::Down(button) => {
                        //button, mouse_event.column, mouse_event.row

                        // Check if the mouse click is within the bounds of the search bar
                        if mouse_event.column >= search_bounds.x
                        && mouse_event.column < search_bounds.x + search_bounds.width
                        && mouse_event.row >= search_bounds.y
                        && mouse_event.row < search_bounds.y + search_bounds.height
                        {
                            app.focused_widget = Widget::Search;
                        }

                        if mouse_event.column >= main_bounds.x
                        && mouse_event.column < main_bounds.x + main_bounds.width
                        && mouse_event.row >= main_bounds.y
                        && mouse_event.row < main_bounds.y + main_bounds.height
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
    //crossterm::execute!(stdout, 
    //    PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,),
    //    PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES)
    //)?;
    terminal.show_cursor()?;

    Ok(())
} //main
