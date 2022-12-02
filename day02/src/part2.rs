use anyhow::{Context, Result};

pub fn run(data: &str) -> Result<i32> {
    let mut rounds = Vec::new();
    for line in data.lines() {
        let round: Round = line.split_once(" ").context("no split")?.into();
        rounds.push(round);
    }
    Ok(rounds.iter().fold(0, |sum, r| sum + r.score()))
}

#[derive(Debug)]
struct Round(Play, Outcome);

impl Round {
    fn score(&self) -> i32 {
        let choice = match (&self.0, &self.1) {
            (Play::Paper, Outcome::Loss) => Play::Rock,
            (Play::Rock, Outcome::Loss) => Play::Scissors,
            (Play::Scissors, Outcome::Loss) => Play::Paper,
            (Play::Paper, Outcome::Win) => Play::Scissors,
            (Play::Rock, Outcome::Win) => Play::Paper,
            (Play::Scissors, Outcome::Win) => Play::Rock,
            (x, Outcome::Draw) => x.clone(),
            (_, _) => Play::Unknown,
        };

        choice.score() + self.1.score()
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

impl From<&str> for Outcome {
    fn from(v: &str) -> Self {
        match v {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => Self::Loss,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => Self::Unknown,
        }
    }
}
