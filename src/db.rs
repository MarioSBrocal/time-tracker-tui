use chrono::NaiveDateTime;
use rusqlite::{Connection, params};

use crate::app::{AppError, AppResult, Period};

pub fn setup_db(db_path: &str) -> AppResult<Connection> {
    let conn = Connection::open(db_path)?;

    // Save the dates in the database as text in ISO 8601 format (YYYY-MM-DD HH:MM:SS).
    conn.execute(
        "CREATE TABLE IF NOT EXISTS periods (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            enter_time TEXT NOT NULL,
            exit_time TEXT
    )",
        [],
    )?;

    Ok(conn)
}

pub fn register_period(
    conn: &Connection,
    enter_time: NaiveDateTime,
    exit_time: NaiveDateTime,
) -> AppResult<()> {
    conn.execute(
        "INSERT INTO periods (enter_time, exit_time) VALUES (?1, ?2)",
        params![enter_time.to_string(), exit_time.to_string()],
    )?;

    Ok(())
}

pub fn delete_period(conn: &Connection, id: i32) -> AppResult<()> {
    conn.execute("DELETE FROM periods WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn update_period(
    conn: &Connection,
    id: i32,
    enter_time: NaiveDateTime,
    exit_time: NaiveDateTime,
) -> AppResult<()> {
    conn.execute(
        "UPDATE periods SET enter_time = ?1, exit_time = ?2 WHERE id = ?3",
        params![enter_time.to_string(), exit_time.to_string(), id],
    )?;

    Ok(())
}

pub fn calculate_hours_range(
    conn: &Connection,
    from: NaiveDateTime,
    to: NaiveDateTime,
) -> AppResult<f64> {
    let mut stmt = conn.prepare(
        "SELECT enter_time, exit_time FROM periods 
        WHERE enter_time >= ?1 AND exit_time <= ?2",
    )?;

    let periods_iter = stmt.query_map(params![from.to_string(), to.to_string()], |row| {
        let enter_time_str: String = row.get(0)?;
        let exit_time_str: String = row.get(1)?;
        Ok((enter_time_str, exit_time_str))
    })?;

    let mut total_seconds = 0;

    for period in periods_iter {
        let (enter_time_str, exit_time_str) = period?;

        // Convert the enter_time and exit_time strings back to NaiveDateTime
        let enter_time = NaiveDateTime::parse_from_str(&enter_time_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| AppError::DateTimeParse(e.to_string()))?;
        let exit_time = NaiveDateTime::parse_from_str(&exit_time_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| AppError::DateTimeParse(e.to_string()))?;

        // Calculate the difference in seconds between enter_time and exit_time
        let duration = exit_time.signed_duration_since(enter_time);
        total_seconds += duration.num_seconds();
    }

    Ok(total_seconds as f64 / 3600.0) // Convert seconds to hours
}

pub fn fetch_month_periods(conn: &Connection, year: i32, month: u32) -> AppResult<Vec<Period>> {
    let mut stmt = conn.prepare(
        "SELECT id, enter_time, exit_time FROM periods 
         WHERE strftime('%Y', enter_time) = ?1 AND strftime('%m', enter_time) = ?2
         ORDER BY enter_time DESC",
    )?;

    let year_str = year.to_string();
    let month_str = format!("{:02}", month); // Ensure month is two digits

    let periods_iter = stmt.query_map(params![year_str, month_str], |row| {
        let id: i32 = row.get(0)?;
        let enter_time_str: String = row.get(1)?;
        let exit_time_str: String = row.get(2)?;

        Ok((id, enter_time_str, exit_time_str))
    })?;

    let mut periods = Vec::new();
    for period in periods_iter {
        let (id, enter_time_str, exit_time_str) = period?;

        // Convert the enter_time and exit_time strings back to NaiveDateTime
        let enter_time = NaiveDateTime::parse_from_str(&enter_time_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| AppError::DateTimeParse(e.to_string()))?;
        let exit_time = NaiveDateTime::parse_from_str(&exit_time_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| AppError::DateTimeParse(e.to_string()))?;

        periods.push(Period {
            id,
            enter_time,
            exit_time,
        });
    }

    Ok(periods)
}
