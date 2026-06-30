use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use rusqlite::Connection;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Input/Output error in terminal: {0}")]
    IO(#[from] std::io::Error),

    #[error("Year parsing error: {0}")]
    YearParse(String),

    #[error("Month parsing error: {0}")]
    MonthParse(String),

    #[error("Day parsing error: {0}")]
    DayParse(String),

    #[error("Hour parsing error: {0}")]
    HourParse(String),

    #[error("Minute parsing error: {0}")]
    MinuteParse(String),

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
            AppError::YearParse(msg) => format!("Invalid year format.\n{}", msg),
            AppError::MonthParse(msg) => format!("Invalid month format.\n{}", msg),
            AppError::DayParse(msg) => format!("Invalid day format.\n{}", msg),
            AppError::HourParse(msg) => format!("Invalid hour format.\n{}", msg),
            AppError::MinuteParse(msg) => format!("Invalid minute format.\n{}", msg),
            AppError::DateParse(msg) => format!("Invalid date format.\n{}", msg),
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

#[derive(Default, Clone)]
pub struct DateTimeAssistant {
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
    pub minute: String,
    pub step: u8, // 0: year, 1: month, 2: day, 3: hour, 4: minute
}
impl DateTimeAssistant {
    pub fn reset(&mut self) {
        self.year.clear();
        self.month.clear();
        self.day.clear();
        self.hour.clear();
        self.minute.clear();
        self.step = 0;
    }

    pub fn iso_format(&self) -> String {
        format!(
            "{}-{}-{} {}:{}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }

    pub fn validate_year(&self, input: &str) -> Result<(), AppError> {
        let year: i32 = input
            .parse()
            .map_err(|_| AppError::YearParse("The year must be a valid number.".to_string()))?;

        let current_year = Local::now().year();
        if (2000..=current_year).contains(&year) {
            Ok(())
        } else {
            Err(AppError::YearParse(format!(
                "The year must be between 2000 and {}.",
                current_year
            )))
        }
    }

    pub fn validate_month(&self, input: &str) -> Result<(), AppError> {
        let month: u32 = input
            .parse()
            .map_err(|_| AppError::MonthParse("The month must be a valid number.".to_string()))?;

        if (1..=12).contains(&month) {
            Ok(())
        } else {
            Err(AppError::MonthParse(
                "The month must be between 1 and 12.".to_string(),
            ))
        }
    }

    pub fn validate_day(&self, input: &str) -> Result<(), AppError> {
        let day: u32 = input
            .parse()
            .map_err(|_| AppError::DayParse("The day must be a valid number.".to_string()))?;

        let year: i32 = self
            .year
            .parse()
            .map_err(|_| AppError::YearParse("The year must be a valid number.".to_string()))?;
        let month: u32 = self
            .month
            .parse()
            .map_err(|_| AppError::MonthParse("The month must be a valid number.".to_string()))?;

        if NaiveDate::from_ymd_opt(year, month, day).is_some() {
            Ok(())
        } else {
            Err(AppError::DayParse(format!(
                "The day {} is not valid for the month {} and year {}.",
                day, month, year
            )))
        }
    }

    pub fn validate_hour(&self, input: &str) -> Result<(), AppError> {
        let hour: u32 = input
            .parse()
            .map_err(|_| AppError::HourParse("The hour must be a valid number.".to_string()))?;

        if (0..=23).contains(&hour) {
            Ok(())
        } else {
            Err(AppError::HourParse(
                "The hour must be between 0 and 23.".to_string(),
            ))
        }
    }

    pub fn validate_minute(&self, input: &str) -> Result<(), AppError> {
        let minute: u32 = input
            .parse()
            .map_err(|_| AppError::MinuteParse("The minute must be a valid number.".to_string()))?;

        if (0..=59).contains(&minute) {
            Ok(())
        } else {
            Err(AppError::MinuteParse(
                "The minute must be between 0 and 59.".to_string(),
            ))
        }
    }
}

#[derive(Default, Clone)]
pub struct DateAssistant {
    pub year: String,
    pub month: String,
    pub day: String,
    pub step: u8, // 0: year, 1: month, 2: day
}
impl DateAssistant {
    pub fn reset(&mut self) {
        self.year.clear();
        self.month.clear();
        self.day.clear();
        self.step = 0;
    }

    pub fn iso_format(&self) -> String {
        format!("{}-{}-{}", self.year, self.month, self.day)
    }

    pub fn validate_year(&self, input: &str) -> Result<(), AppError> {
        let year: i32 = input
            .parse()
            .map_err(|_| AppError::YearParse("The year must be a valid number.".to_string()))?;

        let current_year = Local::now().year();
        if (2000..=current_year).contains(&year) {
            Ok(())
        } else {
            Err(AppError::YearParse(format!(
                "The year must be between 2000 and {}.",
                current_year
            )))
        }
    }

    pub fn validate_month(&self, input: &str) -> Result<(), AppError> {
        let month: u32 = input
            .parse()
            .map_err(|_| AppError::MonthParse("The month must be a valid number.".to_string()))?;

        if (1..=12).contains(&month) {
            Ok(())
        } else {
            Err(AppError::MonthParse(
                "The month must be between 1 and 12.".to_string(),
            ))
        }
    }

    pub fn validate_day(&self, input: &str) -> Result<(), AppError> {
        let day: u32 = input
            .parse()
            .map_err(|_| AppError::DayParse("The day must be a valid number.".to_string()))?;

        let year: i32 = self
            .year
            .parse()
            .map_err(|_| AppError::YearParse("The year must be a valid number.".to_string()))?;
        let month: u32 = self
            .month
            .parse()
            .map_err(|_| AppError::MonthParse("The month must be a valid number.".to_string()))?;

        if NaiveDate::from_ymd_opt(year, month, day).is_some() {
            Ok(())
        } else {
            Err(AppError::DayParse(format!(
                "The day {} is not valid for the month {} and year {}.",
                day, month, year
            )))
        }
    }
}

pub struct AppState {
    pub db: Connection,
    pub ui_mode: UiMode,
    pub input_buffer: String,
    pub date_time_assistant: DateTimeAssistant,
    pub date_assistant: DateAssistant,
    pub temporal_enter_time: Option<NaiveDateTime>,
    pub temporal_start_date: Option<NaiveDateTime>,
    pub calculation_result: Option<f64>,
    pub should_quit: bool,
    pub error_message: Option<String>,
}
impl AppState {
    pub fn new(connection: Connection) -> Self {
        Self {
            db: connection,
            ui_mode: UiMode::Menu,
            input_buffer: String::new(),
            date_time_assistant: DateTimeAssistant::default(),
            date_assistant: DateAssistant::default(),
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
