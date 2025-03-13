use std::error::Error;

use crossterm::event::{self, Event};
use tui::layout::Constraint;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, List, ListItem, Table, Row, Cell};
use tui::Terminal;

use crate::model::db::Db;
use crate::controller::nav::Widget;
use crate::controller::nav::Content;
use crate::controller::state::TodoList;
use crate::controller::state::AppState;
use crate::controller::state::LayoutState;
use crate::controller::handle;

pub async fn run<B>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> 
    where B: tui::backend::Backend
{

    // database conncection
    let list_db = Db::new().await;

    // State
    let mut app = AppState::init();
    let mut layout = LayoutState::init();

    app.upcoming_list = list_db.fetch_upcoming_todos(app.todo_items_offset, app.todo_items_limit).await?;
    app.todo_list = TodoList::new(list_db.fetch_todos(app.todo_items_offset, app.todo_items_limit).await?);
    let mut todo_list = TodoList::new(list_db.fetch_todos(app.todo_items_offset, app.todo_items_limit).await?); // ERROR redundant

    loop {
        // my ghetto way to exit the program, forgot the right way
        if !app.is_running() { break; } 

        terminal.draw(|frame| {
            layout.structure(frame.size());
            layout.update_bounds();

            // TODO move to edit_todo
            // Edit todo... {
            let edit_string = format!("
                Title: {}
               
                Description: {}
              

                DD/MM/YY HH:MM

                Date Due: {}
             
                Reminder Date: {}
            
                Priority 1-9:  {}
                ",
                app.edit.name, app.edit.description,
                app.edit.date_due, app.edit.reminder_date,
                app.edit.priority
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

            //let search_content = generate_search_results(&app, &mut layout); // ERROR
            layout.search_box = Paragraph::new(app.search_string.clone()).block(Block::default().title("Search")
                .borders(Borders::ALL));
            let search_results = app.search_results.iter()
                .map(|todo| ListItem::new(todo.format()).style(Style::default().fg(Color::White)))
                .collect::<Vec<ListItem<'_>>>(); // probably suboptimal
            let search_content = List::new(search_results)
                .block(Block::default().borders(Borders::ALL).title("Search Result"))
                .highlight_style(Style::default().fg(Color::Yellow).bg(Color::Black)); // Highlight the selected item);

            match app.main_content_shown {
                Content::Daylist => layout.main_content = daylist_todos,
                Content::EditTodo => layout.main_content = edit_todo,
                Content::SearchResults => layout.main_content = search_content,
                _ => {},
            }

            // actually put content in that mf. 
            // Later extract to its own generate function
            layout.projects_content = List::new(vec![])
                .block(Block::default().title("Projects").borders(Borders::ALL))
                .highlight_style(Style::default());


            generate_calendar(&mut layout);

            show_focused_widget(&app, &mut layout);

            frame.render_widget(layout.logo_block.clone(), layout.left_column[0]);
            frame.render_widget(layout.projects_content.clone(), layout.left_column[1]);

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
        match user_event {
            // Handle keyboard events
            Event::Key(key) => match app.focused_widget {
                Widget::Search => handle::search_events(key, &mut app, &list_db).await?,
                Widget::Main => handle::list_events(key, &mut app, &mut todo_list, &list_db).await?,
                Widget::EditTodo => handle::edit_events(key, &mut app, &mut todo_list, &list_db).await?,
                _ => handle::default_events(key, &mut app),
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


    Ok(())
} //run

fn generate_search_results<'a>(app: &'a AppState, layout: &'a mut LayoutState<'a>) -> List<'a> {
            layout.search_box = Paragraph::new(app.search_string.clone()).block(Block::default().title("Search")
                .borders(Borders::ALL));
            let search_results = app.search_results.iter()
                .map(|todo| ListItem::new(todo.format()).style(Style::default().fg(Color::White)))
                .collect::<Vec<ListItem<'_>>>(); // probably suboptimal
            let search_content = List::new(search_results)
                .block(Block::default().borders(Borders::ALL).title("Search Result"))
                .highlight_style(Style::default().fg(Color::Yellow).bg(Color::Black)); // Highlight the selected item);

            search_content
}

fn generate_calendar(layout: &mut LayoutState) {
    // ERROR calendar proof of concept
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
}

fn show_focused_widget(app: &AppState, layout: &mut LayoutState) {
    // A list for the bottom row showing keyboard shortcuts
    let default_keybinds = vec![
        Cell::from("q|Quit"),
        Cell::from("Esc|Home"),
        Cell::from("n|New"),
        Cell::from("e|Edit"),
        Cell::from("d|Complete todo"),
        Cell::from("X|Delete todo"),
        Cell::from("L|List todos"),
        Cell::from("Tab|Navigate Todos"),
    ];

    let search_keybinds = vec![
        Cell::from("Esc|Home"),
        Cell::from("d|Complete todo"),
        Cell::from("e|Edit"),
        Cell::from("X|Delete todo"),
        Cell::from("Tab|Navigate Todos"),
        Cell::from("Enter|Search!"),
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
        Cell::from("e|Edit"),
        Cell::from("d|Complete todo"),
        Cell::from("X|Delete todo"),
        Cell::from("L|List todos"),
        Cell::from("Tab|Navigate Todos"),
    ];

    let projects_keybinds = vec![
        Cell::from("q|Quit"),
        Cell::from("Esc|Home"),
        Cell::from("n|New"),
        Cell::from("X|Delete"),
        Cell::from("e|Edit"),
        Cell::from("Tab|Navigate Projects"),
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
        Widget::Projects => {
            layout.projects_content = layout.projects_content.clone().style(Style::default().fg(Color::Yellow));
            current_keybinds = projects_keybinds;
        }
        _ => {
            layout.main_content = layout.main_content.clone().style(Style::default().fg(Color::Yellow));
            // default keybinds
        }
    };

    let bottom_row_list = Row::new(current_keybinds).style(Style::default().fg(Color::Yellow));

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

}

//fn render_layout<'a, B>(layout: LayoutState<'a>, f: tui::Frame<'a, B>) -> Layout_State<'a> where B: Backend {layout}

