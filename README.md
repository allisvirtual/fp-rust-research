# fp-rust-research

Research project exploring how well Rust supports functional programming, built for the Functional Programming course at Fontys University of Applied Sciences.

## Requirements

- Rust **1.99.0** (pinned in `rust-toolchain.toml`, installed automatically by `rustup` on first use)
- Edition 2024
- No external crates

Install Rust via [rustup](https://rustup.rs). On Windows this also needs the Visual Studio C++ Build Tools for the linker.

## Running

```bash
cargo run     # run the prototypes
cargo test    # run the unit tests
cargo fmt     # format the code
```

## Layout

| Path | Contents |
|---|---|
| [`src/main.rs`](src/main.rs) | Entry point, declares the modules |
| [`src/mergesort.rs`](src/mergesort.rs) | Mergesort with list recursion and pattern matching |
| [`src/higher_order.rs`](src/higher_order.rs) | Closures, iterators and higher-order functions |
| [`src/immutable.rs`](src/immutable.rs) | Immutable data and persistent lists |
| [`Cargo.toml`](Cargo.toml) | Project manifest |
| [`rust-toolchain.toml`](rust-toolchain.toml) | Pinned Rust version |


## Owners

Each prototype module is owned by one contributor.

| Module | Owner |
|---|---|
| [`immutable.rs`](src/immutable.rs) | Bartosz Nicolas Aniola |
| [`higher_order.rs`](src/higher_order.rs) | Iuno Philips |
| [`mergesort.rs`](src/mergesort.rs) | Alea Munnik |
