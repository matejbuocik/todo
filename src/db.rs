use rusqlite::{Connection, Result};
use std::time::SystemTime;

const DB_PATH: &str = "./database"; // TODO config file

#[derive(Debug)]
pub struct Note {
    id: i64,
    /// seconds since the UNIX epoch
    date: u64,
    text: String,
}

impl Note {
    pub fn new(text: String) -> Note {
        let id = 0;
        let date = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Note { id, date, text }
    }
}

pub fn connect() -> Result<Connection> {
    let db = Connection::open(DB_PATH)?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id  INTEGER PRIMARY KEY,
            date INTEGER NOT NULL,
            text TEXT NOT NULL
        )",
        (),
    )?;

    Ok(db)
}

pub fn add_note(db: Connection, note: &mut Note) -> Result<()> {
    db.execute(
        "INSERT INTO notes (date, text) VALUES (?1, ?2)",
        (&note.date, &note.text),
    )?;

    note.id = db.last_insert_rowid();
    Ok(())
}

pub fn get_notes(db: Connection) -> Result<Vec<Result<Note>>> {
    let mut stmt = db.prepare("SELECT id, date, text from notes")?;
    let notes = stmt.query_map((), |row| {
        Ok(Note {
            id: row.get(0)?,
            date: row.get(1)?,
            text: row.get(2)?,
        })
    })?;

    Ok(notes.collect())
}
