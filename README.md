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
