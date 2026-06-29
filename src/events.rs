use std::io::Stdout;

use crate::{
    app::{AppResult, AppState},
    ui,
};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{Terminal, backend::CrosstermBackend};

pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut AppState,
) -> AppResult<()> {
    loop {
        // Draw the UI based on the current application state
        terminal.draw(|f| ui::render(f, app))?;

        // Check if we should quit
        if app.should_quit {
            break;
        }

        // Wait and read a keyboard event
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => app.quit(), // Change should_quit to true to exit the application
                _ => {}
            }
        }
    }

    Ok(())
}
