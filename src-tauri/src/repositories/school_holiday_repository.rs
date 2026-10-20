use crate::database::get_connection;
use crate::models::SchoolHoliday;
use rusqlite::Result;

pub fn upsert_many(holidays: &[SchoolHoliday]) -> Result<()> {
    let conn = get_connection()?;
    for h in holidays {
        conn.execute(
            "INSERT OR REPLACE INTO school_holidays (date, zone, name)
             VALUES (?1, ?2, ?3)",
            rusqlite::params![h.date, h.zone, h.name],
        )?;
    }
    Ok(())
}

pub fn get_by_month(year: i32, month: u32, zone: &str) -> Result<Vec<SchoolHoliday>> {
    let conn = get_connection()?;
    let pattern = format!("{:04}-{:02}%", year, month);
    let mut stmt = conn.prepare(
        "SELECT date, zone, name FROM school_holidays
         WHERE date LIKE ?1 AND zone = ?2
         ORDER BY date"
    )?;
    let rows = stmt.query_map(rusqlite::params![pattern, zone], |row| {
        Ok(SchoolHoliday {
            date: row.get(0)?,
            zone: row.get(1)?,
            name: row.get(2)?,
        })
    })?;
    rows.collect()
}

pub fn get_setting(key: &str) -> Result<Option<String>> {
    let conn = get_connection()?;
    let result = conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        rusqlite::params![key],
        |row| row.get(0),
    );
    match result {
        Ok(v) => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn set_setting(key: &str, value: &str) -> Result<()> {
    let conn = get_connection()?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        rusqlite::params![key, value],
    )?;
    Ok(())
}