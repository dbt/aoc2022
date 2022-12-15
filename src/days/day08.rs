use crate::interface::*;

struct Grove {
    w: usize,
    h: usize,
    ch: Vec<char>,
}

impl Grove {
    fn new(input: &Vec<String>) -> Grove {
        let w = input[0].len();
        let h = input.len();
        let ch: Vec<_> = input.join("").chars().collect();

        Grove { w, h, ch }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.w + x
    }

    fn sightlines(&self, x: usize, y: usize) -> Vec<Vec<usize>> {
        let left: Vec<_> = (0..x).map(|i| self.idx(i, y)).collect();
        let right: Vec<_> = ((x + 1)..self.w).map(|i| self.idx(i, y)).collect();
        let top: Vec<_> = (0..y).map(|i| self.idx(x, i)).collect();
        let bottom: Vec<_> = ((y + 1)..self.h).map(|i| self.idx(x, i)).collect();

        vec![left, right, top, bottom]
    }

    fn visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || x == self.w - 1 || y == 0 || y == self.h - 1 {
            return true;
        }
        let pos = y * self.w + x;
        let val = self.ch[pos];
        let hidden = self
            .sightlines(x, y)
            .iter()
            .map(|l| l.iter().any(|c| self.ch[*c] >= val))
            .all(|b| b);
        return !hidden;
    }

    fn count_visible(&self) -> usize {
        let mut viz = 0;
        for x in 0..self.w {
            for y in 0..self.h {
                if self.visible(x, y) {
                    viz += 1;
                }
            }
        }
        viz
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let house_val = self.ch[self.idx(x, y)];
        let score_vec = |v: Vec<(usize, usize)>| -> usize {
            v.iter()
                .position(|(ix, iy)| self.ch[self.idx(*ix, *iy)] >= house_val)
                .map(|n| n + 1)
                .or(Some(v.len()))
                .unwrap()
        };
        let left = score_vec((0..x).rev().map(|i| (i, y)).collect());
        let right = score_vec(((x + 1)..self.w).map(|i| (i, y)).collect());
        let above = score_vec((0..y).rev().map(|i| (x, i)).collect());
        let below = score_vec(((y + 1)..self.h).map(|i| (x, i)).collect());
        let score = left * right * above * below;

        score
    }

    fn best_scenic_score(&self) -> usize {
        (0..self.w)
            .flat_map(|x| (0..self.h).map(move |y| self.scenic_score(x, y)))
            .max()
            .unwrap()
    }
}

pub struct D {}

impl Day for D {
    fn number(&self) -> u8 {
        8
    }
    fn part01(&self) -> Result<()> {
        let grove = Grove::new(&self.input()?);
        println!("{}", grove.count_visible());
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let grove = Grove::new(&self.input()?);
        println!("{}", grove.best_scenic_score());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vals() -> Vec<String> {
        r"30373
25512
65332
33549
35390"
            .split('\n')
            .map(|s| s.to_string())
            .collect()
    }
    #[test]
    fn test_p1() {
        let g = Grove::new(&test_vals());
        assert_eq!(g.count_visible(), 21);
    }
    #[test]
    fn test_p2() {
        let g = Grove::new(&test_vals());
        assert_eq!(g.scenic_score(2, 1), 4);
        assert_eq!(g.scenic_score(2, 3), 8);
        assert_eq!(g.best_scenic_score(), 8);
    }
}
