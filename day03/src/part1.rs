use crate::priority;
use anyhow::Result;
use std::collections::HashSet;
use std::ops::Deref;

pub fn run(data: &str) -> Result<u32> {
    let scores = priority::scores();

    let sum = data.lines().fold(0, |accum, line| {
        let sack: Sack = line.into();
        let score = sack
            // Intersect the two compartments to find common items.
            .intersection()
            // Accumulate the priority sum of the common items.
            .iter()
            .fold(0, |accum, c| {
                let pos = scores.find(c.clone()).unwrap() as u32;
                accum + pos + 1
            });

        accum + score
    });

    Ok(sum)
}

#[derive(Debug)]
struct Sack(ItemSet, ItemSet);

impl Sack {
    fn intersection(&self) -> ItemSet {
        self.0
            .intersection(&self.1)
            .copied()
            .collect::<HashSet<char>>()
            .into()
    }
}

impl From<&str> for Sack {
    fn from(v: &str) -> Self {
        let mid = v.len() / 2;
        v.to_string().split_at(mid).into()
    }
}

impl From<(&str, &str)> for Sack {
    fn from(v: (&str, &str)) -> Self {
        Sack(v.0.into(), v.1.into())
    }
}

#[derive(Debug)]
struct ItemSet {
    items: HashSet<char>,
}

impl From<&str> for ItemSet {
    fn from(v: &str) -> Self {
        Self {
            items: HashSet::from_iter(v.to_string().chars()),
        }
    }
}

impl From<HashSet<char>> for ItemSet {
    fn from(items: HashSet<char>) -> Self {
        Self { items }
    }
}

impl Deref for ItemSet {
    type Target = HashSet<char>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
