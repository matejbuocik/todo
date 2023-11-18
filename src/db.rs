use rusqlite::{Connection, Result};
use std::time::SystemTime;

#[derive(Debug)]
pub struct Note {
    pub id: i64,
    /// seconds since the UNIX epoch
    pub date: u64,
    pub text: String,
    pub done: bool,
}

impl Note {
    pub fn new(text: String) -> Note {
        let id = 0;
        let date = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Note {
            id,
            date,
            text,
            done: false,
        }
    }
}

pub fn connect(path: &str) -> Result<Connection> {
    let db = Connection::open(path)?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id  INTEGER PRIMARY KEY,
            date INTEGER NOT NULL,
            text TEXT NOT NULL,
            done INTEGER NOT NULL
        )",
        (),
    )?;

    Ok(db)
}

pub fn add_note(db: &Connection, note: &mut Note) -> Result<()> {
    db.execute(
        "INSERT INTO notes (date, text, done) VALUES (?1, ?2, ?3)",
        (&note.date, &note.text, &note.done),
    )?;

    note.id = db.last_insert_rowid();
    Ok(())
}

pub fn get_notes(db: &Connection) -> Result<Vec<Result<Note>>> {
    let mut stmt = db.prepare("SELECT id, date, text, done FROM notes")?;
    let notes = stmt.query_map((), |row| {
        Ok(Note {
            id: row.get(0)?,
            date: row.get(1)?,
            text: row.get(2)?,
            done: row.get(3)?,
        })
    })?;

    Ok(notes.collect())
}

pub fn get_note(db: &Connection, note_id: &u64) -> Result<Note> {
    db.query_row(
        "SELECT id, date, text, done FROM notes WHERE id = (?1)",
        [note_id],
        |row| {
            Ok(Note {
                id: row.get(0)?,
                date: row.get(1)?,
                text: row.get(2)?,
                done: row.get(3)?,
            })
        },
    )
}

pub fn set_done(db: &Connection, note_id: &u64) -> Result<()> {
    db.execute("UPDATE notes SET done = true WHERE id = (?1)", [note_id])?;

    Ok(())
}

pub fn remove_note(db: &Connection, note_id: &u64) -> Result<()> {
    db.execute("DELETE FROM notes WHERE id = (?1)", [note_id])?;

    Ok(())
}

pub fn update_text(db: &Connection, note_id: &u64, new_text: String) -> Result<()> {
    db.execute(
        "UPDATE notes SET text = (?1) WHERE id = (?2)",
        (new_text, note_id),
    )?;

    Ok(())
}
