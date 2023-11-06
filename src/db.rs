use rusqlite::{Connection, Result};
use std::time::SystemTime;

const DB_PATH: &str = "./database"; // TODO config file

#[derive(Debug)]
pub struct Note {
    id: i32,
    /// seconds since the UNIX epoch
    date: u64,
    text: String,
}

impl Note {
    pub fn new(date: SystemTime, text: String) -> Note {
        let id = 0;
        let date = date
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

pub fn add_note(db: Connection, note: Note) -> Result<()> {
    db.execute(
        "INSERT INTO notes (date, text) VALUES (?1, ?2)",
        (&note.date, &note.text),
    )?;

    Ok(())
}
