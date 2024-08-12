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

enum Widget {
    Calendar,
    Main,
    Search,
    Upcoming,
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;


    // State
    let mut search_string = String::new();
    let mut main_content_string = String::from("test");
    let mut focused_widget = Widget::Main; 
        focused_widget = Widget::Search; // TEMP
    
    // Initialize widget content 
    main_content_string = db::read();

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

            let center_search_block = Block::default().title("Search").borders(Borders::ALL);
            let center_main_block = Block::default().title("MyDaylist").borders(Borders::ALL);

            let right_top_block = Block::default().title("Upcoming").borders(Borders::ALL);
            let right_bottom_block = Block::default().title("Calendar").borders(Borders::ALL);

            // State Assignments
            let search_widget = Paragraph::new(search_string.as_ref()).block(Block::default().title("Search")
                .borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow));

            let main_content = Paragraph::new(main_content_string.as_ref()).block(Block::default().title("Search")
                .borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow));

            // A list for the bottom row showing keyboard shortcuts
            let bottom_row_items = vec![
                ListItem::new(Span::raw("q: Quit")),
                ListItem::new(Span::raw("h: Help")),
            ];
            let bottom_row_list = List::new(bottom_row_items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default());



            // Static blocks
            f.render_widget(left_top_block, left_column[0]);
            f.render_widget(left_bottom_block, left_column[1]);
            f.render_widget(center_search_block, center_column[0]);
            // f.render_widget(center_main_block, center_column[1]);
            f.render_widget(right_top_block, right_column[0]);
            f.render_widget(right_bottom_block, right_column[1]);
            // f.render_widget(bottom_row_block, chunks[1]);
            f.render_widget(bottom_row_list, chunks[1]);

            // Dynamic Blocks
            f.render_widget(search_widget, center_column[0]);
            f.render_widget(main_content, center_column[1]);
        })?;


        // Match on different types of events
        match event::read()? {
            // Handle keyboard events
            Event::Key(key) => match focused_widget {
                // Search box is focused
                Widget::Search => match key.code {
                    KeyCode::Esc => focused_widget = Widget::Main, // Refocus Main
                    KeyCode::Char(c) => search_string.push(c), // append character to search string
                    KeyCode::Backspace => {search_string.pop();}, // remove last character
                    KeyCode::Enter => {
                        // SUBMIT SEARCH STRING...
                        main_content_string = db::search(&search_string);
                        search_string.clear();
                    }
                    _ => {} // Handle other keys as needed
                },

                // Default Key handling
                _ =>  match key.code {
                    KeyCode::Char('q') => break, // Quit on 'q' press
                    KeyCode::Char('Q') => break, // Quit on 'Q' press
                    KeyCode::Esc => break, // Exit on Escape key - We'll see if this is kept
                    _ => {} // Handle other keys as needed
                },

            }


            // Handle Mouse events
            Event::Mouse(mouse_event) => match focused_widget {
                Widget::Main => match mouse_event.kind {
                    crossterm::event::MouseEventKind::Down(button) => {
                        //button, mouse_event.column, mouse_event.row
                    }
                    crossterm::event::MouseEventKind::Up(button) => {
                        // button, mouse_event.column, mouse_event.row
                    }
                    crossterm::event::MouseEventKind::Drag(button) => {
                        // button, mouse_event.column, mouse_event.row
                    }
                    crossterm::event::MouseEventKind::ScrollUp => {
                        //mouse_event.column, mouse_event.row
                    }
                    crossterm::event::MouseEventKind::ScrollDown => {
                        //mouse_event.column, mouse_event.row
                    }
                    _ => {}
                },
                
                // Default Mouse handling
                _ => match mouse_event.kind {
                    crossterm::event::MouseEventKind::Down(button) => {
                        //button, mouse_event.column, mouse_event.row
                    }
                    crossterm::event::MouseEventKind::Up(button) => {
                        // button, mouse_event.column, mouse_event.row
                    }
                    crossterm::event::MouseEventKind::Drag(button) => {
                        // button, mouse_event.column, mouse_event.row
                    }
                    crossterm::event::MouseEventKind::ScrollUp => {
                        //mouse_event.column, mouse_event.row
                    }
                    crossterm::event::MouseEventKind::ScrollDown => {
                        //mouse_event.column, mouse_event.row
                    }
                    _ => {}
                },
            },

            // Handle terminal resize if needed
            Event::Resize(width, height) => {
                // Draw large resizing message
                println!("Terminal resized to {}x{}", width, height); // tmp
            }
            _ => {}
        }
    }


    // Cleanup
    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

