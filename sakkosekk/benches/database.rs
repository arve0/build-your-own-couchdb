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
