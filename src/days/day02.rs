use crate::interface::*;

#[derive(Copy, Clone, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_char(c: char) -> RPS {
        match c {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,
            _ => unreachable!("invalid RPS: '{}'", c),
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
    fn from_char(c: char) -> Strat {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!("invalid strat: '{}'", c),
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

fn read_strategies(fname: &str) -> Result<Vec<(RPS, Strat)>> {
    let lines = read_lines(fname)?;
    lines
        .into_iter()
        .map(|s| -> Result<(RPS, Strat)> {
            let first = s.chars().next().map(|c| RPS::from_char(c)).unwrap();
            let second = s.chars().nth(2).map(|c| Strat::from_char(c)).unwrap();

            Ok((first, second))
        })
        .collect()
}

fn map_strat((other, strat): (RPS, Strat)) -> (RPS, RPS) {
    (
        other,
        match strat {
            Strat::Lose => RPS::Rock,
            Strat::Draw => RPS::Paper,
            Strat::Win => RPS::Scissors,
        },
    )
}

fn execute_strat((opp, strat): (RPS, Strat)) -> (RPS, RPS) {
    let my_move = strat.pick_move(opp);
    (opp, my_move)
}

fn score((opp, me): (RPS, RPS)) -> i32 {
    me.score(opp)
}

pub struct Day02;
impl Day for Day02 {
    fn number(&self) -> u8 {
        2
    }
    fn part01(&self) -> Result<()> {
        let strats = read_strategies("input02.txt")?;
        let total: i32 = strats.into_iter().map(map_strat).map(score).sum();
        println!("{}", total);
        Ok(())
    }

    fn part02(&self) -> Result<()> {
        let strats = read_strategies("input02.txt")?;
        let total: i32 = strats.into_iter().map(execute_strat).map(score).sum();
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
        let total: i32 = strats.into_iter().map(map_strat).map(score).sum();
        assert_eq!(total, 15)
    }

    #[test]
    fn test_p2() {
        let strats = read_strategies("test02.txt").unwrap();
        let total: i32 = strats.into_iter().map(execute_strat).map(score).sum();
        assert_eq!(total, 12)
    }
}
