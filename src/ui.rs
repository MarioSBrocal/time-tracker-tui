use ratatui::{
    Frame,
    layout::{Constraint, Layout},
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
            let menu_text = "Welcome to the registration.\n\nPress 'e' to enter a new manual period.\nPress 'q' to exit the application.";
            let content = Paragraph::new(menu_text)
                .block(Block::default().borders(Borders::ALL).title(" Main Menu "));
            f.render_widget(content, chunks[1]);
        }
        UiMode::Writing => {
            // Show what the user is typing from app.input_buffer
            let input_text = format!(
                "Enter the date (YYYY-MM-DD HH:MM:SS):\n\n> {}",
                app.input_buffer
            );
            let content = Paragraph::new(input_text)
                .block(Block::default().borders(Borders::ALL).title(" New Period "))
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(content, chunks[1]);
        }
    }

    // Footer section
    let msg_pie = match app.ui_mode {
        UiMode::Menu => " 'q' Exit | 'e' Write period ",
        UiMode::Writing => " 'Esc' Cancel and return to Menu | 'Enter' Save ",
    };
    let pie = Paragraph::new(msg_pie)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(pie, chunks[2]);
}
