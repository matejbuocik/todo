mod config;
mod db;

fn main() {
    let config = config::Config::get().unwrap();

    let database = db::connect(&config.db_path).unwrap();
    // let mut note = db::Note::new("Wassup".to_string());
    // db::add_note(database, &mut note).unwrap();
    for note in db::get_notes(database).unwrap() {
        println!("{:?}", note.unwrap());
    }
}
