use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    let ops = parse(&data)?;
    assert_eq!(part1(&ops).values().sum::<i32>(), 13140);
    assert_eq!(
        part2(&ops),
        r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
    );

    let file = Data::get("input.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    let ops = parse(&data)?;
    println!("Part 1: {:?}", part1(&ops).values().sum::<i32>());
    println!("Part 2:");
    println!("{}", part2(&ops));
    Ok(())
}

#[derive(Debug)]
enum Op {
    Add(i32),
    Noop,
}

fn parse(data: &str) -> Result<Vec<Op>> {
    data.lines()
        .map(|l| l.split_whitespace())
        .map(|split| {
            let chunks = split.collect::<Vec<_>>();
            match chunks[0] {
                "addx" => Ok(Op::Add(chunks[1].parse::<i32>().context("parse")?)),
                "noop" => Ok(Op::Noop),
                _ => Err(anyhow::Error::msg("unknown operation")),
            }
        })
        .collect()
}

fn part1(ops: &Vec<Op>) -> HashMap<i32, i32> {
    let capture_at = HashSet::from([20, 60, 100, 140, 180, 220]);
    let mut samples = HashMap::new();
    let mut x = 1;
    let mut cycles = 0;
    for op in ops {
        match op {
            Op::Add(n) => {
                cycles += 1;
                if capture_at.contains(&cycles) {
                    samples.insert(cycles, cycles * x);
                }
                cycles += 1;
                if capture_at.contains(&cycles) {
                    samples.insert(cycles, cycles * x);
                }
                x += n;
            }
            Op::Noop => {
                cycles += 1;
                if capture_at.contains(&cycles) {
                    samples.insert(cycles, cycles * x);
                }
            }
        };
    }

    samples
}

fn part2(ops: &Vec<Op>) -> String {
    let mut output = String::new();
    let mut x = 1;
    let mut cycle = 0;

    for op in ops {
        match op {
            Op::Add(n) => {
                cycle += 1;
                draw(&mut output, cycle, x);
                cycle += 1;
                draw(&mut output, cycle, x);
                x += n;
            }
            Op::Noop => {
                cycle += 1;
                draw(&mut output, cycle, x);
            }
        };
    }
    output
}

fn draw(mut w: impl Write, cycle: i32, x: i32) {
    let pos = (cycle - 1) % 40;
    let detection = (pos - 1)..=(pos + 1);

    if detection.contains(&x) {
        write!(w, "#");
    } else {
        write!(w, ".");
    }
    if cycle % 40 == 0 {
        writeln!(w);
    }
}
