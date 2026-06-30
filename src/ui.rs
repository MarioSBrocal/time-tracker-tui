use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{AppState, UiMode};

pub fn render(f: &mut Frame, app: &AppState) {
    // Divide the terminal into three sections: header, center, and footer.
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3), // Header section fixed of 3 lines
                Constraint::Min(1),    // Center section takes the remaining space
                Constraint::Length(3), // Footer section fixed of 3 lines
            ]
            .as_ref(),
        )
        .split(f.area());

    // Header section
    let title = Paragraph::new("Time Tracker Application")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);

    // Center section
    match app.ui_mode {
        UiMode::Menu => {
            let menu_text = "Welcome to the registration.\n\nPress 'e' to enter a new manual period.\nPress 'q' to exit the application.\nPress 'c' to calculate hours between two dates.";
            let content = Paragraph::new(menu_text)
                .block(Block::default().borders(Borders::ALL).title(" Main Menu "));
            f.render_widget(content, chunks[1]);
        }
        UiMode::WritingEnterTime => {
            // Show what the user is typing from app.input_buffer
            let input_text = format!(
                "Enter the date (YYYY-MM-DD HH:MM):\n\n> {}",
                app.input_buffer
            );
            let content = Paragraph::new(input_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Enter Time ")
                        .border_style(Style::default().fg(Color::LightGreen)),
                )
                .style(Style::default().fg(Color::White));
            f.render_widget(content, chunks[1]);
        }
        UiMode::WritingExitTime => {
            // Show what the user is typing from app.input_buffer
            let input_text = format!(
                "Enter the exit date (YYYY-MM-DD HH:MM):\n\n> {}",
                app.input_buffer
            );
            let content = Paragraph::new(input_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Exit Time ")
                        .border_style(Style::default().fg(Color::LightRed)),
                )
                .style(Style::default().fg(Color::White));
            f.render_widget(content, chunks[1]);
        }
        UiMode::CalculatingStart => {
            let input_text = format!(
                "Enter the start date for calculation (YYYY-MM-DD):\n\n> {}",
                app.input_buffer
            );
            let content = Paragraph::new(input_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Calculate Hours - Start Date ")
                        .border_style(Style::default().fg(Color::LightGreen)),
                )
                .style(Style::default().fg(Color::White));
            f.render_widget(content, chunks[1]);
        }
        UiMode::CalculatingEnd => {
            let input_text = format!(
                "Enter the end date for calculation (YYYY-MM-DD):\n\n> {}",
                app.input_buffer
            );
            let content = Paragraph::new(input_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Calculate Hours - End Date ")
                        .border_style(Style::default().fg(Color::LightRed)),
                )
                .style(Style::default().fg(Color::White));
            f.render_widget(content, chunks[1]);
        }
        UiMode::CalculatingShowResult => {
            let result_text = match app.calculation_result {
                Some(hours) => {
                    let total_minutes = (hours * 60.0).round() as u64;
                    let total_hours = total_minutes / 60;
                    let remaining_minutes = total_minutes % 60;

                    format!(
                        "Total hours calculated between the specified dates: {:.2} hours\nExact time: {:02}:{:02}",
                        hours, total_hours, remaining_minutes
                    )
                }
                None => "No calculation result available.".to_string(),
            };
            let content = Paragraph::new(result_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Calculation Result ")
                        .border_style(Style::default().fg(Color::Magenta)),
                )
                .style(Style::default().fg(Color::White));
            f.render_widget(content, chunks[1]);
        }
    }

    // Footer section
    let msg_pie = match app.ui_mode {
        UiMode::Menu => " 'q' Exit | 'e' Write period | 'c' Calculate hours ",
        UiMode::WritingEnterTime => " 'Esc' Cancel and return to Menu | 'Enter' Save ",
        UiMode::WritingExitTime => " 'Esc' Cancel and return to Menu | 'Enter' Save ",
        UiMode::CalculatingStart => " 'Esc' Cancel and return to Menu | 'Enter' Save ",
        UiMode::CalculatingEnd => " 'Esc' Cancel and return to Menu | 'Enter' Save ",
        UiMode::CalculatingShowResult => " 'Esc' Return to Menu | 'Enter' Return to Menu ",
    };
    let pie = Paragraph::new(msg_pie)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(pie, chunks[2]);

    if let Some(error) = &app.error_message {
        let popup_area = centered_area(50, 20, f.area());

        let popup = Paragraph::new(error.as_str())
            .block(
                Block::default()
                    .title(" ERROR ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red)),
            )
            .style(Style::default().fg(Color::White));
        f.render_widget(popup, popup_area);
    }
}

fn centered_area(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
