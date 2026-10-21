### Why RUST

I was trying to figure that out myself.  I have years of experience with Java backend, .NET COM backend, JavaScript nodeJS backend experience.  So why would anyone want to venture into RUST?

| Area | Rust | Java | JavaScript (Node.js) |
| --- | --- | --- | --- |
| **Object orientation** | Uses structs and enums to model data, and traits to define shared behavior. It has methods and encapsulation, but no classes or inheritance; composition and traits are the common alternatives. | Class-based object-oriented programming with classes, interfaces, inheritance, and encapsulation. | Prototype-based object orientation with ES6 `class` syntax syntactic sugar; relies heavily on object composition, closures, and first-class functions. |
| **Type checking** | Statically and strongly typed, with extensive checks at compile time. Ownership and borrowing rules help prevent data races and memory errors without a garbage collector. | Statically and strongly typed. The JVM garbage collector manages memory, while some errors can still arise at runtime, such as null dereferences or failed casts. | Dynamically and weakly typed at runtime with V8 garbage collection; requires TypeScript or JSDoc for static compile-time checking. |
| **Developer tools** | Cargo provides an integrated workflow for builds, dependencies, tests, and publishing. `rustfmt` and Clippy support consistent formatting and linting. | A mature ecosystem with Maven and Gradle for builds and dependencies, plus broad IDE support and JVM profiling and debugging tools. | Package managers (`npm`, `yarn`, `pnpm`) and script runners; formatting and linting via ESLint and Prettier; native debugging through Chrome DevTools and VS Code. |
| **Developer community** | A growing global community, with official learning resources and a strong presence in systems programming, embedded software, and performance-sensitive services. | A large, long-established global community, with extensive documentation, libraries, frameworks, and enterprise experience. | Massive, ubiquitous full-stack community backed by the npm ecosystem (the world's largest package registry) and widespread adoption across web APIs and microservices. |

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
