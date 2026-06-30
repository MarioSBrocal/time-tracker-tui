use std::io::Stdout;

use crate::{
    app::{AppError, AppResult, AppState, UiMode},
    db::{self, register_period},
    ui,
};
use chrono::{NaiveDate, NaiveDateTime};
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

            if app.error_message.is_some() {
                match key.code {
                    KeyCode::Enter | KeyCode::Esc => app.error_message = None,
                    _ => {}
                }
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
                    KeyCode::Char('c') => {
                        // Change the UI mode to CalculatingStart and clear the input buffer for calculation
                        app.ui_mode = UiMode::CalculatingStart;
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
                        match NaiveDateTime::parse_from_str(&app.input_buffer, "%Y-%m-%d %H:%M") {
                            Ok(enter_time) => {
                                app.temporal_enter_time = Some(enter_time);
                                app.input_buffer.clear();
                                app.ui_mode = UiMode::WritingExitTime;
                            }
                            Err(e) => {
                                app.error_message =
                                    Some(AppError::DateTimeParse(e.to_string()).user_message());
                            }
                        }
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
                        if let Some(enter_time_str) = &app.temporal_enter_time {
                            match NaiveDateTime::parse_from_str(&app.input_buffer, "%Y-%m-%d %H:%M")
                            {
                                Ok(exit_time) => {
                                    // Save the period to the database
                                    match register_period(&app.db, *enter_time_str, exit_time) {
                                        Ok(_) => {
                                            // Clear the input buffer and return to the main menu
                                            app.input_buffer.clear();
                                            app.temporal_enter_time = None;
                                            app.ui_mode = UiMode::Menu;
                                        }
                                        Err(e) => {
                                            app.error_message = Some(e.user_message());
                                        }
                                    }
                                }
                                Err(e) => {
                                    app.error_message =
                                        Some(AppError::DateTimeParse(e.to_string()).user_message());
                                }
                            }
                        }
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
                UiMode::CalculatingStart => match key.code {
                    KeyCode::Esc => {
                        // Cancel the calculating mode and return to the main menu
                        app.ui_mode = UiMode::Menu;
                        app.input_buffer.clear();
                    }
                    KeyCode::Enter => {
                        // Calculate the start time based on the input buffer
                        match NaiveDate::parse_from_str(&app.input_buffer, "%Y-%m-%d") {
                            Ok(date) => {
                                app.temporal_start_date = Some(date.and_hms_opt(0, 0, 0).unwrap());
                                app.input_buffer.clear();
                                app.ui_mode = UiMode::CalculatingEnd;
                            }
                            Err(e) => {
                                app.error_message =
                                    Some(AppError::DateParse(e.to_string()).user_message());
                            }
                        }
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
                UiMode::CalculatingEnd => match key.code {
                    KeyCode::Esc => {
                        // Cancel the calculating mode and return to the main menu
                        app.ui_mode = UiMode::Menu;
                        app.input_buffer.clear();
                    }
                    KeyCode::Enter => {
                        if let Some(start_date) = &app.temporal_start_date {
                            // Calculate the end time based on the input buffer and the previously entered start date
                            match NaiveDate::parse_from_str(&app.input_buffer, "%Y-%m-%d") {
                                Ok(date) => {
                                    let end_date = date.and_hms_opt(23, 59, 59).unwrap();

                                    match db::calculate_hours_range(&app.db, *start_date, end_date)
                                    {
                                        Ok(hours) => {
                                            app.calculation_result = Some(hours);
                                            app.input_buffer.clear();
                                            app.ui_mode = UiMode::CalculatingShowResult;
                                        }
                                        Err(e) => {
                                            app.error_message = Some(e.user_message());
                                        }
                                    }
                                }
                                Err(e) => {
                                    app.error_message =
                                        Some(AppError::DateParse(e.to_string()).user_message());
                                }
                            }
                        } else {
                            app.error_message = Some(
                                AppError::InvalidState("Start date not set".to_string())
                                    .user_message(),
                            );
                        }
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
                UiMode::CalculatingShowResult => match key.code {
                    KeyCode::Esc | KeyCode::Enter => {
                        // Return to the main menu after showing the calculation result
                        app.ui_mode = UiMode::Menu;
                        app.input_buffer.clear();
                        app.calculation_result = None;
                        app.temporal_start_date = None;
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(())
}
