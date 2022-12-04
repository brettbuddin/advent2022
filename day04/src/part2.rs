use anyhow::{Context, Result};
use std::cmp;
use std::ops::RangeInclusive;

pub fn run(data: &str) -> Result<i32> {
    let team: Result<Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>> = data
        .lines()
        .map(|line| {
            let (one, two) = line.split_once(",").context("no comma")?;
            let one = parse_range(one)?;
            let two = parse_range(two)?;
            Ok((one, two))
        })
        .collect();

    let sum = team?.iter().fold(0, |accum, team| {
        // Detect the overlap between the ranges
        let start = cmp::max(team.0.start(), team.1.start());
        let end = cmp::min(team.0.end(), team.1.end());
        if start > end {
            return accum;
        }
        accum + 1
    });

    Ok(sum)
}

fn parse_range(v: &str) -> Result<RangeInclusive<i32>> {
    let (start, end) = v.split_once("-").context("no hyphen")?;
    let start = start.parse::<i32>()?;
    let end = end.parse::<i32>()?;
    Ok(start..=end)
}
