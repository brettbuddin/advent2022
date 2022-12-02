I'm doing my [Advent of Code 2022](https://adventofcode.com) in Rust this year.

This is a [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
containing a crate for each day of the calendar.

To add a day I use [Cargo Generate](https://github.com/cargo-generate/cargo-generate) to generate a crate for that day.

```console
$ cargo generate --name day02 --path ./template
```

Add the new day to the [`Cargo.toml`](Cargo.toml):

```toml
[workspace]
members = [
    "day01",
    "day02", # Day 2!
]
```

To run a specific day:

```console
$ cargo run --bin day01
```
