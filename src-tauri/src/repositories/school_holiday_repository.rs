use crate::database::get_connection;
use crate::models::SchoolHoliday;
use rusqlite::Result;

/// Enregistre les jours de vacances en une seule transaction : tout ou rien.
pub fn upsert_many(holidays: &[SchoolHoliday]) -> Result<()> {
    let mut conn = get_connection()?;
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO school_holidays (date, zone, name)
             VALUES (?1, ?2, ?3)",
        )?;
        for h in holidays {
            stmt.execute(rusqlite::params![h.date, h.zone, h.name])?;
        }
    }
    tx.commit()
}

pub fn get_by_month(year: i32, month: u32, zone: &str) -> Result<Vec<SchoolHoliday>> {
    let conn = get_connection()?;
    let pattern = format!("{:04}-{:02}%", year, month);
    let mut stmt = conn.prepare(
        "SELECT date, zone, name FROM school_holidays
         WHERE date LIKE ?1 AND zone = ?2
         ORDER BY date",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::initialize;

    fn holiday(date: &str, zone: &str) -> SchoolHoliday {
        SchoolHoliday {
            date: date.to_string(),
            zone: zone.to_string(),
            name: "Vacances d'hiver".to_string(),
        }
    }

    #[test]
    fn upsert_many_is_all_or_nothing() {
        initialize().expect("init");
        // La zone "Z" viole la contrainte CHECK : rien ne doit être écrit.
        let batch = [holiday("2026-02-14", "A"), holiday("2026-02-15", "Z")];

        assert!(upsert_many(&batch).is_err());
        assert!(get_by_month(2026, 2, "A").expect("lecture").is_empty());
    }

    #[test]
    fn upsert_many_inserts_and_replaces() {
        initialize().expect("init");
        upsert_many(&[holiday("2026-02-14", "A"), holiday("2026-02-15", "A")]).expect("insert");
        upsert_many(&[holiday("2026-02-14", "A")]).expect("replace");

        assert_eq!(get_by_month(2026, 2, "A").expect("lecture").len(), 2);
    }
}
