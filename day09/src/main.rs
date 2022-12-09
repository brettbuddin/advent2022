use anyhow::{Context, Result};
use std::collections::HashSet;

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    assert_eq!(simulate_rope(&data, 2)?.history.len(), 13);
    assert_eq!(simulate_rope(&data, 10)?.history.len(), 1);

    let file = Data::get("input.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    println!("Part 1: {}", simulate_rope(&data, 2)?.history.len());
    println!("Part 2: {}", simulate_rope(&data, 10)?.history.len());

    Ok(())
}

fn simulate_rope(data: &str, knots: usize) -> Result<Rope> {
    let mut rope = Rope::new(knots);
    let ops = parse_ops(&data)?;
    rope.apply(ops)?;
    Ok(rope)
}

#[derive(Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
    history: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(count: usize) -> Self {
        let knots = (0..count).map(|_| (0, 0)).collect();
        let history = HashSet::from([(0, 0)]);
        Self { knots, history }
    }

    fn apply(&mut self, ops: Vec<(Direction, usize)>) -> Result<()> {
        for op in ops {
            for _ in 0..op.1 {
                self.step(&op.0)?;
            }
        }
        Ok(())
    }

    fn step(&mut self, d: &Direction) -> Result<()> {
        self.step_head(d);
        self.step_tail();
        self.update_history()?;
        Ok(())
    }

    fn step_head(&mut self, d: &Direction) {
        let ref mut head = self.knots[0];
        match d {
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
        }
    }

    fn step_tail(&mut self) {
        for i in 1..self.knots.len() {
            follow(self.knots[i - 1], &mut self.knots[i]);
        }
    }

    fn update_history(&mut self) -> Result<()> {
        self.history
            .insert(self.knots.last().context("tail")?.clone());
        Ok(())
    }
}

fn follow(head: (i32, i32), tail: &mut (i32, i32)) {
    fn limit(n: i32) -> i32 {
        if n > 0 {
            return 1;
        }
        if n < 0 {
            return -1;
        }
        0
    }

    let diff = (head.0 - tail.0, head.1 - tail.1);
    if diff.0.abs() > 1 || diff.1.abs() > 1 {
        tail.0 += limit(diff.0);
        tail.1 += limit(diff.1);
    }
}

fn parse_ops(data: &str) -> Result<Vec<(Direction, usize)>> {
    let mut ops: Vec<(Direction, usize)> = Vec::new();
    for line in data.lines() {
        let (dir, step) = line.split_once(" ").context("split")?;
        let step = step.parse::<usize>().context("parse step")?;
        let m = match dir {
            "U" => Some((Direction::Up, step)),
            "D" => Some((Direction::Down, step)),
            "L" => Some((Direction::Left, step)),
            "R" => Some((Direction::Right, step)),
            _ => None,
        };
        if m.is_some() {
            ops.push(m.unwrap());
        }
    }
    Ok(ops)
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
