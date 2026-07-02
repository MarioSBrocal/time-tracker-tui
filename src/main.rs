use std::io;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    app::{AppResult, AppState},
    db::setup_db,
    events::run_app,
};

mod app;
mod db;
mod events;
mod ui;

fn main() -> AppResult<()> {
    // Initialize the database connection
    let conn = setup_db()?;

    // Create the global application state
    let mut app_state = AppState::new(conn);

    // Configure the terminal for TUI rendering
    enable_raw_mode()?; // Disables the standard input
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?; // Switches to a clean alternate screen
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the main application loop
    let res = run_app(&mut terminal, &mut app_state);

    // Restore the terminal to its original state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    // Handle any errors that occurred during the application run
    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}
