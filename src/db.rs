use rusqlite::Connection;

use crate::app::AppResult;

pub fn setup_db(db_path: &str) -> AppResult<Connection> {
    let conn = Connection::open(db_path)?;

    // Save the dates in the database as text in ISO 8601 format (YYYY-MM-DD HH:MM:SS).
    conn.execute(
        "CREATE TABLE IF NOT EXISTS time_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            enter_time TEXT NOT NULL,
            exit_time TEXT
    )",
        [],
    )?;

    Ok(conn)
}
