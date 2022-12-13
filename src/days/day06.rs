use crate::interface::*;
use std::collections::{HashSet, VecDeque};

pub struct D {}

impl Day for D {
    fn number(&self) -> u8 {
        6
    }

    fn part01(&self) -> Result<()> {
        let lines = self.input()?;
        let s = lines.get(0).unwrap();
        println!("{}", find_signal(&s[..], 4).unwrap());
        Ok(())
    }

    fn part02(&self) -> Result<()> {
        let lines = self.input()?;
        let s = lines.get(0).unwrap();
        println!("{}", find_signal(&s[..], 14).unwrap());
        Ok(())
    }
}

fn find_signal(signal: &str, size: usize) -> Option<usize> {
    let mut q = VecDeque::new();
    for (pos, ch) in signal.char_indices() {
        q.push_back(ch);
        if q.len() < size {
            continue;
        }
        if q.len() > size {
            q.pop_front();
        }
        let s: HashSet<_> = q.iter().cloned().collect();
        if s.len() == size {
            return Some(pos + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestVals {
        input: &'static str,
        pos_4: usize,
        pos_14: usize,
    }
    fn test_vals() -> Vec<TestVals> {
        vec![
            TestVals {
                input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                pos_4: 7,
                pos_14: 19,
            },
            TestVals {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
                pos_4: 5,
                pos_14: 23,
            },
            TestVals {
                input: "nppdvjthqldpwncqszvftbrmjlhg",
                pos_4: 6,
                pos_14: 23,
            },
            TestVals {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                pos_4: 10,
                pos_14: 29,
            },
            TestVals {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                pos_4: 11,
                pos_14: 26,
            },
        ]
    }
    #[test]
    fn test_p1() {
        for t in test_vals() {
            assert_eq!(find_signal(t.input, 4), Some(t.pos_4));
        }
    }
    #[test]
    fn test_p2() {
        for t in test_vals() {
            assert_eq!(find_signal(t.input, 14), Some(t.pos_14));
        }
    }
}
