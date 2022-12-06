use anyhow::{Context, Result};

mod part1;
mod part2;

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    assert_eq!(part1::run(data)?, "CMZ");
    assert_eq!(part2::run(data)?, "MCD");

    let file = Data::get("input.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    println!("Part 1: {}", part1::run(data)?);
    println!("Part 2: {}", part2::run(data)?);

    Ok(())
}
