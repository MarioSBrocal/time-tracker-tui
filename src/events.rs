use std::io::Stdout;

use crate::{
    app::{AppResult, AppState, UiMode},
    db::register_period,
    ui,
};
use chrono::NaiveDateTime;
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
                        // Change the UI mode to WritingEnterTime and clear the input buffer for new period entry
                        app.ui_mode = UiMode::WritingEnterTime;
                        app.input_buffer.clear();
                    }
                    _ => {}
                },
                UiMode::WritingEnterTime => match key.code {
                    KeyCode::Esc => {
                        // Cancel the writing mode and return to the main menu
                        app.ui_mode = UiMode::Menu;
                        app.input_buffer.clear();
                    }
                    KeyCode::Enter => {
                        // Save the entered time in a temporary variable and switch to WritingExitTime mode
                        app.temporal_enter_time_input = Some(app.input_buffer.clone());
                        app.input_buffer.clear();
                        app.ui_mode = UiMode::WritingExitTime;
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
                UiMode::WritingExitTime => match key.code {
                    KeyCode::Esc => {
                        // Cancel the writing mode and return to the main menu
                        app.ui_mode = UiMode::Menu;
                        app.input_buffer.clear();
                    }
                    KeyCode::Enter => {
                        if let Some(enter_time_str) = &app.temporal_enter_time_input {
                            let exit_time_str = &app.input_buffer;

                            // Try to parse the entered times into NaiveDateTime
                            let parse_enter_time =
                                NaiveDateTime::parse_from_str(enter_time_str, "%Y-%m-%d %H:%M:%S");
                            let parse_exit_time =
                                NaiveDateTime::parse_from_str(exit_time_str, "%Y-%m-%d %H:%M:%S");

                            if let (Ok(enter_time), Ok(exit_time)) =
                                (parse_enter_time, parse_exit_time)
                            {
                                // Register the period in the database
                                let _ = register_period(&app.db, enter_time, exit_time);
                            } else {
                                // TODO: Handle parsing errors (e.g., show an error message to the user)
                            }
                        }

                        // Clear the input buffer and return to the main menu
                        app.input_buffer.clear();
                        app.temporal_enter_time_input = None;
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
