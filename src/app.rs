use chrono::NaiveDateTime;
use rusqlite::Connection;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Input/Output error in terminal: {0}")]
    IO(#[from] std::io::Error),

    #[error("Date parsing error: {0}")]
    DateParse(String),

    #[error("DateTime parsing error: {0}")]
    DateTimeParse(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
impl AppError {
    pub fn user_message(&self) -> String {
        match self {
            AppError::Database(_) => {
                "Database error occurred. Please check the database.".to_string()
            }
            AppError::IO(_) => {
                "Input/Output error occurred. Please check the terminal.".to_string()
            }
            AppError::DateParse(_) => "Invalid date format. Please use YYYY-MM-DD.".to_string(),
            AppError::DateTimeParse(_) => {
                "Invalid date-time format. Please use YYYY-MM-DD HH:MM.".to_string()
            }
            AppError::InvalidState(msg) => format!("Invalid state: {}", msg),
            AppError::Unexpected(msg) => format!("Unexpected error: {}", msg),
        }
    }
}

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(PartialEq)]
pub enum UiMode {
    Menu,
    WritingEnterTime,
    WritingExitTime,
    CalculatingStart,
    CalculatingEnd,
    CalculatingShowResult,
}

pub struct Config {
    pub db_path: String,
}

pub struct AppState {
    pub config: Config,
    pub db: Connection,
    pub ui_mode: UiMode,
    pub input_buffer: String,
    pub temporal_enter_time: Option<NaiveDateTime>,
    pub temporal_start_date: Option<NaiveDateTime>,
    pub calculation_result: Option<f64>,
    pub should_quit: bool,
    pub error_message: Option<String>,
}
impl AppState {
    pub fn new(connection: Connection) -> Self {
        Self {
            config: Config {
                db_path: String::from("time_tracker.db"),
            },
            db: connection,
            ui_mode: UiMode::Menu,
            input_buffer: String::new(),
            temporal_enter_time: None,
            temporal_start_date: None,
            calculation_result: None,
            should_quit: false,
            error_message: None,
        }
    }

    /// Quit the application by setting the `should_quit` flag to true.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
