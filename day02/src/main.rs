use anyhow::Result;
use std::path::PathBuf;

mod part1;
mod part2;

fn main() -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file = dir.join("data/rounds.txt");

    println!("Phase 1: {:?}", part1::run(&file)?);
    println!("Phase 2: {:?}", part2::run(&file)?);

    Ok(())
}
