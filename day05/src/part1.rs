use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;

pub fn run(data: &str) -> Result<String> {
    let (stacks, moves): (&str, &str) = data
        .split_once("\n\n")
        .context("no separation between sections")?;

    let mut stacks = parse_stacks(stacks)?;
    let max_key = stacks.keys().cloned().max().context("no keys")?;
    apply_moves(&mut stacks, parse_moves(moves)?)?;

    let mut tops = Vec::<char>::new();
    for i in 0..=max_key {
        let ch = stacks
            .get(&i)
            .context("no stack at index")?
            .last()
            .context("no elements")?;
        tops.push(*ch);
    }
    Ok(tops.iter().collect())
}

fn parse_stacks(text: &str) -> Result<HashMap<usize, Vec<char>>> {
    let mut stacks = HashMap::<usize, Vec<char>>::new();
    for line in text.lines().rev().skip(1) {
        let chars = line.chars().collect::<Vec<char>>();
        let moves = chars
            .chunks(4)
            .enumerate()
            .filter(|chunk| !chunk.1[1].is_whitespace())
            .map(|chunk| (chunk.0, chunk.1[1]));

        for m in moves {
            match stacks.get_mut(&m.0) {
                Some(stack) => {
                    stack.push(m.1);
                }
                None => {
                    stacks.insert(m.0, vec![m.1]);
                }
            }
        }
    }
    Ok(stacks)
}

fn apply_moves(stacks: &mut HashMap<usize, Vec<char>>, moves: Vec<Move>) -> Result<()> {
    for op in moves {
        for _ in 0..op.count {
            let item = {
                let from = stacks.get_mut(&op.from).context("no stack: from")?;
                from.pop().context("nothing to pop")?
            };
            let to = stacks.get_mut(&op.to).context("no stack: to")?;
            to.push(item);
        }
    }
    Ok(())
}

fn parse_moves(moves: &str) -> Result<Vec<Move>> {
    let re = Regex::new(r"^move\s+(\d+)\s+from\s+(\d+)\s+to\s+(\d+)$")?;
    Ok(moves
        .lines()
        .filter_map(|line| parse_move(&re, line))
        .collect())
}

fn parse_move(re: &Regex, v: &str) -> Option<Move> {
    let captures = re.captures(v)?;
    let count = captures.get(1)?.as_str().parse::<usize>().ok()?;
    let from = captures.get(2)?.as_str().parse::<usize>().ok()? - 1;
    let to = captures.get(3)?.as_str().parse::<usize>().ok()? - 1;
    Some(Move { count, from, to })
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}
