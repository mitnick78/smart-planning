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
#[cfg(not(test))]
pub fn get_connection() -> Result<Connection> {
    let db_path = get_db_path();
    let conn = Connection::open(&db_path)?;
    Ok(conn)
}

// En test : jamais la base réelle de l'utilisateur.
// Mesure transitoire (lot 0 de AUDIT.md) en attendant que les repositories
// reçoivent une `&Connection` injectée.
#[cfg(test)]
pub fn get_connection() -> Result<Connection> {
    test_db::open()
}

/// Base SQLite en mémoire, propre à chaque thread de test.
///
/// Les repositories ouvrent une nouvelle connexion à chaque appel : une base
/// `:memory:` classique serait vide à chaque fois. On utilise donc une base
/// mémoire nommée en cache partagé, unique par thread (cargo test lance chaque
/// test dans son propre thread), et maintenue en vie par une connexion gardée
/// dans le thread-local.
#[cfg(test)]
mod test_db {
    use rusqlite::{Connection, Result};
    use std::cell::RefCell;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    thread_local! {
        static URI: String = format!(
            "file:smart_planning_test_{}?mode=memory&cache=shared",
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        );
        static KEEP_ALIVE: RefCell<Option<Connection>> = const { RefCell::new(None) };
    }

    pub fn open() -> Result<Connection> {
        let uri = URI.with(String::clone);
        KEEP_ALIVE.with(|keep| -> Result<()> {
            if keep.borrow().is_none() {
                *keep.borrow_mut() = Some(Connection::open(&uri)?);
            }
            Ok(())
        })?;
        Connection::open(&uri)
    }
}

//initialize database
pub fn initialize() -> Result<()> {
    let conn = get_connection()?;
    conn.execute_batch(
        "
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
        
        -- Table vacances scolaires (cache local)
        CREATE TABLE IF NOT EXISTS school_holidays (
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            date  DATE NOT NULL,
            zone  TEXT NOT NULL CHECK(zone IN ('A', 'B', 'C')),
            name  TEXT NOT NULL,
            UNIQUE(date, zone)
        );

        -- Table settings (zone sélectionnée, etc.)
        CREATE TABLE IF NOT EXISTS settings (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
    ",
    )?;

    Ok(())
}

// test manually
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_initialization() {
        let result = initialize();
        assert!(
            result.is_ok(),
            "Database initialization failed: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_get_connection() {
        let conn = get_connection();
        assert!(conn.is_ok(), "La connexion doit réussir");
    }

    #[test]
    fn test_connection_never_uses_real_database_file() {
        let conn = get_connection().expect("connexion de test");
        // Une base en mémoire n'a pas de chemin de fichier (None ou "").
        assert!(
            matches!(conn.path(), None | Some("")),
            "les tests ne doivent jamais ouvrir un fichier : {:?}",
            conn.path()
        );
    }

    #[test]
    fn test_each_test_thread_gets_its_own_database() {
        initialize().expect("init");
        let conn = get_connection().expect("connexion");
        conn.execute("INSERT INTO settings (key, value) VALUES ('k', 'v')", [])
            .expect("insert");

        let other_thread_sees_it = std::thread::spawn(|| {
            initialize().expect("init");
            let conn = get_connection().expect("connexion");
            conn.query_row("SELECT COUNT(*) FROM settings", [], |r| r.get::<_, i64>(0))
                .expect("count")
        })
        .join()
        .expect("thread");

        assert_eq!(other_thread_sees_it, 0);
    }
}
