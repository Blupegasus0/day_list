use std::error::Error;
use std::io;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, List, ListItem};
use tui::text::{Spans, Span};
use tui::Terminal;
use console::Term;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono;

use DayList::db;
use DayList::nav::Widget;
use DayList::nav::Content;


fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    
    //let state = state::init(); // Implement when the state is finalized

    // database conncection
    let connection = &mut db::establish_connection();
    // let pool = establish_connection_pool();
    // let conn = pool.get().expect("Failed to get a connection from the pool.");

    // State
    let mut search_string = String::new();
    let mut search_results = vec![ListItem::new("")];
    let mut focused_widget = Widget::Main; 
    let mut main_content_shown = Content::Daylist;
    let mut todo_items_limit = 10; // the amount of items displayed should depend
    let mut todo_items_offset = 0;
    
    // Widget Boundaries
    let mut search_bounds = Rect::default();
    let mut main_bounds = Rect::default();
    let mut calendar_bounds = Rect::default();
    let mut upcoming_bounds = Rect::default();
    

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
                        Constraint::Percentage(20),
                        Constraint::Percentage(80),
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



            // Define the blocks
            let left_top_block = Block::default().title("Left Top").borders(Borders::ALL);
            let left_bottom_block = Block::default().title("Left Bottom").borders(Borders::ALL);

            // State Assignments
            let mut search_widget = Paragraph::new(search_string.as_ref()).block(Block::default().title("Search")
                .borders(Borders::ALL));

            let mut main_content = List::new([ListItem::new("")].to_vec()).block(Block::default().title("Daylist")
                .borders(Borders::ALL));

            let mut right_top_block = Block::default().title("Upcoming").borders(Borders::ALL);
            let mut right_bottom_block = Block::default().title("Calendar").borders(Borders::ALL);


            // A list for the bottom row showing keyboard shortcuts
            let bottom_row_items = vec![
                ListItem::new(Span::raw("q: Quit")),
                ListItem::new(Span::raw("h: Help")),
            ];
            let bottom_row_list = List::new(bottom_row_items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default());

            // ERROR Still cant be looping this
            let mut daylist_items = db::fetch_todos(connection, todo_items_offset, todo_items_limit);

            let daylist_todos = List::new(daylist_items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default());

            let search_content = List::new(&*search_results.clone())
                    .block(Block::default().borders(Borders::ALL))
                    .highlight_style(Style::default());


            match main_content_shown {
                Content::Daylist => main_content = daylist_todos,
                // this is madness ERROR ...
                Content::Search_Results => main_content = search_content,
                _ => {},
            }

            // Display the focused widget
            match focused_widget {
                Widget::Main => main_content = main_content.style(Style::default().fg(Color::Yellow)),
                Widget::Search => search_widget = search_widget.style(Style::default().fg(Color::Yellow)),
                Widget::Calendar => right_bottom_block = right_bottom_block.style(Style::default().fg(Color::Yellow)),
                Widget::Upcoming => right_top_block = right_top_block.style(Style::default().fg(Color::Yellow)),
                _ => main_content = main_content.style(Style::default().fg(Color::Yellow)),
            };

            // Update boundaries
            search_bounds = center_column[0];
            main_bounds = center_column[1];
            upcoming_bounds = right_column[0];
            calendar_bounds = right_column[1];

            // Static blocks
            f.render_widget(left_top_block, left_column[0]);
            f.render_widget(left_bottom_block, left_column[1]);
            // f.render_widget(center_search_block, center_column[0]);
            // f.render_widget(center_main_block, center_column[1]);
            f.render_widget(right_top_block, right_column[0]);
            f.render_widget(right_bottom_block, right_column[1]);
            // f.render_widget(bottom_row_block, chunks[1]);
            f.render_widget(bottom_row_list, chunks[1]);

            // Dynamic Blocks
            f.render_widget(search_widget, center_column[0]);
            // f.render_widget(main_content, center_column[1]);
            f.render_widget(main_content, center_column[1]);
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
                        search_results = vec![ListItem::new(db::search(connection, &search_string))];
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

                    KeyCode::Char('L') => focused_widget = focused_widget.right(),

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

