use std::collections::HashSet;

use adventools::prelude::*;

type Pos = (i32, i32);

fn sign(i: i32) -> i32 {
    if i == 0 {
        0
    } else if i < 0 {
        -1
    } else {
        1
    }
}

fn chase((hx, hy): Pos, (tx, ty): Pos) -> Pos {
    let nx: i32 = hx - tx;
    let ny: i32 = hy - ty;
    if nx.abs() > 1 || ny.abs() > 1 {
        (tx + sign(nx), ty + sign(ny))
    } else {
        (tx, ty)
    }
}

fn watch_tail(len: usize, moves: Vec<String>) -> usize {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut snake: Vec<Pos> = Vec::new();
    for _ in 0..len {
        snake.push((0, 0));
    }
    for m in moves {
        let (dx, dy) = match m.chars().next().unwrap() {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => unimplemented!(),
        };
        let count: usize = m[2..].parse().unwrap();
        for _ in 0..count {
            let (hx, hy) = snake[0];
            snake[0] = (hx + dx, hy + dy);
            for idx in 1..len {
                let updated = chase(snake[idx - 1], snake[idx]);
                snake[idx] = updated;
            }
            visited.insert(*snake.iter().rev().next().unwrap());
        }
    }
    visited.len()
}

pub struct D {}

impl Day for D {
    fn number(&self) -> u8 {
        9
    }
    fn part01(&self) -> Result<()> {
        let moves = self.input()?;
        println!("{}", watch_tail(2, moves));
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let moves = self.input()?;
        println!("{}", watch_tail(10, moves));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_vals() -> Vec<String> {
        r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        .split('\n')
        .map(|s| s.to_string())
        .collect()
    }

    fn bigger_test() -> Vec<String> {
        r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            .split('\n')
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_p1() {
        let t = test_vals();
        assert_eq!(watch_tail(2, t), 13);
    }
    #[test]
    fn test_p2() {
        let t = test_vals();
        assert_eq!(watch_tail(10, t), 1);
        assert_eq!(watch_tail(10, bigger_test()), 36);
    }
}
