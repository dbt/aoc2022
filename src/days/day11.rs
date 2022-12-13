use adventools::prelude::*;
use anyhow::anyhow;
use lazy_static::lazy_static;
use num::integer::lcm;
use regex::{Captures, Regex};

enum Operation {
    Multiply(usize),
    Add(usize),
    Square,
}

impl Operation {
    fn apply(&self, val: usize) -> usize {
        match self {
            Operation::Add(add) => *add + val,
            Operation::Multiply(factor) => *factor * val,
            Operation::Square => val * val,
        }
    }
}

struct Monkey {
    id: usize,
    items: Vec<usize>,
    operation: Operation,
    test_divisor: usize,
    dest_true: usize,
    dest_false: usize,
    extra_divisor: usize,
    counter: usize,
}

fn parse<'a>(re: &Regex, input: &'a str) -> Result<Captures<'a>> {
    let opt = re.captures(input);
    opt.ok_or_else(|| anyhow!("Invalid input: '{}'", input))
}

impl Monkey {
    fn new(input: &[String], extra_divisor: usize) -> Result<Monkey> {
        lazy_static! {
            static ref LINE_1: Regex = Regex::new(r"Monkey (\d+):").unwrap();
            static ref LINE_2: Regex = Regex::new(r"Starting items: (\d+(, \d+)*)$").unwrap();
            static ref LINE_3: Regex =
                Regex::new(r"Operation: new = old ([+*]) (old|(\d+))$").unwrap();
            static ref LINE_4: Regex = Regex::new(r"Test: divisible by (\d+)$").unwrap();
            static ref LINE_5: Regex = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
            static ref LINE_6: Regex = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
        }
        let capture1 = parse(&LINE_1, &input[0])?;
        let id = capture1[1].parse()?;
        let capture2 = parse(&LINE_2, &input[1])?;
        let starting: Vec<_> = capture2[1]
            .split(", ")
            .map(|s| s.parse::<usize>())
            .collect::<Result<_, _>>()?;
        let cap3 = parse(&LINE_3, &input[2])?;
        let op = if &cap3[1] == "+" {
            Operation::Add(cap3[2].parse()?)
        } else if &cap3[1] == "*" && &cap3[2] == "old" {
            Operation::Square
        } else if &cap3[1] == "*" {
            Operation::Multiply(cap3[3].parse()?)
        } else {
            Err(anyhow!("Invalid input: '{}'", &input[2]))?
        };
        let test_divisor: usize = parse(&LINE_4, &input[3])?[1].parse()?;
        let dest_true = parse(&LINE_5, &input[4])?[1].parse()?;
        let dest_false = parse(&LINE_6, &input[5])?[1].parse()?;

        Ok(Monkey {
            id,
            items: starting,
            operation: op,
            test_divisor,
            dest_true,
            dest_false,
            extra_divisor,
            counter: 0,
        })
    }

    fn look(&mut self) -> (Vec<usize>, Vec<usize>) {
        let outputs: Vec<_> = self
            .items
            .iter()
            .map(|&val| self.operation.apply(val) / self.extra_divisor)
            .collect();
        self.counter += self.items.len();
        self.items.clear();
        let trues = outputs
            .iter()
            .filter(|&val| val % self.test_divisor == 0)
            .copied()
            .collect();
        let falses = outputs
            .iter()
            .filter(|&val| val % self.test_divisor != 0)
            .copied()
            .collect();
        (trues, falses)
    }
}

struct Barrel {
    monkeys: Vec<Monkey>,
    lcm: usize,
}

impl Barrel {
    fn new(v: &Vec<String>, extra_divisor: usize) -> Result<Barrel> {
        let r = v
            .chunks(7)
            .map(|c| Monkey::new(c, extra_divisor))
            .collect::<Result<Vec<_>, _>>()?;
        if !r.iter().enumerate().all(|(i, m)| m.id == i) {
            Err(anyhow!("Can't deal with monkeys out of order"))?;
        }
        let lcm = r
            .iter()
            .map(|m| m.test_divisor)
            .fold(1, |acc, el| lcm(acc, el));
        Ok(Barrel { monkeys: r, lcm })
    }

    fn process(&mut self) {
        for idx in 0..self.monkeys.len() {
            let ((mut trues, mut falses), idx_true, idx_false) = {
                let m = &mut self.monkeys[idx];
                (m.look(), m.dest_true, m.dest_false)
            };
            trues.iter_mut().for_each(|v| *v %= self.lcm);
            falses.iter_mut().for_each(|v| *v %= self.lcm);
            self.monkeys[idx_true].items.append(&mut trues);
            self.monkeys[idx_false].items.append(&mut falses);
        }
    }

    fn business(&self) -> usize {
        let mut v: Vec<_> = self.monkeys.iter().map(|m| m.counter).collect();
        v.sort();
        v.reverse();
        v[0] * v[1]
    }
}

pub struct D {}

impl Day for D {
    fn number(&self) -> u8 {
        11
    }
    fn part01(&self) -> Result<()> {
        let mut barrel = Barrel::new(&self.input()?, 3)?;
        for _ in 0..20 {
            barrel.process();
        }
        println!("{}", barrel.business());
        Ok(())
    }

    fn part02(&self) -> Result<()> {
        let mut barrel = Barrel::new(&self.input()?, 3)?;
        for _ in 0..10000 {
            barrel.process();
        }
        println!("{}", barrel.business());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use adventools::prelude::split_str;

    fn test_data() -> Vec<String> {
        split_str(
            r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        )
    }
    #[test]
    fn test_p1() {
        let mut barrel = Barrel::new(&test_data(), 3).unwrap();
        barrel.process();
        assert_eq!(barrel.monkeys[0].items.len(), 4);
        assert_eq!(barrel.monkeys[1].items.len(), 6);
        assert_eq!(barrel.monkeys[0].items, vec![20, 23, 27, 26]);
        for _ in 0..19 {
            barrel.process()
        }
        assert_eq!(barrel.monkeys[0].counter, 101);
        assert_eq!(barrel.monkeys[3].counter, 105);
        assert_eq!(barrel.business(), 10605);
    }

    // #[test]
    fn test_p2() {
        let mut barrel = Barrel::new(&test_data(), 3).unwrap();
        let mut expected: HashMap<usize, Vec<usize>> = HashMap::new();
        // expected.insert(1, vec![2,4,3,6]);
        expected.insert(20, vec![99, 97, 8, 103]);
        for idx in 1..=10000 {
            barrel.process();
            if expected.contains_key(&idx) {
                let exp = &expected[&idx];
                let actual: Vec<_> = barrel.monkeys.iter().map(|m| m.counter).collect();
                assert_eq!(&actual, exp, "failed on iteration {}", idx);
            }
        }
        assert_eq!(barrel.business(), 2713310158);
    }
}
