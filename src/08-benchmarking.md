# Benchmarking
As you might have noticed, we did not add an index when removing the primary key. This may affect performance when looking up entries on the `id` column.

The [criterion] crate gives some nice tools for statistics-driven benchmarking.

## Splitting into library and binary
Criterion has some [known limitations]. One limitation is benchmarking a binary crate, which is not possible. To overcome the limitation, we split our project into a library and a binary.

Move *main.rs* to *lib.rs*:

```sh
mv src/main.rs src/lib.rs
```

Remove the `main`-function and make `get_db_create_if_missing`, `Document`, `Document`-fields and some of the `Document`-methods public:
```rust
use rusqlite::{named_params, Connection, Error as SqliteError, Row};
use std::path::Path;

mod tests;

pub fn get_db_create_if_missing(filename: &str) -> Connection {
    ...
}

#[derive(PartialEq)]
pub struct Document {
    pub id: String,
    pub revision: i64,
    pub hash: Vec<u8>,
    pub data: String,
}

impl Document {
    ...

    pub fn insert(&self, db: &Connection) -> Result<usize, SqliteError> {
        ...
    }

    pub fn get_by_id(id: &str, db: &Connection) -> Result<Vec<Self>, SqliteError> {
        ...
    }

    ...
}
```

`...` is omitted code that have not changed.

Create a new minimal `main` function in *src/bin/sakkosekk.rs*:
```rust
use sakkosekk::get_db_create_if_missing;

fn main() {
    let db = get_db_create_if_missing("database.sqlite");
    dbg!(db);
}
```

Check that both `cargo run` and `cargo test` works:
```sh
~/g/b/sakkosekk $ cargo run
   Compiling sakkosekk v0.1.0 (/Users/arve/git/build-your-own-couchdb/sakkosekk)
    Finished dev [unoptimized + debuginfo] target(s) in 11.00s
     Running `target/debug/sakkosekk`
[src/bin/sakkosekk.rs:5] db = Connection {
    path: Some(
        "database.sqlite",
    ),
}
~/g/b/sakkosekk $ cargo test
   Compiling sakkosekk v0.1.0 (/Users/arve/git/build-your-own-couchdb/sakkosekk)
    Finished dev [unoptimized + debuginfo] target(s) in 2.38s
     Running target/debug/deps/sakkosekk-c97044fdfdcd0074

running 6 tests
test tests::database::creating_database_twice_should_not_fail ... ok
test tests::database::get_by_missing_id_should_give_no_results ... ok
test tests::database::double_insertion_should_fail ... ok
test tests::database::get_by_id ... ok
test tests::database::insertion ... ok
test tests::database::insert_multiple_revisions ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/sakkosekk-7673159c8931cebe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests sakkosekk

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Creating our first benchmark
Add criterion as a development dependency and define a benchmark in *Cargo.toml:*

```toml
[dev-dependencies]
criterion = "0.2"

[[bench]]
name = "database"
harness = false
```

`name = "database"` must match the filename, make the benchmark as the file *benches/database.rs*:
```rust
use criterion::{criterion_group, criterion_main, Criterion};
use sakkosekk::{get_db_create_if_missing, Document};
use rusqlite::{Connection};
use std::fs::remove_file;

criterion_group!(benches, benchmark);
criterion_main!(benches);

fn benchmark(c: &mut Criterion) {
    c.bench_function("get documents by id", |b| {
        let db = BenchDatabase::new("bench_get_documents_by_id.sqlite");
        let doc = Document::get_by_id("bench100", &db.connection).unwrap();
        assert!(doc.len() == 1);

        b.iter(|| Document::get_by_id("bench100", &db.connection))
    });
}

struct BenchDatabase {
    filename: &'static str,
    connection: Connection,
}

impl BenchDatabase {
    fn new(filename: &'static str) -> Self {
        remove_file(filename).unwrap_or(());
        let mut connection = get_db_create_if_missing(filename);
        let transaction = connection.transaction().unwrap();

        for i in 0..1000 {
            let document = Document {
                id: format!("bench{}", i),
                revision: 0,
                hash: vec![],
                data: format!(r#"{{ "number": {} }}"#, i)
            };
            document.insert(&transaction).unwrap();
        }

        transaction.commit().unwrap();

        Self { filename, connection }
    }
}
```

`c.bench_function` takes a setup function. Latest statement in `bench_function` is `b.iter`. Criterion will call the function given to `b.iter` in a tight loop and collect timing statistics.

Run it with `cargo bench`:
```
get documents by id     time:   [25.567 us 25.989 us 26.485 us]
```

Lets try adding an index on `id` and see results of benchmark again.

Adding index index on `id` in *lib.rs*:
```rust
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
```

Run `cargo bench` to review benchmark:
```
get documents by id     time:   [26.169 us 26.702 us 27.417 us]
                        change: [-0.4154% +2.4533% +5.1353%] (p = 0.10 > 0.05)
                        No change in performance detected.
```

No change in performance detected. Lets investigate with the SQLite REPL:

```sql
~/g/b/sakkosekk $ sqlite3 bench_get_documents_by_id.sqlite
SQLite version 3.24.0 2018-06-04 14:10:15
Enter ".help" for usage hints.
sqlite> EXPLAIN QUERY PLAN select * from documents where id='bench100';
QUERY PLAN
`--SEARCH TABLE documents USING INDEX documents_id_idx (id=?)
sqlite> drop index documents_id_idx;
sqlite> EXPLAIN QUERY PLAN select * from documents where id='bench100';
QUERY PLAN
`--SEARCH TABLE documents USING INDEX sqlite_autoindex_documents_1 (id=?)
sqlite>
```

SQLite actually creates an *autoindex*, so we see no performance gain when creating the index. A search in the [SQLite query optimizer documentation] reveals some details:

> In SQLite version 3.8.0 (2013-08-26) and later, an SQLITE_WARNING_AUTOINDEX message is sent to the error log every time a statement is prepared that uses an automatic index. Application developers can and should use these warnings to identify the need for new persistent indices in the schema.

Following the recomendation, I'll leave the definition of the index.

## Enabling write-ahead logging (WAL)
A common performance recommandation for SQLite is [write-ahead logging].

Add Enabling WAL-mode:
```rust
    ...

    if !exists {
        // create schema
        Document::create_table(&db).expect("Unable to create documents table.");

        enable_write_ahead_logging(&db);
    }

    db
}

fn enable_write_ahead_logging(db: &Connection) {
    // PRAGMA journal_mode=wal;
    let result: String = db
        .pragma_update_and_check(None, "journal_mode", &"wal", |row| row.get(0))
        .unwrap();
    assert!("wal" == &result);
}
```

As before, we use `cargo bench` to assess performance:
```
get documents by id     time:   [13.426 us 13.583 us 13.759 us]
                        change: [-51.227% -49.928% -48.677%] (p = 0.00 < 0.05)
                        Performance has improved.
```

2x speedup! 14 microseconds gives a decent upper limit for fetches, 1s / 14us ≈ 70k fetches per second on this hardware (2013 macbook air with 250 GB SSD). I'm happy with that, so let's continue with adding a HTTP-layer in front of the disk persistence.

**Next up:** Defining an API.


[criterion]: https://github.com/bheisler/criterion.rs
[known limitations]: https://bheisler.github.io/criterion.rs/book/user_guide/known_limitations.html
[SQLite query optimizer documentation]: https://www.sqlite.org/optoverview.html
[write-ahead logging]: https://www.sqlite.org/wal.html
