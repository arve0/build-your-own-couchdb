# Getting all revisions of document
In [Persistent storage] we naively used `id` to look up a single document, but a document can have multiple revisions. In other words, `Document::get_by_id` should return a list of documents.

## Refactoring tests
First, the tests are repeating themself. Currently the tests `insertion`, `double_insertion_should_fail` and `insert_multiple_revisions` are all repeating declaration of a `document`.

Move document declaration into a function:
```rust
    fn get_document(revision: i64) -> Document {
        Document {
            id: String::from("asdf"),
            revision: revision,
            hash: vec![0u8],
            data: String::from(r#"{ "a": 1, "b": 123 }"#),
        }
    }
```

Tests refactored to use `get_document`:
```rust
    #[test]
    fn insertion() {
        with("insertion.sqlite", |db| {
            get_document(0).insert(&db).expect("Unable to insert document.");
        });
    }

    #[test]
    fn double_insertion_should_fail() {
        with("double_insertion.sqlite", |db| {
            get_document(0).insert(&db).expect("Unable to insert document.");
            let second_insert_result = get_document(0).insert(&db);
            assert!(second_insert_result.is_err());
        });
    }

    #[test]
    fn insert_multiple_revisions() {
        with("insert_multiple_revisions.sqlite", |db| {
            get_document(0).insert(&db).expect("Unable to insert document.");
            get_document(1).insert(&db).expect("Unable to insert document.");
        });
    }

```

## Get document by identifier test
Now the test for `Document::get_by_id`, using `get_document`. The test should check that `Document::get_by_id` returns all documents:

```rust
    #[test]
    fn get_by_id() {
        with("get_by_id.sqlite", |db| {
            get_document(0).insert(&db).expect("Unable to insert document.");
            get_document(1).insert(&db).expect("Unable to insert document.");

            let documents_from_db = Document::get_by_id("asdf", &db);

            assert!(documents_from_db == [get_document(0), get_document(1)]);
        });
    }
```

This fails compiling for two reasons. Lets takle number one first; `Document` does not implement the method `eq` from the [`PartialEq` trait].

Error message:
```
error[E0369]: binary operation `==` cannot be applied to type `std::result::Result<Document, rusqlite::error::Error>`
  --> src/tests.rs:53:39
   |
53 |             assert!(documents_from_db == Ok([get_document(0), get_document(1)]));
   |                     ----------------- ^^ -------------------------------------- std::result::Result<[Document; 2], _>
   |                     |
   |                     std::result::Result<Document, rusqlite::error::Error>
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `std::result::Result<Document, rusqlite::error::Error>`
```

As all fields in `Document` implements `PartialEq`, we can derive `PartialEq`:
```rust
#[derive(PartialEq)]
struct Document {
    id: String,
    revision: i64,
    hash: Vec<u8>,
    data: String,
}
```

Now, `cargo tests` yields the other error:
```
error[E0308]: mismatched types
  --> src/tests.rs:53:45
   |
53 |             assert!(documents_from_db == Ok([get_document(0), get_document(1)]));
   |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Document`, found array of 2 elements
   |
   = note: expected type `Document`
              found type `[Document; 2]`
```

`query_row_named` only returns a single row. [`Statement`] provides [`query_map_named`] that gives more then one row. [`Connection::prepare`] yields a `Statement`.

`Document::get_by_id` rewritten to use a `Statement`:
```rust
    fn get_by_id(id: &str, db: &Connection) -> Result<Vec<Self>, SqliteError> {
        db.prepare("select id, revision, hash, data from documents where id=:id")?
            .query_map_named(named_params!(":id": id), Document::row_mapper)?
            .collect()
    }
```

`cargo tests` gives a compilation error:
```
error[E0609]: no field `data` on type `std::vec::Vec<Document>`
  --> src/main.rs:21:44
   |
21 |     println!("data: {}", &document_from_db.data);
   |                                            ^^^^ unknown field
```

As `get_by_id` have a new return signature and `Vec` does not have a `data`-field. Fix it by removing the line.

Running tests shows that `get_by_missing_id` is failing:
```
running 5 tests
test tests::database::creating_database_twice_should_not_fail ... ok
test tests::database::get_by_missing_id_should_fail ... FAILED
test tests::database::double_insertion_should_fail ... ok
test tests::database::insert_multiple_revisions ... ok
test tests::database::insertion ... ok
```

Fix the test by having it expect an empty vector:
```rust
    #[test]
    fn get_by_missing_id_should_give_no_results() {
        with("get_by_id_missing.sqlite", |db| {
            let documents = Document::get_by_id("asdf", &db).expect("Unable to get documents.");
            assert!(documents.is_empty());
        });
    }
```

### Understanding `get_by_id`
A lot happens in `get_by_id`. Looking at the types can help in understanding the code.

0. `Result<T>` is here `rusqlite::Result<T>`, which is equivalent to `std::result::Result<T, rusqlite::Error>`.
1. `prepare` returns `Result<Statement>`.
2. The question mark `?` translates to *unwrap result or exit early with error*.
3. `?` unwraps `Statement`.
4. `query_map_named` returns `Result<MappedRows>`.
5. `?` unwraps `MappedRows`.
6. `MappedRows` implements `IntoIterator`, giving us `Iterator<Result<Document>>`.
7. `Iterator::collect` uses the return signature, `Result<Vec<Self>, SqliteError>`, and unwraps `Result<Document>` one by one, exiting early if one of them fails.

### Note on compiler warnings
You might have noticed compiler warnings like:

```
warning: unused variable: `db`
 --> src/main.rs:7:9
  |
7 |     let db = get_db_create_if_missing("database.sqlite");
  |         ^^ help: consider prefixing with an underscore: `_db`
  |
  = note: #[warn(unused_variables)] on by default
```

We'll fix these warnings later when using the database interface in our actual application.

**Next up:** Benchmarking

[Persistent storage]: ./02-persistent-storage.md
[`Statement`]: https://docs.rs/rusqlite/0.20.0/rusqlite/struct.Statement.html
[`query_map_named`]: https://docs.rs/rusqlite/0.20.0/rusqlite/struct.Statement.html#method.query_map_named
