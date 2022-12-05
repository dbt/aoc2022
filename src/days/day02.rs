use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::anyhow;

use crate::interface::*;

#[derive(Copy, Clone, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_char(c: char) -> Option<RPS> {
        match c {
            'A' => Some(RPS::Rock),
            'B' => Some(RPS::Paper),
            'C' => Some(RPS::Scissors),
            'X' => Some(RPS::Rock),
            'Y' => Some(RPS::Paper),
            'Z' => Some(RPS::Scissors),
            _ => None,
        }
    }
    fn value(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
    fn score(&self, other: RPS) -> i32 {
        let diff = (self.value() + 4 - other.value()) % 3;
        diff * 3 + self.value()
    }
}

#[derive(Copy, Clone, Debug)]
enum Strat {
    Lose,
    Draw,
    Win,
}

impl Strat {
    fn from_rps(rps: RPS) -> Strat {
        match rps {
            RPS::Rock => Self::Lose,
            RPS::Paper => Self::Draw,
            RPS::Scissors => Self::Win,
        }
    }
    fn pick_move(&self, opponent: RPS) -> RPS {
        use RPS::*;
        let index = ((match self {
            Strat::Lose => -1,
            Strat::Draw => 0,
            Strat::Win => 1,
        } + opponent.value()
            + 2)
            % 3) as usize;
        let moves = vec![Rock, Paper, Scissors];
        moves[index]
    }
}

fn read_strategies(fname: &str) -> Result<Vec<(RPS, RPS)>> {
    let f = File::open(fname)?;
    let lines = BufReader::new(f).lines();
    lines
        .into_iter()
        .map(|x| -> Result<(RPS, RPS)> {
            let s = x?;
            let handy = |ochar: Option<char>| -> Result<RPS> {
                let rps = match ochar {
                    Some(c) => RPS::from_char(c),
                    None => None,
                };
                match rps {
                    Some(m) => Ok(m),
                    None => Err(anyhow!("Invalid input: {}", s)),
                }
            };
            let first = handy(s.chars().next())?;
            let second = handy(s.chars().nth(2))?;

            Ok((first, second))
        })
        .collect()
}

fn lookup_strat(inp: (RPS, RPS)) -> (RPS, RPS) {
    let (opp, strat_hack) = inp;
    let strat = Strat::from_rps(strat_hack);
    let my_move = strat.pick_move(opp);
    (opp, my_move)
}

pub struct Day02;
impl Day for Day02 {
    fn number(&self) -> u8 {
        2
    }
    fn part01(&self) -> Result<()> {
        let strats = read_strategies("input02.txt")?;
        let total: i32 = strats.into_iter().map(|(lhs, rhs)| rhs.score(lhs)).sum();
        println!("{}", total);
        Ok(())
    }

    fn part02(&self) -> Result<()> {
        let strats = read_strategies("input02.txt")?;
        let total: i32 = strats
            .into_iter()
            .map(lookup_strat)
            .map(|(opp, me)| me.score(opp))
            .sum();
        println!("{}", total);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let strats = read_strategies("test02.txt").unwrap();
        let total: i32 = strats.into_iter().map(|(lhs, rhs)| rhs.score(lhs)).sum();
        assert_eq!(total, 15)
    }

    #[test]
    fn test_p2() {
        let strats = read_strategies("test02.txt").unwrap();
        let total: i32 = strats
            .into_iter()
            .map(lookup_strat)
            .map(|(lhs, rhs)| rhs.score(lhs))
            .sum();
        assert_eq!(total, 12)
    }
}
