# Build your own modern CouchDB with Rust
Read book at https://arve0.github.io/build-your-own-couchdb/.

## Writing
Edit book inside [src](./src/) and code inside [sakkosekk](./sakkosekk/).

Build HTML with mdbook:
```sh
cargo install mdbook
mdbook build
```

## Development
Make sure you have [Rust installed](https://rustup.rs). Source is in [sakkosekk](./sakkosekk/):

```sh
cd sakkosekk
```

### Tests
```sh
cargo test
```

### Benchmarks
See benchmarks in [benched](./sakkosekk/benches/).
```sh
cargo bench
```

### Running
```sh
cargo run
```

### Building
```sh
cargo build --release
```

Binary in `./target/release/`:
```sh
./target/release/sakkosekk
```
