use anyhow::{Context, Result};
use std::cmp;
use std::ops::RangeInclusive;

pub fn run(data: &str) -> usize {
    data.lines()
        .filter_map(|line| {
            let (one, two) = line.split_once(",")?;
            let one = parse_range(one).ok()?;
            let two = parse_range(two).ok()?;
            Some((one, two))
        })
        .filter_map(|pair| {
            let start = cmp::max(pair.0.start(), pair.1.start());
            let end = cmp::min(pair.0.end(), pair.1.end());
            if start > end {
                return None; // no overlap
            }
            Some(pair)
        })
        .count()
}

fn parse_range(v: &str) -> Result<RangeInclusive<i32>> {
    let (start, end) = v.split_once("-").context("no hyphen")?;
    let start = start.parse::<i32>()?;
    let end = end.parse::<i32>()?;
    Ok(start..=end)
}
