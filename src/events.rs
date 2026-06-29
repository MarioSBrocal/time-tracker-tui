use std::io::Stdout;

use crate::{
    app::{AppResult, AppState, UiMode},
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

        // Read keyboard events and handle them based on the current UI mode
        if let Event::Key(key) = event::read()? {
            // Ignore release events and only handle key presses
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            // Handle key events based on the current UI mode
            match app.ui_mode {
                UiMode::Menu => match key.code {
                    KeyCode::Char('q') => {
                        app.quit(); // Change should_quit to true to exit the application
                    }
                    KeyCode::Char('e') => {
                        // Change the UI mode to Writing and clear the input buffer for new period entry
                        app.ui_mode = UiMode::Writing;
                        app.input_buffer.clear();
                    }
                    _ => {}
                },
                UiMode::Writing => match key.code {
                    KeyCode::Esc => {
                        // Cancel the writing mode and return to the main menu
                        app.ui_mode = UiMode::Menu;
                        app.input_buffer.clear();
                    }
                    KeyCode::Enter => {
                        // TODO: Implement the logic to process the input buffer and save the new period to the database
                        app.ui_mode = UiMode::Menu;
                    }
                    KeyCode::Char(c) => {
                        // Append typed characters to the input buffer
                        app.input_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        // Remove the last character from the input buffer if it exists
                        app.input_buffer.pop();
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(())
}
