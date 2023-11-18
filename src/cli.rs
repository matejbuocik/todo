use crate::config;
use crate::db;
use crate::{Cli, Commands};
use ansi_term::Style;
use dialoguer::Editor;
use rusqlite::{Connection, Result};

pub fn run(args: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::get()?;
    let db = db::connect(&config.db_path)?;

    // TODO import from file, export to file
    // TODO fully interactive (tui)
    match &args.command {
        Some(Commands::Add { notes }) => add(&db, notes)?,
        Some(Commands::Edit { notes }) => edit(&db, notes)?,
        Some(Commands::Done { notes }) => done(&db, notes)?,
        Some(Commands::Remove { notes }) => remove(&db, notes)?,
        Some(Commands::List) | None => list(&db)?,
    };

    Ok(())
}

fn add(db: &Connection, notes: &Vec<String>) -> Result<()> {
    for text in notes {
        let mut note = db::Note::new(text.to_string());
        db::add_note(db, &mut note)?;
    }

    Ok(())
}

fn edit(db: &Connection, notes: &Vec<u64>) -> Result<(), Box<dyn std::error::Error>> {
    for note_id in notes {
        let note = db::get_note(db, note_id)?;
        if let Some(text) = Editor::new().edit(&note.text)? {
            db::update_text(db, note_id, text)?;
        }
    }
    Ok(())
}

fn done(db: &Connection, notes: &Vec<u64>) -> Result<()> {
    for note_id in notes {
        db::set_done(db, note_id)?;
    }

    Ok(())
}

fn remove(db: &Connection, notes: &Vec<u64>) -> Result<()> {
    for note_id in notes {
        db::remove_note(db, note_id)?;
    }

    Ok(())
}

fn list(db: &Connection) -> Result<()> {
    for note in db::get_notes(db)? {
        let note = note?;
        if note.done {
            println!(
                "{} {}",
                note.id,
                Style::new().strikethrough().paint(note.text)
            );
        } else {
            println!("{} {}", note.id, note.text);
        }
    }

    Ok(())
}
