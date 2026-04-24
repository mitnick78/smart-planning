use crate::database::get_connection;
use crate::models::{PlanningEntry, StatEntry};
use rusqlite::{params, Result};

// GET BY MONTH
// get all planning entries for a given month, including label name and color
pub fn get_by_month(year: i32, month: u32) -> Result<Vec<PlanningEntry>> {
    let conn = get_connection()?;

    // Format : 2026-03%
    let pattern = format!("{:04}-{:02}%", year, month);

    let mut stmt = conn.prepare(
        "SELECT p.id, p.date, p.period, p.label_id,
                p.note, p.created_at, p.updated_at,
                l.name AS label_name,
                l.color_hex AS label_color
         FROM planning p
         JOIN labels l ON p.label_id = l.id
         WHERE p.date LIKE ?1
         ORDER BY p.date, p.period"
    )?;

    let entries = stmt.query_map(params![pattern], |row| {
        Ok(PlanningEntry {
            id: row.get(0)?,
            date: row.get(1)?,
            period: row.get(2)?,
            label_id: row.get(3)?,
            note: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
            label_name: row.get(7)?,
            label_color: row.get(8)?,
        })
    })?
    .collect::<Result<Vec<PlanningEntry>>>()?;

    Ok(entries)
}

// GET BY DATE AND PERIOD
// get a planning entry by date and period, including label name and color
pub fn get_by_date_and_period(
    date: &str,
    period: &str
) -> Result<Option<PlanningEntry>> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "SELECT p.id, p.date, p.period, p.label_id,
                p.note, p.created_at, p.updated_at,
                l.name AS label_name,
                l.color_hex AS label_color
         FROM planning p
         JOIN labels l ON p.label_id = l.id
         WHERE p.date = ?1 AND p.period = ?2"
    )?;

    let result = stmt.query_row(
        params![date, period],
        |row| Ok(PlanningEntry {
            id: row.get(0)?,
            date: row.get(1)?,
            period: row.get(2)?,
            label_id: row.get(3)?,
            note: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
            label_name: row.get(7)?,
            label_color: row.get(8)?,
        })
    );

    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// INSERT ou UPDATE selon si le créneau existe déjà
// if entry with same date and period exists, update it, otherwise insert a new one
// UPSERT = UPDATE + INSERT

pub fn upsert(entry: &PlanningEntry) -> Result<i64> {
    let conn = get_connection()?;

    conn.execute(
        "INSERT INTO planning (date, period, label_id, note, updated_at)
         VALUES (?1, ?2, ?3, ?4, CURRENT_TIMESTAMP)
         ON CONFLICT(date, period)
         DO UPDATE SET
             label_id   = excluded.label_id,
             note       = excluded.note,
             updated_at = CURRENT_TIMESTAMP",
        params![entry.date, entry.period, entry.label_id, entry.note],
    )?;

    // get the id of the inserted/updated entry
    let id: i64 = conn.query_row(
        "SELECT id FROM planning WHERE date = ?1 AND period = ?2",
        params![entry.date, entry.period],
        |row| row.get(0),
    )?;

    Ok(id)
}

// DELETE BY DATE AND PERIOD
// delete a planning entry by date and period
pub fn delete_by_date_and_period(date: &str, period: &str) -> Result<bool> {
    let conn = get_connection()?;

    let rows = conn.execute(
        "DELETE FROM planning WHERE date = ?1 AND period = ?2",
        params![date, period],
    )?;

    Ok(rows > 0)
}

// STATS 
// get stats for a month : count of entries by label and period
pub fn get_stats_by_month(year: i32, month: u32) -> Result<Vec<StatEntry>> {
    let conn = get_connection()?;
    let pattern = format!("{:04}-{:02}%", year, month);
    println!("Pattern : {}", pattern);
    println!("BDD : {:?}", crate::database::get_db_path());

    let mut stmt = conn.prepare(
        "SELECT l.name, l.color_hex, p.period, COUNT(*) as count
         FROM planning p
         JOIN labels l ON p.label_id = l.id
         WHERE p.date LIKE ?1
         GROUP BY l.name, l.color_hex, p.period
         ORDER BY l.name, p.period"
    )?;

    let stats = stmt.query_map(params![pattern], |row| {
        Ok(StatEntry {
            label_name: row.get(0)?,
            label_color: row.get(1)?,
            period: row.get(2)?,
            count: row.get(3)?,
        })
    })?
    .collect::<Result<Vec<StatEntry>>>()?;

    Ok(stats)
}

// TESTS 
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::initialize;
    use crate::label_repository;
    use crate::models::Label;

    fn setup() {
        initialize().expect("Init BDD échouée");
    }

    #[test]
    fn test_upsert_and_get() {
        setup();

        //  create a label
        let label = Label::new("Test", "#3B82F6");
        let label_id = label_repository::create(&label)
            .expect("Création label échouée");

        // create a planning entry
        let entry = PlanningEntry::new("2026-03-15", "matin", label_id);
        let id = upsert(&entry).expect("Upsert échoué");
        assert!(id > 0);

        // verify  why the entry can be retrieved by date and period
        let found = get_by_date_and_period("2026-03-15", "matin")
            .expect("Erreur")
            .expect("Entrée non trouvée");

        assert_eq!(found.date, "2026-03-15");
        assert_eq!(found.period, "matin");
    }

    #[test]
    fn test_get_by_month() {
        setup();
        let entries = get_by_month(2026, 3).expect("Erreur get_by_month");
        let _ = entries;
    }

    #[test]
    fn test_stats() {
        setup();
        let stats = get_stats_by_month(2026, 3).expect("Erreur stats");
        let _ = stats;
    }
}