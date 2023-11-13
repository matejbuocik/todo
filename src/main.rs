mod db;

fn main() {
    let database = db::connect().unwrap();
    // let mut note = db::Note::new("Wassup".to_string());
    // db::add_note(database, &mut note).unwrap();
    for note in db::get_notes(database).unwrap() {
        println!("{:?}", note.unwrap());
    }
}
