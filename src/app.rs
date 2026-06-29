use rusqlite::Connection;

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(PartialEq)]
pub enum UiMode {
    Menu,
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
