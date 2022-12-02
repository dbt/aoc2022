use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::interface::*;

pub struct Day01;

fn elves(fname: &str) -> Result<Vec<i32>> {
    let f = File::open(fname)?;
    let lines = BufReader::new(f).lines();
    let mut res = vec![];
    let mut rolling = 0;
    for l in lines {
        let s = l?;
        let tr = s.trim();
        if tr.is_empty() {
            res.push(rolling);
            rolling = 0;
        } else {
            let i: i32 = tr.parse()?;
            rolling += i;
        }
    }
    Ok(res)
}

pub fn top_n(elves: Vec<i32>, count: usize) -> Vec<i32> {
    let mut top = vec![];
    for e in elves {
        top.push(e);
        if top.len() > count {
            top.sort();
            top.swap_remove(0);
        }
    }
    top
}

impl Day for Day01 {
    fn day(&self) -> &'static str {
        "Day 1"
    }
    fn part01(&self) -> Result<()> {
        let vals = elves("input01.txt")?;
        let top = top_n(vals, 1);
        println!("{}", top[0]);
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let vals = elves("input01.txt")?;
        let top = top_n(vals, 3);
        println!("{}", top[0] + top[1] + top[2]);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vals() -> Vec<i32> {
        vec![
            1000 + 2000 + 3000,
            4000,
            5000 + 6000,
            7000 + 8000 + 9000,
            10000,
        ]
    }

    #[test]
    fn test_p1() {
        let elves = test_vals();
        assert_eq!(vec![24000], top_n(elves, 1));
    }
    #[test]
    fn test_p2() {
        let elves = test_vals();
        assert_eq!(45000, top_n(elves, 3).into_iter().sum::<i32>());
    }
}
