# Naive implementation
We'll start by creating a naiv implementation in Rust. By naiv I mean:

- no persistence
- API-usage only, no communication

Each entry is defined as

```rs
Entry {
    index: i64,
    id: UUID,
    prev: Option<String>,
    hash: String,
    data: String,
}
```

The structure of an entry



This will be the basis
