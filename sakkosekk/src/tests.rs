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

    fn with<F>(filename: &str, test: F)
    where
        F: Fn(Connection) -> (),
    {
        let db = get_db_create_if_missing(filename);
        test(db);
        remove_file(filename).unwrap();
    }
}
