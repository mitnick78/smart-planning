use rusqlite::{Connection, Result};
use std::path::PathBuf;

//path database
pub fn get_db_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("tauri_app");
    std::fs::create_dir_all(&path).expect("Failed to create database directory");
    path.push("smart_planning.db");
    path
}

//connect to database
pub fn get_connection() -> Result<Connection> {
    let db_path = get_db_path();
    let conn = Connection::open(&db_path)?;
    Ok(conn)
}

//initialize database
pub fn initialize() -> Result<()> {
    let conn = get_connection()?;
    conn.execute_batch("
        -- Table labels
        CREATE TABLE IF NOT EXISTS labels (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            name       TEXT    NOT NULL,
            color_hex  TEXT    NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
 
        -- Table planning
        CREATE TABLE IF NOT EXISTS planning (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            date       DATE    NOT NULL,
            period     TEXT    NOT NULL
                CHECK(period IN ('matin', 'après-midi', 'soir', 'journée')),
            label_id   INTEGER NOT NULL,
            note       TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (label_id) REFERENCES labels(id)
        );
 
        -- Index unique : un seul label par (date + période)
        CREATE UNIQUE INDEX IF NOT EXISTS idx_planning_date_period
            ON planning(date, period);
 
        -- Index de performance sur la date
        CREATE INDEX IF NOT EXISTS idx_planning_date
            ON planning(date);
    ")?;

    Ok(())
}

// test manually
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_initialization() {
        let result = initialize();
        assert!(result.is_ok(), "Database initialization failed: {:?}", result.err());
    }

    #[test]
    fn test_get_connection() {
        let conn = get_connection();
        assert!(conn.is_ok(), "La connexion doit réussir");
    }
}