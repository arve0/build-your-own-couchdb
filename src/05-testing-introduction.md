# Testing introduction
Cargo provides a test runner, `cargo test` which runs functions annotated with `#[test]`. Lets create a test which checks that `get_db_create_if_missing` does not crash if called twice, *src/tests.rs*:

```rust
#[cfg(test)]
mod database {
    use crate::*;
    use std::fs::remove_file;

    #[test]
    fn creating_database_twice_should_not_fail() {
        get_db_create_if_missing("test.sqlite");
        get_db_create_if_missing("test.sqlite");
        remove_file("test.sqlite").unwrap();
    }
}
```

Here, `#[cfg(test)]` tells Rust that the module should only compile when compiling tests. `mod database` is a grouping for the database tests.

Running the test:
```sh
~/g/b/sakkosekk $ cargo test
   Compiling sakkosekk v0.1.0 (/Users/arve/git/build-your-own-couchdb/sakkosekk)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running target/debug/deps/sakkosekk-5d572632b2e9bfcc

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

*Running 0 tests*? We need to add `mod tests` to *src/main.rs*, such that the `tests` module is found:

```rust
use rusqlite::{named_params, Connection, Error as SqliteError, Row};
use std::path::Path;

mod tests;

fn main() {
    let db = get_db_create_if_missing("database.sqlite");
...
```

Really run the test:
```sh
~/g/b/sakkosekk $ cargo test
   Compiling sakkosekk v0.1.0 (/Users/arve/git/build-your-own-couchdb/sakkosekk)
    Finished dev [unoptimized + debuginfo] target(s) in 1.85s
     Running target/debug/deps/sakkosekk-5d572632b2e9bfcc

running 1 test
test tests::database::creating_database_twice_should_not_fail ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Insertion
Naivly adding an insertion test:

```rust
#[cfg(test)]
mod database {
    use crate::*;
    use std::fs::remove_file;

    const TEST_DB_FILENAME: &str = "test.sqlite";

    #[test]
    fn creating_database_twice_should_not_fail() {
        get_db_create_if_missing(TEST_DB_FILENAME);
        get_db_create_if_missing(TEST_DB_FILENAME);
        clean_up();
    }

    #[test]
    fn insertion() {
        let db = get_db_create_if_missing(TEST_DB_FILENAME);

        let document = Document {
            id: String::from("asdf"),
            revision: 0,
            hash: vec![0u8],
            data: String::from(r#"{ "a": 1, "b": 123 }"#),
        };

        document.insert(&db).expect("Unable to insert document.");

        clean_up();
    }

    fn clean_up() {
        remove_file(TEST_DB_FILENAME).unwrap();
    }
}
```

This will fail:
```sh
~/g/b/sakkosekk (master|✚2…) $ cargo test
   Compiling sakkosekk v0.1.0 (/Users/arve/git/build-your-own-couchdb/sakkosekk)
    Finished dev [unoptimized + debuginfo] target(s) in 1.14s
     Running target/debug/deps/sakkosekk-5d572632b2e9bfcc

running 2 tests
test tests::database::insertion ... ok
test tests::database::creating_database_twice_should_not_fail ... FAILED

failures:

---- tests::database::creating_database_twice_should_not_fail stdout ----
thread 'tests::database::creating_database_twice_should_not_fail' panicked at 'Unable to create documents table.: SqliteFailure(Error { code: Unknown, extended_code: 1 }, Some("table documents already exists"))', src/libcore/result.rs:999:5
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    tests::database::creating_database_twice_should_not_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--bin sakkosekk'
```

The test `creating_database_twice_should_not_fail` fails, as tests runs in parallel. We'll use a helper function `with` that:

- takes a filename and a test function,
- creates a database connection,
- runs the given test function with the created database connection,
- and removes the database file.

Using the function should look like:
```rust
with("filename.sqlite", |db| {
    // use db connection
});
// with will clean up / remove the database
```

`with` implementation:
```rust
    fn with<F>(filename: &str, test: F)
    where
        F: Fn(Connection) -> (),
    {
        let db = get_db_create_if_missing(filename);
        test(db);
        remove_file(filename).unwrap();
    }
```

**Note:** An alternative `with` is to implement [`Drop`] for our own `Connection`-type.

The tests rewritten to use `with`:
```rust
    #[test]
    fn creating_database_twice_should_not_fail() {
        with("creating_twice.sqlite", |_| {
            get_db_create_if_missing("creating_twice.sqlite");
        });
    }

    #[test]
    fn insertion() {
        with("insertion.sqlite", |db| {
            let document = Document {
                id: String::from("asdf"),
                revision: 0,
                hash: vec![0u8],
                data: String::from(r#"{ "a": 1, "b": 123 }"#),
            };

            document.insert(&db).expect("Unable to insert document.");
        });
    }
```

Note that the tests use different filenames for the database.

Running the tests does not fail:
```sh
~/g/b/sakkosekk $ cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running target/debug/deps/sakkosekk-5d572632b2e9bfcc

running 2 tests
test tests::database::creating_database_twice_should_not_fail ... ok
test tests::database::insertion ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Next up: Tests that are expected to fail, like inserting a document with the same identifier twice.

[`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
