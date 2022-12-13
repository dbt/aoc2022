use std::str::FromStr;

use crate::interface::*;
use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day04;

struct Assignment {
    b: u32,
    e: u32,
}

struct AssigmentPair {
    first: Assignment,
    second: Assignment,
}

impl FromStr for AssigmentPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
        }
        match RE.captures_iter(s).next() {
            Some(x) => Ok(AssigmentPair {
                first: Assignment {
                    b: x[1].parse().unwrap(),
                    e: x[2].parse().unwrap(),
                },
                second: Assignment {
                    b: x[3].parse().unwrap(),
                    e: x[4].parse().unwrap(),
                },
            }),
            None => Err(anyhow!("Invalid input: {}", s)),
        }
    }
}

fn full_overlap(pair: &AssigmentPair) -> bool {
    let b1 = &pair.first.b;
    let e1 = &pair.first.e;
    let b2 = &pair.second.b;
    let e2 = &pair.second.e;
    (b1 <= b2 && e1 >= e2) || (b2 <= b1 && e2 >= e1)
}

fn partial_overlap(pair: &AssigmentPair) -> bool {
    let b1 = &pair.first.b;
    let e1 = &pair.first.e;
    let b2 = &pair.second.b;
    let e2 = &pair.second.e;
    !(e1 < b2 || e2 < b1)
}

impl Day for Day04 {
    fn number(&self) -> u8 {
        4
    }
    fn part01(&self) -> Result<()> {
        let overlapping = self.input()?
            .into_iter()
            .map(|s| s.parse::<AssigmentPair>().unwrap())
            .filter(full_overlap)
            .count();
        println!("{}", overlapping);
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let overlapping = self.input()?
            .into_iter()
            .map(|s| s.parse::<AssigmentPair>().unwrap())
            .filter(partial_overlap)
            .count();
        println!("{}", overlapping);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vals() -> Vec<&'static str> {
        vec![
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ]
    }

    #[test]
    fn test_p1() {
        let overlapping = test_vals()
            .into_iter()
            .map(|s| s.parse::<AssigmentPair>().unwrap())
            .filter(full_overlap)
            .count();
        assert_eq!(overlapping, 2);
    }
    #[test]
    fn test_p2() {
        let overlapping = test_vals()
            .into_iter()
            .map(|s| s.parse::<AssigmentPair>().unwrap())
            .filter(partial_overlap)
            .count();
        assert_eq!(overlapping, 4);
    }
}
