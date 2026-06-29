use rusqlite::Connection;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Input/Output error in terminal: {0}")]
    IO(#[from] std::io::Error),

    #[error("Date parsing error: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(PartialEq)]
pub enum UiMode {
    Menu,
    Writing,
}

pub struct Config {
    pub db_path: String,
}

pub struct AppState {
    pub config: Config,
    pub db: Connection,
    pub ui_mode: UiMode,
    pub input_buffer: String,
    pub should_quit: bool,
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
            should_quit: false,
        }
    }

    /// Quit the application by setting the `should_quit` flag to true.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
