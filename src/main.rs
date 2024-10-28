use std::error::Error;
use std::io;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, List, ListItem, Table, Row, Cell};
// use tui::text::{Spans, Span};
use tui::Terminal;
use console::Term;
use chrono;

use DayList::db;
use DayList::nav::Widget;
use DayList::nav::Content;
use DayList::state::Todo_List;


fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    
    //let state = state::init(); // Implement when the state is finalized

    // database conncection
    let pool = db::establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get a connection from the pool.");

    // State
    let mut search_string = String::new();
    let mut search_results = db::search(pool.clone(), &search_string);

    let mut todo_name = String::new();
    let mut todo_description = String::new();
    let mut todo_name_selected = true;

    let mut focused_widget = Widget::Main; 
    let mut main_content_shown = Content::Daylist;

    let mut todo_items_limit = 10; // the amount of items displayed should depend
    let mut todo_items_offset = 0;
    //let mut daylist_items = vec![ListItem::new("")];
    
    // Widget Boundaries
    let mut search_bounds = Rect::default();
    let mut main_bounds = Rect::default();
    let mut calendar_bounds = Rect::default();
    let mut upcoming_bounds = Rect::default();
    
    
    // testing daylist state
    let mut todo_list = Todo_List::new(db::fetch_todos(pool.clone(), todo_items_offset, todo_items_limit));

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
            let labels_block = Block::default().title("Labels").borders(Borders::ALL);

            // State Assignments
            let mut search_box = Paragraph::new(search_string.as_ref()).block(Block::default().title("Search")
                .borders(Borders::ALL));

            let mut main_content = List::new([ListItem::new("")].to_vec()).block(Block::default().title("Daylist")
                .borders(Borders::ALL));

            let mut right_top_block = Block::default().title("Upcoming").borders(Borders::ALL);
            let mut right_bottom_block = Block::default().title("Calendar").borders(Borders::ALL);

            // A list for the bottom row showing keyboard shortcuts
            let row = Row::new(vec![
                Cell::from("q|Quit"),
                Cell::from("Esc|Home"),
                Cell::from("n|New todo"),
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

            let edit_string = format!("Title: {}\n-----\nDescription: {}",todo_name, todo_description);
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

            let mut search_results = search_results.iter()
                    .map(|todo| ListItem::new(db::format_todo(todo)).style(Style::default().fg(Color::White)))
                    .collect::<Vec<ListItem<'_>>>(); // probably suboptimal
            let search_content = List::new(search_results)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default());


            match main_content_shown {
                Content::Daylist => main_content = daylist_todos,
                Content::Edit_Todo => main_content = edit_todo,
                Content::Search_Results => main_content = search_content,
                _ => {},
            }

            // Display the focused widget
            match focused_widget {
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
            Event::Key(key) => match focused_widget {
                // Search box is focused
                Widget::Search => match key.code {
                    KeyCode::Esc => {
                        focused_widget = Widget::Main;
                        main_content_shown = Content::Daylist;
                    }, 
                    KeyCode::Char(c) => search_string.push(c), // append character to search string
                    KeyCode::Backspace => {search_string.pop();}, // remove last character
                    KeyCode::Enter => {
                        // SUBMIT SEARCH STRING...
                        // to be updated to lazy loading
                        let mut conn = pool.get().expect("Failed to get a connection from the pool.");
                        search_results = db::search(pool.clone(), &search_string);
                        main_content_shown = Content::Search_Results;
                        search_string.clear();
                    }

                    KeyCode::Up => focused_widget = focused_widget.up(),
                    KeyCode::Down => focused_widget = focused_widget.down(),
                    KeyCode::Left => focused_widget = focused_widget.left(),
                    KeyCode::Right => focused_widget = focused_widget.right(),
                    _ => {} // Handle other keys as needed
                },

                Widget::Main =>  match key.code {
                    KeyCode::Char('q') => break, // Quit on 'q' press
                    KeyCode::Char('Q') => break, // Quit on 'Q' press
                    KeyCode::Esc => main_content_shown = Content::Daylist,

                    KeyCode::Char('L') => todo_list.set_todos(db::fetch_todos(pool.clone(), todo_items_offset, todo_items_limit)),

                    KeyCode::Char('n') => {
                        focused_widget = Widget::Edit_Todo; 
                        main_content_shown = Content::Edit_Todo;
                    }

                    KeyCode::Tab => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                todo_list.previous();
                            } else {
                                todo_list.next();
                            }
                        }

                    KeyCode::Char('d') => {
                        db::complete_todo(pool.clone(), todo_list.get_selected_id());
                        todo_list.set_todos(db::fetch_todos(pool.clone(), todo_items_offset, todo_items_limit));
                    },

                    KeyCode::Char('X') => {
                        db::delete_todo(pool.clone(), todo_list.get_selected_id());
                        todo_list.set_todos(db::fetch_todos(pool.clone(), todo_items_offset, todo_items_limit));
                    },

                    KeyCode::Char('k') => focused_widget = focused_widget.up(),
                    KeyCode::Char('j') => focused_widget = focused_widget.down(),
                    KeyCode::Char('h') => focused_widget = focused_widget.left(),
                    KeyCode::Char('l') => focused_widget = focused_widget.right(),
                    KeyCode::Up => focused_widget = focused_widget.up(),
                    KeyCode::Down => focused_widget = focused_widget.down(),
                    KeyCode::Left => focused_widget = focused_widget.left(),
                    KeyCode::Right => focused_widget = focused_widget.right(),
                    _ => {}, // Handle other keys as needed
                },

                Widget::Edit_Todo => match key.code {
                    KeyCode::Esc => {
                        main_content_shown = Content::Daylist;
                        focused_widget = Widget::Main;
                        todo_name.clear();
                        todo_description.clear();
                    },
                    KeyCode::Backspace => {
                        if todo_name_selected {todo_name.pop();}
                        else {todo_description.pop();}
                    }, // remove last character
                    KeyCode::Enter => {
                        if todo_name_selected {
                            todo_name_selected = false;
                        } else {
                            // Add todo 
                            let mut conn = pool.get().expect("Failed to get a connection from the pool.");
                            db::create(pool.clone(), todo_name.clone(), todo_description.clone());

                            main_content_shown = Content::Daylist;
                            focused_widget = Widget::Main;

                            todo_name.clear();
                            todo_description.clear();
                            todo_name_selected = true;
                        }
                        // Reload todos
                        todo_list.set_todos(db::fetch_todos(pool.clone(), todo_items_offset, todo_items_limit));
                    },
                    //  KeyCode::Tab .... add tab functionality TODO
                    KeyCode::Char(c) => {
                        if todo_name_selected {todo_name.push(c);}
                        else {todo_description.push(c);}
                    }, 

                    KeyCode::Up => focused_widget = focused_widget.up(),
                    KeyCode::Down => focused_widget = focused_widget.down(),
                    KeyCode::Left => focused_widget = focused_widget.left(),
                    KeyCode::Right => focused_widget = focused_widget.right(),
                    _ => {},
                },

                // Default Key handling
                _ =>  match key.code {
                    KeyCode::Char('q') => break, // Quit on 'q' press
                    KeyCode::Char('Q') => break, // Quit on 'Q' press
                    KeyCode::Esc => break, // Exit on Escape key - We'll see if this is kept

                    KeyCode::Char('k') => focused_widget = focused_widget.up(),
                    KeyCode::Char('j') => focused_widget = focused_widget.down(),
                    KeyCode::Char('h') => focused_widget = focused_widget.left(),
                    KeyCode::Char('l') => focused_widget = focused_widget.right(),
                    KeyCode::Up => focused_widget = focused_widget.up(),
                    KeyCode::Down => focused_widget = focused_widget.down(),
                    KeyCode::Left => focused_widget = focused_widget.left(),
                    KeyCode::Right => focused_widget = focused_widget.right(),
                    _ => {}, // Handle other keys as needed
                },

            }


            Event::Mouse(mouse_event) => match focused_widget {
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
                            focused_widget = Widget::Search;
                        }

                        if mouse_event.column >= main_bounds.x
                        && mouse_event.column < main_bounds.x + main_bounds.width
                        && mouse_event.row >= main_bounds.y
                        && mouse_event.row < main_bounds.y + main_bounds.height
                        {
                            focused_widget = Widget::Main;
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

