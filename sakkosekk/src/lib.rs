use rusqlite::{named_params, Connection, Error as SqliteError, Row};
use std::path::Path;

mod tests;

pub fn get_db_create_if_missing(filename: &str) -> Connection {
    // Connection::open will create file if missing, check before.
    let exists = Path::new(filename).exists();

    let db = Connection::open(filename)
        .unwrap_or_else(|_| panic!(format!("Unable to open database file {}", filename)));

    if !exists {
        // create schema
        Document::create_table(&db).expect("Unable to create documents table.");
    }

    db
}

#[derive(PartialEq)]
pub struct Document {
    pub id: String,
    pub revision: i64,
    pub hash: Vec<u8>,
    pub data: String,
}

impl Document {
    fn create_table(db: &Connection) -> Result<(), SqliteError> {
        db.execute_batch(
            "create table documents (
                id text not null,
                revision integer not null,
                hash blob not null,
                data text not null,
                unique(id, revision, hash)
            );
            create index documents_id_idx on documents(id);
            ",
        )
    }

    pub fn insert(&self, db: &Connection) -> Result<usize, SqliteError> {
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

    pub fn get_by_id(id: &str, db: &Connection) -> Result<Vec<Self>, SqliteError> {
        db.prepare("select id, revision, hash, data from documents where id=:id")?
            .query_map_named(named_params!(":id": id), Document::row_mapper)?
            .collect()
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
