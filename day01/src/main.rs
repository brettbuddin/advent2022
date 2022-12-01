use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file = dir.join("data/calories.txt");

    phase1(&file)?;
    phase2(&file)?;

    Ok(())
}

fn phase1(file: &PathBuf) -> Result<()> {
    let file = File::open(file)?;
    let elves = elf_calories(&file)?;
    let max = elves.iter().max().context("no maximum")?;
    println!("Phase 1: {}", max);
    Ok(())
}

fn phase2(file: &PathBuf) -> Result<()> {
    let file = File::open(file)?;
    let mut elves = elf_calories(&file)?;
    elves.sort();
    let top3_sum = &elves[elves.len() - 3..].iter().sum::<i32>();
    println!("Phase 2: {}", top3_sum);
    Ok(())
}

fn elf_calories(file: &File) -> Result<Vec<i32>> {
    let mut sums = vec![0];
    for line in BufReader::new(file).lines() {
        let idx = sums.len() - 1;
        match line?.parse::<i32>() {
            Ok(v) => {
                sums[idx] += v;
            }
            Err(_) => sums.push(0),
        }
    }
    Ok(sums)
}
