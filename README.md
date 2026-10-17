### Essential Commands & Best Practices

| Command | Purpose | When to Use |
| --- | --- | --- |
| `cargo check` | Verifies code compiles without producing an executable. | Fast feedback loop while writing code. |
| `cargo run` | Builds and runs the binary in debug mode. | Daily development and testing. |
| `cargo build --release` | Compiles optimized code in `target/release/`. | Benchmarking or production builds. |
| `cargo fmt` | Formats code according to official style guidelines. | Before committing. |
| `cargo clippy` | Lints for idioms, performance, and common bugs. | Code reviews and static analysis. |


### Learning Path

### Recommended Learning Path

1. **["The Book"](https://doc.rust-lang.org/book/) (*The Rust Programming Language*)**  
   The official, free introductory guide. Read Chapters 1 through 10 to grasp the fundamentals, especially **ownership, borrowing, lifetimes, and enums/pattern matching**.

   To read it offline in your browser, run:

   ```bash
   rustup doc --book
   ```

2. **[Rustlings](https://github.com/rust-lang/rustlings)**  
   An interactive terminal exercise course with small, broken code snippets for you to fix. Install it with:

   ```bash
   curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash
   ```

3. **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)**  
   Practical, bite-sized examples covering Rust and its standard library. View it offline with `rustup doc --rbe`.


### Tests

 1. Concurrency test
  ```
    cargo test --bin concurrency
  ```

   to display the thread id and the vector of integers
  ``` 
  cargo test --bin concurrency -- --nocapture
  ```

### RESTful Catalog Service

Run the service with:

```bash
cargo run --bin restful_service
```

The service listens on `http://127.0.0.1:3000` and creates `catalog.db` in the
current directory with sample products on its first run.

- `GET /api/catalog` returns the catalog as JSON.
- `POST /api/catalog` adds a product. Send JSON with `name`, `description`, and
  `price_cents` fields:

  ```bash
  curl -X POST http://127.0.0.1:3000/api/catalog \
    -H 'Content-Type: application/json' \
    -d '{"name":"Notebook","description":"Ruled paper notebook","price_cents":599}'
  ```

Run the service tests with `cargo test --bin restful_service`.