use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    assert_eq!(part1(&data)?, 10605);
    assert_eq!(part2(&data)?, 2713310158);

    let file = Data::get("input.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    println!("Part 1: {}", part1(&data)?);
    println!("Part 2: {}", part2(&data)?);

    Ok(())
}

fn part1(data: &str) -> Result<i64> {
    let mut monkeys = parse(&data)?;

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            let mut to_send = Vec::<(i64, i64)>::new();

            while m.items.len() > 0 {
                let item = m.items.remove(0);
                m.items_seen += 1;

                let new = match m.operator {
                    Operator::Add => {
                        item + match m.operand {
                            Operand::Old => item,
                            Operand::Int(n) => n,
                        }
                    }
                    Operator::Mult => {
                        item * match m.operand {
                            Operand::Old => item,
                            Operand::Int(n) => n,
                        }
                    }
                };
                let reduced = new / 3;
                let to_monkey = match reduced % m.divisible_by {
                    0 => m.true_monkey,
                    _ => m.false_monkey,
                };
                to_send.push((to_monkey, reduced));
            }

            for (k, v) in &to_send {
                let to_monkey = monkeys.get_mut(*k as usize).unwrap();
                to_monkey.items.push(*v);
            }
        }
    }

    let mut seen = monkeys.iter().map(|m| m.items_seen).collect::<Vec<_>>();
    seen.sort();
    seen.reverse();
    Ok(seen[0..2].iter().product())
}

fn part2(data: &str) -> Result<i64> {
    let mut monkeys = parse(&data)?;
    let reducer: i64 = monkeys.iter().map(|m| m.divisible_by).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            let mut to_send = Vec::<(i64, i64)>::new();

            while m.items.len() > 0 {
                let item = m.items.remove(0);
                m.items_seen += 1;

                let new = match m.operator {
                    Operator::Add => {
                        item + match m.operand {
                            Operand::Old => item,
                            Operand::Int(n) => n,
                        }
                    }
                    Operator::Mult => {
                        item * match m.operand {
                            Operand::Old => item,
                            Operand::Int(n) => n,
                        }
                    }
                };
                let to_monkey = match new % m.divisible_by {
                    0 => m.true_monkey,
                    _ => m.false_monkey,
                };
                to_send.push((to_monkey, new % reducer));
            }

            for (k, v) in &to_send {
                let to_monkey = monkeys.get_mut(*k as usize).unwrap();
                to_monkey.items.push(*v);
            }
        }
    }

    let mut seen = monkeys.iter().map(|m| m.items_seen).collect::<Vec<_>>();
    seen.sort();
    seen.reverse();
    Ok(seen[0..2].iter().product())
}

#[derive(Debug)]
enum Operator {
    Add,
    Mult,
}

#[derive(Debug)]
enum Operand {
    Old,
    Int(i64),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    items_seen: i64,
    operator: Operator,
    operand: Operand,
    divisible_by: i64,
    true_monkey: i64,
    false_monkey: i64,
}

lazy_static! {
    static ref CHUNK_PATTERN: Regex = Regex::new(
        r"Monkey \d+:\s*
  Starting items: (\d+(?:,\s+\d+)*)\s*
  Operation: new = old (\*|\+) (old|\d+)\s*
  Test: divisible by (\d+)\s*
    If true: throw to monkey (\d+)\s*
    If false: throw to monkey (\d+)",
    )
    .unwrap();
}

fn parse(data: &str) -> Result<Vec<Monkey>> {
    let mut monkeys = Vec::<Monkey>::new();

    for cap in CHUNK_PATTERN.captures_iter(&data) {
        let items: Vec<i64> = cap[1]
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let operator = match &cap[2] {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Mult),
            _ => Err(anyhow::Error::msg("unknown operator")),
        }?;
        let operand = match &cap[3] {
            "old" => Operand::Old,
            n => Operand::Int(n.parse::<i64>().unwrap()),
        };
        let divisible_by = cap[4].parse::<i64>().unwrap();
        let true_monkey = cap[5].parse::<i64>().unwrap();
        let false_monkey = cap[6].parse::<i64>().unwrap();

        monkeys.push(Monkey {
            items_seen: 0,
            items,
            operator,
            operand,
            divisible_by,
            true_monkey,
            false_monkey,
        });
    }

    Ok(monkeys)
}
