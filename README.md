I'm doing my [Advent of Code 2022](https://adventofcode.com) in Rust this year.

This is a [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
containing a crate for each day of the calendar.

To add a day I create a new crate:

```console
$ cargo new day02  # Create the crate for Day 2.
$ mkdir day02/data # Create a data directory for input samples.
```

Add the new day to the [`Cargo.toml`](Cargo.toml):

```toml
[workspace]
members = [
    "day01",
    "day02", # Day 2!
]
```

I run these using `cargo run` so that I can take advantage of the
`CARGO_MANIFEST_DIR` environment variable to locate the `data/` directory for
that day.

```console
$ cargo run day01
```

Within the `main.rs` you can obtain an example data file path like this:

```rust
let file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/example.txt");
```
