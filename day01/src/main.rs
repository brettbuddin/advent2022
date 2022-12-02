use anyhow::{Context, Result};

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    assert_eq!(phase1(&data)?, 24000);
    assert_eq!(phase2(&data)?, 45000);

    let file = Data::get("calories.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    println!("Phase 1: {}", phase1(&data)?);
    println!("Phase 2: {}", phase2(&data)?);

    Ok(())
}

fn phase1(data: &str) -> Result<i32> {
    let elves = elf_calories(&data)?;
    elves.iter().max().context("no maximum").map(|v| *v)
}

fn phase2(data: &str) -> Result<i32> {
    let mut elves = elf_calories(&data)?;
    elves.sort();
    Ok(elves[elves.len() - 3..].iter().sum::<i32>())
}

fn elf_calories(data: &str) -> Result<Vec<i32>> {
    let mut sums = vec![0];
    for line in data.lines() {
        let idx = sums.len() - 1;
        match line.parse::<i32>() {
            Ok(v) => {
                sums[idx] += v;
            }
            Err(_) => sums.push(0),
        }
    }
    Ok(sums)
}
