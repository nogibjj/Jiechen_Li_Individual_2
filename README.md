# rust-new-project-template

# Build Rust Environment

1. Use ``rustup`` to install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Restart the shell and run

```bash
source $HOME/.cargo/env
```

3. Create a Rust project

```bash
cargo new rust_sqlite_project
```

# Add SQLite Dependency

1. Use ``rusqlite`` crate in Rust

```bash
cargo new rust_sqlite_project
```

2. Then build it

```bash
cargo build
```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* Check for latest Rust [dependency](https://crates.io/search?q=main)
