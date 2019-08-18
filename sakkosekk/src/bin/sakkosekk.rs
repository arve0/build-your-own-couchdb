use sakkosekk::get_db_create_if_missing;

fn main() {
    let db = get_db_create_if_missing("database.sqlite");
    dbg!(db);
}
