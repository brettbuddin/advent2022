use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn run(file: &PathBuf) -> Result<i32> {
    let file = BufReader::new(File::open(file)?);

    let mut rounds = Vec::new();
    for line in file.lines() {
        let line = line?;
        let round: Round = line.split_once(" ").context("no split")?.into();
        rounds.push(round);
    }
    Ok(rounds.iter().fold(0, |sum, r| sum + r.score()))
}

struct Round(Play, Play);

impl Round {
    fn score(&self) -> i32 {
        self.outcome().score() + self.1.score()
    }

    fn outcome(&self) -> Outcome {
        // outcome from *our* perspective
        match (&self.1, &self.0) {
            (Play::Paper, Play::Rock)
            | (Play::Rock, Play::Scissors)
            | (Play::Scissors, Play::Paper) => Outcome::Win,
            (x, y) if x == y => Outcome::Draw,
            _ => Outcome::Loss,
        }
    }
}

impl From<(&str, &str)> for Round {
    fn from(v: (&str, &str)) -> Round {
        Round(v.0.into(), v.1.into())
    }
}

#[derive(Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Play {
    Unknown,
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
            _ => 0,
        }
    }
}

impl From<&str> for Play {
    fn from(v: &str) -> Self {
        match v {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => Self::Unknown,
        }
    }
}
