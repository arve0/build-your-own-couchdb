#[cfg(test)]
mod database {
    use crate::*;
    use rusqlite::Connection;
    use std::fs::remove_file;

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

    #[test]
    fn get_by_missing_id_should_fail() {
        with("get_by_id_missing.sqlite", |db| {
            let result = Document::get_by_id("asdf", &db);
            assert!(result.is_err());
        });
    }

    fn with<F>(filename: &str, test: F)
    where
        F: Fn(Connection) -> (),
    {
        remove_file(filename).unwrap_or(());
        let db = get_db_create_if_missing(filename);
        test(db);
        remove_file(filename).unwrap();
    }
}
