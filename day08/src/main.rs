use anyhow::{Context, Result};

use std::collections::HashSet;

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    assert_eq!(part1(&data), 21);
    assert_eq!(part2(&data)?, 8);

    let file = Data::get("input.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data)?);

    Ok(())
}

fn part1(data: &str) -> usize {
    let m = Matrix::parse(&data);
    let row = visible_by_row(&m);
    let col = visible_by_col(&m);

    let mut combine = HashSet::<(usize, usize)>::new();
    combine.extend(&row);
    combine.extend(&col);
    combine.len()
}

fn part2(data: &str) -> Result<usize> {
    let m = Matrix::parse(&data);

    let max = (1..m.rows)
        .flat_map(|row| {
            (1..m.columns)
                .map(|col| position_visible_product(&m, row, col))
                .collect::<Vec<usize>>()
        })
        .max()
        .context("no maximum")?;

    Ok(max)
}

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<u32>>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    fn parse(data: &str) -> Self {
        let data = data
            .lines()
            .fold(Vec::<Vec<u32>>::new(), |mut accum, line| {
                accum.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
                accum
            });
        let rows = data[0].len();
        let columns = data.len();

        Self {
            data,
            rows,
            columns,
        }
    }

    fn row(&self, i: usize) -> Vec<u32> {
        self.data.get(i).unwrap().clone()
    }

    fn col(&self, i: usize) -> Vec<u32> {
        self.data.iter().map(|r| *r.get(i).unwrap()).collect()
    }
}

fn edge_visible(trees: Vec<u32>) -> HashSet<usize> {
    let mut set = HashSet::new();
    let mut max = (-1, -1);
    let double_ended = trees.iter().enumerate().zip(trees.iter().enumerate().rev());

    for v in double_ended {
        let ((fwd_i, fwd_v), (rev_i, rev_v)) = v;

        let fwd_v = *fwd_v as i32;
        if fwd_v > max.0 {
            max.0 = fwd_v;
            set.insert(fwd_i);
        }

        let rev_v = *rev_v as i32;
        if rev_v > max.1 {
            max.1 = rev_v;
            set.insert(rev_i);
        }
    }
    set
}

fn visible_by_row(m: &Matrix) -> HashSet<(usize, usize)> {
    (0..m.rows).fold(HashSet::new(), |mut accum, row| {
        for col in edge_visible(m.row(row)) {
            accum.insert((row, col));
        }
        accum
    })
}

fn visible_by_col(m: &Matrix) -> HashSet<(usize, usize)> {
    (0..m.columns).fold(HashSet::new(), |mut accum, col| {
        for row in edge_visible(m.col(col)) {
            accum.insert((row, col));
        }
        accum
    })
}

fn position_visible(trees: Vec<u32>, pos: usize) -> usize {
    debug_assert!(trees.len() > pos);

    let mut rev_sum = 0;
    for v in trees[..pos].iter().rev() {
        rev_sum += 1;
        if *v >= trees[pos] {
            break;
        }
    }

    let mut fwd_sum = 0;
    for v in trees[pos + 1..].iter() {
        fwd_sum += 1;
        if *v >= trees[pos] {
            break;
        }
    }

    rev_sum * fwd_sum
}

fn position_visible_product(m: &Matrix, row: usize, col: usize) -> usize {
    let row_prod = position_visible(m.row(row), col);
    let col_prod = position_visible(m.col(col), row);
    row_prod * col_prod
}
