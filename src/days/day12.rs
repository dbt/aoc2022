use adventools::prelude::*;

pub struct D {}

impl Day for D {
    fn number(&self) -> u8 {
        12
    }
    fn part01(&self) -> Result<()> {
        println!("{}", path(&read_lines("input12.txt")?, "S"));
        Ok(())
    }

    fn part02(&self) -> Result<()> {
        println!("{}", path(&read_lines("input12.txt")?, "Sa"));
        Ok(())
    }
}

fn height(ch: char) -> i32 {
    match ch {
        'S' => 1,
        'E' => 26,
        'a'..='z' => ('a'..='z').position(|c| c == ch).unwrap() as i32,
        _ => unimplemented!(),
    }
}

fn path(grid: &Vec<String>, starts: &str) -> usize {
    let w = grid[0].len();
    let h = grid.len();
    let idx = |x: usize, y: usize| y * w + x;
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut visited = vec![false; w * h];
    for y in 0..h {
        for (x, ch) in grid[y].char_indices() {
            if starts.contains(ch) {
                queue.push((x, y));
                visited[idx(x, y)] = true;
            }
        }
    }
    for n in 0..(w * h) {
        let mut next_queue: Vec<(usize, usize)> = Vec::new();
        // println!("on distance {} queue size {}", n, queue.len());
        while let Some((x, y)) = queue.pop() {
            let ch = grid[y].chars().nth(x).unwrap();
            if ch == 'E' {
                return n;
            }
            let pos = height(ch);
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                if (x as i32 + dx) < 0 || (y as i32 + dy) < 0 {
                    continue;
                }
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx == w || ny == h {
                    continue;
                }
                if visited[idx(nx, ny)] {
                    continue;
                }
                let npos = height(grid[ny].chars().nth(nx).unwrap());
                if pos + 1 >= npos {
                    visited[idx(nx, ny)] = true;
                    next_queue.push((nx, ny));
                }
            }
        }
        if next_queue.is_empty() {
            unimplemented!();
        }
        queue.append(&mut next_queue);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_data() -> Vec<String> {
        split_str(
            r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        )
    }

    #[test]
    fn test_p1() {
        assert_eq!(path(&test_data(), "S"), 31);
    }

    #[test]
    fn test_p2() {
        assert_eq!(path(&test_data(), "Sa"), 29);
    }
}
