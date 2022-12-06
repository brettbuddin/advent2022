use anyhow::{Context, Result};

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("sop-example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    for line in data.lines() {
        let (example, pos) = line.split_once(" ").unwrap();
        assert_eq!(detect(example, 4)?, pos.parse::<usize>()?);
    }

    let file = Data::get("input.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    println!("Part 1: {}", detect(data, 4)?);

    let file = Data::get("som-example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    for line in data.lines() {
        let (example, pos) = line.split_once(" ").unwrap();
        assert_eq!(detect(example, 14)?, pos.parse::<usize>()?);
    }

    let file = Data::get("input.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    println!("Part 2: {}", detect(data, 14)?);

    Ok(())
}

pub fn detect(data: &str, size: usize) -> Result<usize> {
    let data: Vec<u8> = data.as_bytes().to_vec();
    let windows = data.windows(size).enumerate();

    'outer: for w in windows {
        let (idx, w) = w;
        let len = w.len();

        for i in 0..len {
            for j in i + 1..len {
                if w[i] == w[j] {
                    continue 'outer;
                }
            }
        }
        return Ok(idx + size);
    }
    Err(anyhow::Error::msg("no marker found"))
}
