use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    assert_eq!(part1(&data)?, 95437);
    assert_eq!(part2(&data)?, 24933642);

    let file = Data::get("input.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    println!("Part 1: {}", part1(&data)?);
    println!("Part 2: {}", part2(&data)?);

    Ok(())
}

fn part1(data: &str) -> Result<usize> {
    const LIMIT: usize = 100_000;

    let sum = calculate_sizes(data)?
        .values()
        .filter(|v| **v < LIMIT)
        .sum();

    Ok(sum)
}

fn part2(data: &str) -> Result<usize> {
    const MAX: usize = 70_000_000;
    const NEEDED: usize = 30_000_000;

    let sizes = calculate_sizes(data)?;
    let used = sizes.get("/").context("no root size")?;
    let available = MAX - used;
    let release = NEEDED - available;

    let remove_size = sizes
        .values()
        .filter(|v| **v >= release)
        .min()
        .context("no minimum")?;

    Ok(*remove_size)
}

fn calculate_sizes(data: &str) -> Result<HashMap<String, usize>> {
    let mut path = PathBuf::from("/");
    let mut sizes = HashMap::<String, usize>::new();

    for line in data.lines() {
        match line {
            // Skip
            "$ cd /" | "$ ls" => {}
            s if s.starts_with("dir") => {}

            // Movement
            "$ cd .." => {
                path.pop();
            }
            s if s.starts_with("$ cd") => {
                path.push(s.strip_prefix("$ cd ").unwrap().to_string());
            }

            // Sizes
            s => {
                let (size, _) = s.split_once(" ").context("no space in file line")?;
                let size = size.parse::<usize>()?;

                for ancestor in path.as_path().ancestors() {
                    let ancestor = ancestor.to_str().context("can't convert to str")?;
                    sizes
                        .entry(ancestor.into())
                        .and_modify(|entry| *entry += size)
                        .or_insert(size);
                }
            }
        }
    }

    Ok(sizes)
}
