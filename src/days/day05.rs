use std::str::FromStr;

use crate::interface::*;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day05;

struct Containers {
    stacks: Vec<Vec<char>>,
}

impl Containers {
    fn read(inputs: Vec<&str>, batch: bool) -> Containers {
        let mut stacks: Vec<Vec<char>> = vec![];
        lazy_static! {
            static ref C_RE: Regex = Regex::from_str(r"(\[(?P<id>[A-Z])\])|( {4})").unwrap();
            static ref M_RE: Regex =
                Regex::from_str(r"move (?P<count>\d+) from (?P<src>\d) to (?P<dest>\d)").unwrap();
        }
        for i in &inputs {
            for (idx, contents) in C_RE.captures_iter(i).enumerate() {
                if stacks.len() <= idx {
                    stacks.push(vec![]);
                }
                match contents.name("id") {
                    Some(id) => stacks[idx].push(i.chars().nth(id.start()).unwrap()),
                    None => (),
                }
            }
        }
        for i in 0..stacks.len() {
            stacks[i].reverse();
        }
        for i in inputs {
            match M_RE.captures(i) {
                None => (),
                Some(g) => {
                    let src: usize = g["src"].parse().unwrap();
                    let dest: usize = g["dest"].parse().unwrap();
                    let count: usize = g["count"].parse().unwrap();
                    if batch {
                        let split = stacks[src - 1].len() - count;
                        let mut crane: Vec<_> = stacks[src - 1].split_off(split);
                        stacks[dest - 1].append(&mut crane);
                    } else {
                        for _ in 0..count {
                            let crane_contents = stacks[src - 1].pop().unwrap();
                            stacks[dest - 1].push(crane_contents);
                        }
                    }
                }
            }
        }
        Containers { stacks: stacks }
    }
    fn tops(&self) -> String {
        self.stacks.iter().map(|v| v[v.len() - 1]).collect()
    }
}

impl Day for Day05 {
    fn number(&self) -> u8 {
        5
    }
    fn part01(&self) -> Result<()> {
        let lines = read_lines("input05.txt")?;
        let container = Containers::read(lines.iter().map(|s| s.as_str()).collect(), false);
        println!("{}", container.tops());
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let lines = read_lines("input05.txt")?;
        let container = Containers::read(lines.iter().map(|s| s.as_str()).collect(), true);
        println!("{}", container.tops());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vals() -> Vec<&'static str> {
        vec![
            "    [D]     ",
            "[N] [C]     ",
            "[Z] [M] [P] ",
            " 1   2   3  ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
    }

    #[test]
    fn test_p1() {
        let c = Containers::read(test_vals(), false);
        println!("{:?}", c.stacks);
        assert_eq!(c.tops(), "CMZ");
    }
    #[test]
    fn test_p2() {
        let c = Containers::read(test_vals(), true);
        println!("{:?}", c.stacks);
        assert_eq!(c.tops(), "MCD");
    }
}
