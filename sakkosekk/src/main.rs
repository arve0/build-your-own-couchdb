use rusqlite::{named_params, Connection};

fn main() {
    let db = Connection::open("database.sqlite").expect("Unable to open 'database.sqlite'.");

    db.execute_batch(
        "create table documents (
            id text primary key not null,
            revision integer not null,
            hash blob not null,
            data text not null
        )",
    )
    .expect("Unable to create documents table.");

    db.execute_named(
        "insert into documents (id, revision, hash, data)
        values (:id, :revision, :hash, :data)",
        named_params!(
            ":id": "asdf",
            ":revision": 0,
            ":hash": vec![0u8],
            ":data": r#"{ "a": 1, "b": 123 }"#
        ),
    )
    .expect("Unable to insert document.");

    let data: String = db
        .query_row_named(
            "select data from documents where id=:id",
            named_params!(":id": "asdf"),
            |row| row.get(0),
        )
        .expect("Unable to get document with id 'asdf'");

    println!("data: {}", data);
}
