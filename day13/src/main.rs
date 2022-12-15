use anyhow::{Context, Result};
use serde::Deserialize;
use std::cmp::{Ord, Ordering};

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    assert_eq!(part1(&data)?, 13);
    assert_eq!(part2(&data)?, 140);

    let file = Data::get("input.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    println!("Part 1: {}", part1(&data)?);
    println!("Part 2: {}", part2(&data)?);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum Value {
    List(Vec<Value>),
    Digit(u32),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

impl Ord for Value {
    fn cmp(&self, o: &Self) -> Ordering {
        match (self, o) {
            (Self::Digit(a), Self::Digit(b)) => a.cmp(b),
            (Self::Digit(a), b) => Self::List(vec![Self::Digit(*a)]).cmp(b),
            (a, Self::Digit(b)) => a.cmp(&Self::List(vec![Self::Digit(*b)])),
            (Self::List(a), Self::List(b)) => {
                let mut a_iter = a.iter();
                let mut b_iter = b.iter();
                loop {
                    match (a_iter.next(), b_iter.next()) {
                        (Some(a), Some(b)) => match a.cmp(b) {
                            Ordering::Equal => continue,
                            ord => return ord,
                        },
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
        }
    }
}

fn part1(data: &str) -> Result<i32> {
    let pairs = data.split("\n\n");
    let mut in_order = Vec::new();
    for (i, p) in pairs.enumerate() {
        let lines: Vec<&str> = p.lines().collect();
        let list0 = lex(lines[0])?;
        let list1 = lex(lines[1])?;
        if list0 < list1 {
            in_order.push((i + 1) as i32);
        }
    }

    Ok(in_order.iter().sum())
}

fn part2(data: &str) -> Result<i32> {
    let lines = data.split("\n");
    let mut all = Vec::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let list = lex(line)?;
        all.push(list);
    }

    all.push(Value::List(vec![Value::List(vec![Value::Digit(2)])]));
    all.push(Value::List(vec![Value::List(vec![Value::Digit(6)])]));

    all.sort();
    let markers: Vec<_> = all
        .iter()
        .enumerate()
        .filter_map(|t| {
            if is_marker(2, t.1) || is_marker(6, t.1) {
                return Some((t.0 + 1) as i32);
            }
            None
        })
        .collect();

    Ok(markers.iter().product())
}

fn is_marker(d: u32, v: &Value) -> bool {
    match v {
        Value::List(v) => {
            if v.len() != 1 {
                return false;
            }
            match &v[0] {
                Value::List(v) => {
                    if v.len() != 1 {
                        return false;
                    }
                    if let Value::Digit(v) = v[0] {
                        return v == d;
                    }
                    false
                }
                _ => false,
            }
        }
        _ => false,
    }
}

#[derive(PartialEq)]
enum LexState {
    Stopped,
    Scanning,
    InDigit,
}

fn lex(line: &str) -> Result<Value> {
    let line = line.to_string();
    let mut chars = line.chars();

    let mut parents = Vec::new();
    let mut current_list: Value = Value::List(vec![]);
    let mut current_digit: Value = Value::Digit(0);
    let mut state = LexState::Stopped;

    while let Some(ch) = chars.next() {
        match ch {
            '[' => {
                if state == LexState::Scanning {
                    parents.push(current_list);
                }
                state = LexState::Scanning;
                current_list = Value::List(Vec::new());
            }
            ']' => {
                if state == LexState::InDigit {
                    if let Value::List(v) = &mut current_list {
                        v.push(current_digit.clone());
                    }
                }
                if let Some(mut parent) = parents.pop() {
                    if let Value::List(v) = &mut parent {
                        v.push(current_list.clone());
                    }
                    current_list = parent;
                };
                state = LexState::Scanning;
                current_digit = Value::Digit(0);
            }
            ',' => {
                if state == LexState::InDigit {
                    if let Value::List(v) = &mut current_list {
                        v.push(current_digit.clone());
                    }
                }
                state = LexState::Scanning;
                current_digit = Value::Digit(0);
            }
            ch => {
                if let Value::Digit(d) = current_digit {
                    current_digit = Value::Digit(10 * d + ch.to_digit(10).context("not a number")?);
                }
                state = LexState::InDigit;
            }
        }
    }

    Ok(current_list)
}
