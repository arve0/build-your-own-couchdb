# Persistent storage
Lets start with designing the disk storage format. The non-goal *Writing low level code* stears off designing a file format and using direct file access. A good alternative is [SQLite]. Lets set it up first.

## Bootstrapping the project
I'll call the project *sakkosekk*, which is Norwegian for bean bag chair.

Bootstrapping with [Cargo]:
```sh
~/g/build-your-own-couchdb $ cargo init sakkosekk
     Created binary (application) package
~/g/build-your-own-couchdb $ cd sakkosekk/
~/g/b/sakkosekk $ cargo run
   Compiling sakkosekk v0.1.0 (/Users/arve/git/build-your-own-couchdb/sakkosekk)
    Finished dev [unoptimized + debuginfo] target(s) in 10.24s
     Running `target/debug/sakkosekk`
Hello, world!
```

## Adding SQLite
Bindings for SQLite is available through the [rusqlite] crate:

```sh
~/g/b/sakkosekk $ echo '[dependencies]' >> Cargo.toml
~/g/b/sakkosekk $ echo 'rusqlite = { version = "0.20", features = ["bundled"] }' >> Cargo.toml
```

The *bundled* feature is enabled for hassle free sqlite3 linking.

## Database schema
Documents in the database will have the columns:

- indentifier,
- revision,
- hash and
- document data.

Open database:
```rust
use rusqlite::{named_params, Connection};

fn main() {
    let db = Connection::open("database.sqlite").expect("Unable to open 'database.sqlite'.");
```

Creating table:
```rust
    db.execute_batch(
        "create table documents (
            id text primary key not null,
            revision integer not null,
            hash blob not null,
            data text not null
        )",
    )
    .expect("Unable to create documents table.");
```

Inserting document:
```rust
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
```

Reading document by the identifier:
```rust
    let data: String = db
        .query_row_named(
            "select data from documents where id=:id",
            named_params!(":id": "asdf"),
            |row| row.get(0),
        )
        .expect("Unable to get document with id 'asdf'");

    println!("data: {}", data);
}
```

Result:
```sh
~/g/b/sakkosekk $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/sakkosekk`
data: { "a": 1, "b": 123 }
```

Next up is creating abstractions around creating, inserting and reading from the database.

[SQLite]: https://sqlite.org/
[Cargo]: https://doc.rust-lang.org/cargo/
[rusqlite]: https://github.com/jgallagher/rusqlite/
