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
            get_document(0)
                .insert(&db)
                .expect("Unable to insert document.");
        });
    }

    #[test]
    fn double_insertion_should_fail() {
        with("double_insertion.sqlite", |db| {
            get_document(0)
                .insert(&db)
                .expect("Unable to insert document.");
            let second_insert_result = get_document(0).insert(&db);
            assert!(second_insert_result.is_err());
        });
    }

    #[test]
    fn insert_multiple_revisions() {
        with("insert_multiple_revisions.sqlite", |db| {
            get_document(0)
                .insert(&db)
                .expect("Unable to insert document.");
            get_document(1)
                .insert(&db)
                .expect("Unable to insert document.");
        });
    }

    #[test]
    fn get_by_missing_id_should_give_no_results() {
        with("get_by_id_missing.sqlite", |db| {
            let documents = Document::get_by_id("asdf", &db).expect("Unable to get documents.");
            assert!(documents.is_empty());
        });
    }

    fn get_by_id() {
        with("get_by_id.sqlite", |db| {
            get_document(0)
                .insert(&db)
                .expect("Unable to insert document.");
            get_document(1)
                .insert(&db)
                .expect("Unable to insert document.");

            let documents_from_db = Document::get_by_id("asdf", &db);

            assert!(documents_from_db == Ok(vec![get_document(0), get_document(1)]));
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

    fn get_document(revision: i64) -> Document {
        Document {
            id: String::from("asdf"),
            revision: revision,
            hash: vec![0u8],
            data: String::from(r#"{ "a": 1, "b": 123 }"#),
        }
    }
}
