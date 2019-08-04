# Create database only when missing
Currently, our application crashes when the database exists. Helper function that checks for existence and only create tables upon an empty database:

```rust
fn get_db_create_if_missing(filename: &str) -> Connection {
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
```

`Path` import:

```rust
use std::path::Path;
```

`main` function simplifies to:
```rust
fn main() {
    let db = get_db_create_if_missing("database.sqlite");

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
```

The application still crashes, but with a different error:
```sh
~/g/b/sakkosekk $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/sakkosekk`
thread 'main' panicked at 'Unable to insert document.: SqliteFailure(Error { code: ConstraintViolation, extended_code: 1555 }, Some("UNIQUE constraint failed: documents.id"))', src/libcore/result.rs:999:5
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

Next up: Refactor the main function into tests.
