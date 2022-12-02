use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file = dir.join("data/calories.txt");

    println!("Phase 1: {}", phase1(&file)?);
    println!("Phase 2: {}", phase2(&file)?);

    Ok(())
}

fn phase1(file: &PathBuf) -> Result<i32> {
    let file = File::open(file)?;
    let elves = elf_calories(&file)?;
    elves.iter().max().context("no maximum").map(|v| *v)
}

fn phase2(file: &PathBuf) -> Result<i32> {
    let file = File::open(file)?;
    let mut elves = elf_calories(&file)?;
    elves.sort();
    Ok(elves[elves.len() - 3..].iter().sum::<i32>())
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
