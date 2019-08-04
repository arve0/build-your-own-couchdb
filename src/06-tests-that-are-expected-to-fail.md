# Tests that are expected to fail
We could make our tests panic and annotate tests with `#[should_panic]`, but as our methods return `Result`, the [`is_err`] method tells us that the correct method call failed.

## Insertion failure
A double insertion of the same document should fail:
```rust
    #[test]
    fn double_insertion_should_fail() {
        with("double_insertion.sqlite", |db| {
            let document = Document {
                id: String::from("asdf"),
                revision: 0,
                hash: vec![0u8],
                data: String::from(r#"{ "a": 1, "b": 123 }"#),
            };

            document.insert(&db).expect("Unable to insert document.");
            let second_insert_result = document.insert(&db);
            assert!(second_insert_result.is_err());
        });
    }
```

Actually, this test is wrong. In CouchDB a document can have many revisions, making `id`, `revision` and `hash` unique.

## Insertion failure only when same revision
This fails as expected:
```rust
    #[test]
    fn insert_multiple_revisions() {
        with("insert_multiple_revisions.sqlite", |db| {
            let insert = |revision: i64| {
                let document = Document {
                    id: String::from("asdf"),
                    revision: revision,
                    hash: vec![0u8],
                    data: String::from(r#"{ "a": 1, "b": 123 }"#),
                };

                document.insert(&db).expect("Unable to insert document.");
            };

            insert(0);
            insert(1);
        });
    }
```

We'll fix it by removing `primary key` from `id`:
```rust
    fn create_table(db: &Connection) -> Result<(), SqliteError> {
        db.execute_batch(
            "create table documents (
            id text not null,
            revision integer not null,
            hash blob not null,
            data text not null
        )",
        )
    }
```

Running tests again does not seem to have fixed `insert_multiple_revisions`, also `double_insertion_should_fail` fails now:
```
running 4 tests
test tests::database::creating_database_twice_should_not_fail ... ok
test tests::database::insert_multiple_revisions ... FAILED
test tests::database::double_insertion_should_fail ... FAILED
test tests::database::insertion ... ok
```

As the tests paniced, `remove_file` in `with` never runs. Fix it by removing file before opening:
```rust
    fn with<F>(filename: &str, test: F)
    where
        F: Fn(Connection) -> (),
    {
        remove_file(filename).unwrap_or(());
        let db = get_db_create_if_missing(filename);
        test(db);
        remove_file(filename).unwrap();
    }
```

`unwrap_or(())` ignores any errors in deleting the file.

Running tests again:
```
running 4 tests
test tests::database::creating_database_twice_should_not_fail ... ok
test tests::database::double_insertion_should_fail ... FAILED
test tests::database::insert_multiple_revisions ... ok
test tests::database::insertion ... ok
```

`insert_multiple_revisions` is OK, but `double_insertion_should_fail` is still failing. It fails as `primary key` constraint was removed.

Adding a unique constraint should fix id:
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
            ",
        )
    }
```

## Get non-existent document
Getting a missing document should fail:
```rust
    #[test]
    fn get_by_missing_id_should_fail() {
        with("get_by_id_missing.sqlite", |db| {
            let result = Document::get_by_id("asdf", &db);
            assert!(result.is_err());
        });
    }
```

Make sure all tests pass:
```
running 5 tests
test tests::database::get_by_missing_id_should_fail ... ok
test tests::database::creating_database_twice_should_not_fail ... ok
test tests::database::double_insertion_should_fail ... ok
test tests::database::insert_multiple_revisions ... ok
test tests::database::insertion ... ok
```

Next up: Get by id should return all revisions of document.

[`is_err`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.is_err
