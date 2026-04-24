use crate::database::get_connection;
use crate::models::Label;
use rusqlite::{params, Result};

// GET ALL
// get all labels, ordered by name, returns a vector of Label or an error if something went wrong
pub fn get_all() -> Result<Vec<Label>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, name, color_hex, created_at
         FROM labels
         ORDER BY name"
    )?;

    let labels = stmt.query_map([], |row| {
        Ok(Label {
            id: row.get(0)?,         
            name: row.get(1)?,       
            color_hex: row.get(2)?,  
            created_at: row.get(3)?, 
        })
    })?
    .collect::<Result<Vec<Label>>>()?;
 
    Ok(labels)
}

// GET BY ID
// get a label by id, returns None if not found, Some(label) if found, or an error if something went wrong

pub fn get_by_id(id: i64) -> Result<Option<Label>> {
    let conn = get_connection()?;
 
    let mut stmt = conn.prepare(
        "SELECT id, name, color_hex, created_at
         FROM labels WHERE id = ?"
    )?;
 
    // query_row = récupère UNE seule ligne
    let result = stmt.query_row(
        params![id],
        |row| Ok(Label {
            id: row.get(0)?,
            name: row.get(1)?,
            color_hex: row.get(2)?,
            created_at: row.get(3)?,
        })
    );
 
    match result {
        Ok(label) => Ok(Some(label)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e), 
    }
}

// CREATE 
/// Insère un nouveau label et retourne son id généré

pub fn create(label: &Label) -> Result<i64> {
    let conn = get_connection()?;
    conn.execute(
        "INSERT INTO labels (name, color_hex) VALUES (?1, ?2)",
        params![label.name, label.color_hex],
    )?;
    Ok(conn.last_insert_rowid())
}

// UPDATE
// update a label by id, returns number of rows updated (should be 1 if successful)

pub fn update(label: &Label) -> Result<usize> {
    let conn = get_connection()?;
    let rows_updated = conn.execute(
        "UPDATE labels SET name = ?1, color_hex = ?2 WHERE id = ?3",
        params![label.name, label.color_hex, label.id],
    )?;
    Ok(rows_updated)
}

//   DELETE 
// delete a label by id, if delete_entries is true, also delete associated planning entries
pub fn delete(id: i64, delete_entries: bool) -> Result<()> {
    let mut conn = get_connection()?;
    let tx = conn.transaction()?;
 
    if delete_entries {
        tx.execute(
            "DELETE FROM planning WHERE label_id = ?1",
            params![id],
        )?;
    }

    // delete label
    tx.execute(
        "DELETE FROM labels WHERE id = ?1",
        params![id],
    )?;
 
    // Commit — valide la transaction
    tx.commit()?;
 
    Ok(())
}

//  TESTS
#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::initialize;
 
    fn setup() {
        initialize().expect("Init BDD échouée");
    }
 
    #[test]
    fn test_create_and_get() {
        setup();
 
        let label = Label::new("Test École", "#3B82F6");
        let id = create(&label).expect("Création échouée");
 
        assert!(id > 0, "L'id doit être positif");
 
        let found = get_by_id(id)
            .expect("Erreur get_by_id")
            .expect("Label non trouvé");
 
        assert_eq!(found.name, "Test École");
        assert_eq!(found.color_hex, "#3B82F6");
    }
 
    #[test]
    fn test_get_all() {
        setup();
        let labels = get_all().expect("Erreur get_all");
        let _ = labels; 
    }
 
    #[test]
    fn test_delete() {
        setup();
        let label = Label::new("À supprimer", "#EF4444");
        let id = create(&label).expect("Création échouée");
 
        delete(id, false).expect("Suppression échouée");
 
        let found = get_by_id(id).expect("Erreur");
        assert!(found.is_none(), "Le label doit être supprimé");
    }
}