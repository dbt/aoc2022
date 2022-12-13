use crate::interface::*;

pub struct Day01;

fn elves(v: &Vec<String>) -> Result<Vec<i32>> {
    let mut res = vec![];
    let mut rolling = 0;
    for s in v {
        if s.is_empty() {
            res.push(rolling);
            rolling = 0;
        } else {
            let i: i32 = s.parse()?;
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
    fn number(&self) -> u8 {
        1
    }
    fn part01(&self) -> Result<()> {
        let vals = elves(&self.input()?)?;
        let top = top_n(vals, 1);
        println!("{}", top[0]);
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let vals = elves(&self.input()?)?;
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

    #[test]
    fn test_string() {
        assert_eq!("input01.txt", format!("input{:02}.txt", 1));
    }
}
