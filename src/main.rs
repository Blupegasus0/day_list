use std::error::Error;
use std::io;

use tui::Terminal;
use tui::backend::{Backend, CrosstermBackend};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent,
    KeyModifiers
};

use day_list::view::render;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Init
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    // running loop
    render::run(&mut terminal).await.expect("Daylist encountered an error...");

    // Cleanup
    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

