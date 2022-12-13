use adventools::prelude::*;
use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};
use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(s_n), Packet::Number(o_n)) => s_n.cmp(o_n),
            (Packet::List(s_l), Packet::List(o_l)) => {
                let mut s_iter = s_l.iter();
                let mut o_iter = o_l.iter();
                while let Some(s_next) = s_iter.next() {
                    let o_next = o_iter.next();
                    if o_next.is_none() {
                        return Ordering::Greater;
                    }
                    let comp = s_next.cmp(o_next.unwrap());
                    if comp != Ordering::Equal {
                        return comp;
                    }
                }
                if o_iter.next().is_some() {
                    return Ordering::Less;
                } else {
                    return Ordering::Equal;
                }
            }
            (Packet::Number(s_n), Packet::List(_)) => {
                Packet::List(vec![Packet::Number(*s_n)]).cmp(other)
            }
            (Packet::List(_), Packet::Number(o_n)) => {
                self.cmp(&Packet::List(vec![Packet::Number(*o_n)]))
            }
        }
    }
}

fn p_empty_list(s: &str) -> IResult<&str, Packet> {
    map_res(tag("[]"), |_| Ok::<_, anyhow::Error>(Packet::List(vec![])))(s)
}

fn p_packet(s: &str) -> IResult<&str, Packet> {
    nom::branch::alt((p_list, p_number, p_empty_list))(s)
}

fn p_list(s: &str) -> IResult<&str, Packet> {
    let list_p = separated_list1(terminated(char(','), multispace0), p_packet);
    let mapped = map_res(list_p, |v| Ok::<_, anyhow::Error>(Packet::List(v)));
    delimited(char('['), mapped, char(']'))(s)
}

fn p_number(s: &str) -> IResult<&str, Packet> {
    map_res(digit1, |val: &str| val.parse().map(|u| Packet::Number(u)))(s)
}

impl FromStr for Packet {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (leftover, p) = p_packet(s).unwrap();
        if !leftover.is_empty() {
            Err(anyhow!("Trailing data: {}", leftover))
        } else {
            Ok(p)
        }
    }
}

fn ordered_indices(v: &Vec<String>) -> usize {
    let mut sum = 0;
    for (i, lines) in v.chunks(3).enumerate() {
        let first: Packet = lines[0].parse().unwrap();
        let second: Packet = lines[1].parse().unwrap();
        if first < second {
            sum += i + 1;
        }
    }
    sum
}

fn decoder_key(v: &Vec<String>) -> usize {
    let mut packets: Vec<_> = v
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Packet>())
        .collect::<Result<_>>()
        .unwrap();
    let div1: Packet = "[[2]]".parse().unwrap();
    let div2: Packet = "[[6]]".parse().unwrap();
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();
    let pos1 = packets.iter().position(|v| v == &div1).unwrap() + 1;
    let pos2 = packets.iter().position(|v| v == &div2).unwrap() + 1;
    return pos1 * pos2;
}

pub struct D {}

impl Day for D {
    fn number(&self) -> u8 {
        13
    }
    fn part01(&self) -> Result<()> {

        println!("{}", ordered_indices(&self.input()?));
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        println!("{}", decoder_key(&self.input()?));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let testdata = vec![
            ("1", Packet::Number(1)),
            (
                "[1,2]",
                Packet::List(vec![Packet::Number(1), Packet::Number(2)]),
            ),
            (
                "[1, 2, [3, 4]]",
                Packet::List(vec![
                    Packet::Number(1),
                    Packet::Number(2),
                    Packet::List(vec![Packet::Number(3), Packet::Number(4)]),
                ]),
            ),
            ("[]", Packet::List(vec![])),
            ("[[]]", Packet::List(vec![Packet::List(vec![])])),
        ];
        for (inp, expected) in testdata {
            assert_eq!(inp.parse::<Packet>().unwrap(), expected, "{}", inp);
        }
    }
    #[test]
    fn test_cmp() {
        let test_data = [
            ("[1,1,3,1,1]", "[1,1,5,1,1]", Ordering::Less),
            ("[[1],[2,3,4]]", "[[1],4]", Ordering::Less),
            ("[9]", "[[8,7,6]]", Ordering::Greater),
            ("[[4,4],4,4]", "[[4,4],4,4,4]", Ordering::Less),
            ("[7,7,7,7]", "[7,7,7]", Ordering::Greater),
            ("[]", "[3]", Ordering::Less),
            ("[[[]]]", "[[]]", Ordering::Greater),
            (
                "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                "[1,[2,[3,[4,[5,6,0]]]],8,9]",
                Ordering::Greater,
            ),
        ];
        for (one, two, expected) in test_data {
            assert_eq!(
                one.parse::<Packet>()
                    .unwrap()
                    .cmp(&two.parse::<Packet>().unwrap()),
                expected
            );
        }
    }

    fn test_data() -> Vec<String> {
        split_str(
            r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        )
    }
    #[test]
    fn test_p1() {
        assert_eq!(ordered_indices(&test_data()), 13);
    }
    #[test]
    fn test_p2() {
        assert_eq!(decoder_key(&test_data()), 140);
    }
}
