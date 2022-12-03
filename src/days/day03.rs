use crate::interface::*;

pub struct Day03;

fn find_common_item(items: Vec<&str>) -> u8 {
    let mut counts: [usize; 128] = [0; 128];
    let count = items.len();
    for coll in items {
        let mut hits: [bool; 128] = [false; 128];
        for ch in coll.as_bytes() {
            hits[(*ch as usize)] = true;
        }
        for i in 0..hits.len() {
            if hits[i] {
                counts[i] += 1;
            }
        }
    }
    for i in 0..128 {
        if counts[i] == count {
            return i as u8;
        }
    }
    0
}

fn find_halves(sack: &str) -> u8 {
    let (first, second) = sack.split_at(sack.len() / 2);
    let items = vec![first, second];
    find_common_item(items)
}

fn priority(item: u8) -> u8 {
    if item >= b'a' {
        return *(&item) - b'a' + 1;
    }
    return item - b'A' + 27;
}

impl Day for Day03 {
    fn day(&self) -> &'static str {
        "Day 3"
    }
    fn part01(&self) -> Result<()> {
        let sacks = read_lines("input03.txt")?;
        let total: i32 = sacks
            .into_iter()
            .map(|s| find_halves(s.as_str()))
            .map(priority)
            .map(i32::from)
            .sum();
        println!("{}", total);
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let sacks = read_lines("input03.txt")?;
        let groups = sacks.chunks(3);
        let sum: i32 = groups
            .into_iter()
            .map(|x| {
                let v: Vec<&str> = x.into_iter().map(|s| s.as_str()).collect();
                find_common_item(v)
            })
            .map(priority)
            .map(i32::from)
            .sum();
        println!("{}", sum);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vals() -> Vec<&'static str> {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
    }

    #[test]
    fn test_p1() {
        let sacks = test_vals();
        let total: i32 = sacks
            .into_iter()
            .map(find_halves)
            .debug("halves")
            .map(priority)
            .debug("priority")
            .map(i32::from)
            .sum();
        assert_eq!(157, total);
    }
    #[test]
    fn test_p2() {}
}
