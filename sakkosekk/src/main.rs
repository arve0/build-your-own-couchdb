use rusqlite::{named_params, Connection, Error as SqliteError, Row};

fn main() {
    let db = Connection::open("database.sqlite").expect("Unable to open 'database.sqlite'.");

    Document::create_table(&db).expect("Unable to create documents table.");

    let document = Document {
        id: String::from("asdf"),
        revision: 0,
        hash: vec![0u8],
        data: String::from(r#"{ "a": 1, "b": 123 }"#),
    };

    document.insert(&db).expect("Unable to insert document.");

    let document_from_db = Document::get_by_id("asdf", &db)
        .expect("Unable to get document with id 'asdf'");

    println!("data: {}", &document_from_db.data);
}

struct Document {
    id: String,
    revision: i64,
    hash: Vec<u8>,
    data: String,
}

impl Document {
    fn create_table(db: &Connection) -> Result<(), SqliteError> {
        db.execute_batch(
            "create table documents (
            id text primary key not null,
            revision integer not null,
            hash blob not null,
            data text not null
        )",
        )
    }

    fn insert(&self, db: &Connection) -> Result<usize, SqliteError> {
        db.execute_named(
            "insert into documents (id, revision, hash, data)
        values (:id, :revision, :hash, :data)",
            named_params!(
                ":id": &self.id,
                ":revision": self.revision,
                ":hash": &self.hash,
                ":data": &self.data,
            ),
        )
    }

    fn get_by_id(id: &str, db: &Connection) -> Result<Self, SqliteError> {
        db.query_row_named(
            "select id, revision, hash, data from documents where id=:id",
            named_params!(":id": id),
            Document::row_mapper,
        )
    }

    fn row_mapper(row: &Row) -> Result<Self, SqliteError> {
        Ok(Self {
            id: row.get(0)?,
            revision: row.get(1)?,
            hash: row.get(2)?,
            data: row.get(3)?,
        })
    }
}
