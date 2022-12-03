use crate::priority;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Deref;

pub fn run(data: &str) -> Result<u32> {
    let scores = priority::scores();
    let chunks = data.lines().into_iter().chunks(3);

    let sum = chunks.into_iter().fold(0, |accum, chunk| {
        // For each line in the chunk, map them into an ItemSet and collect them into a vector.
        let mut group: Vec<ItemSet> = chunk.map_into().collect();

        // Take the first elf as a starting point for folding.
        let (first, rest) = (group.pop().unwrap(), group);

        let sum = rest
            // Fold over the first elf's ItemSet searching for common items across the remaining elves.
            .iter()
            .fold(first, |common, items| common.intersection(items))
            // Accumulate the priority sum of the common items.
            .iter()
            .fold(0, |accum, c| {
                let pos = scores.find(c.clone()).unwrap() as u32;
                accum + pos + 1
            });

        accum + sum
    });

    Ok(sum)
}

#[derive(Debug, Clone)]
struct ItemSet {
    items: HashSet<char>,
}

impl ItemSet {
    fn intersection(&self, o: &ItemSet) -> ItemSet {
        self.items
            .intersection(o)
            .copied()
            .collect::<HashSet<char>>()
            .into()
    }
}

impl From<HashSet<char>> for ItemSet {
    fn from(items: HashSet<char>) -> Self {
        Self { items }
    }
}

impl From<&str> for ItemSet {
    fn from(v: &str) -> Self {
        Self {
            items: HashSet::from_iter(v.to_string().chars()),
        }
    }
}

impl Deref for ItemSet {
    type Target = HashSet<char>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
