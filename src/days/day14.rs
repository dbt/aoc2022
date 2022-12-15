use std::convert::identity;
use std::iter::repeat;

use adventools::prelude::*;
use adventools::grid::{Grid, CharSrc};

use regex::Regex;
use lazy_static::lazy_static;

pub struct D {}

impl Day for D {
    fn number(&self) -> u8 {
        14
    }
    fn part01(&self) -> Result<()> {
        let mut grid = grid_from_lines(&self.input()?);
        println!("{}", count_drops(&mut grid));
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let mut grid = grid_from_lines(&self.input()?);
        draw_floor(&mut grid);
        println!("{}", count_drops(&mut grid));
        Ok(())
    }
}
fn pairs_from_line(line: &String) -> Vec<(usize, usize)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\b(\d+),(\d+)\b").unwrap();
    }
    RE.captures_iter(line).map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap())).collect()
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Air,
    Rock,
    Sand,
    Source
}

impl CharSrc for Cell {
    fn char(&self) -> char {
        match self {
            Cell::Air => '.',
            Cell::Rock => '#',
            Cell::Sand => 'o',
            Cell::Source => '+',
        }
    }
}

fn grid_from_lines(input: &Vec<String>) -> Grid<Cell> {
    let lines: Vec<_> = input.iter().map(pairs_from_line).collect();
    let all_pairs: Vec<_> = lines.iter().flat_map(identity).collect();
    let max_y = *all_pairs.iter().map(|(_x, y)| y).max().unwrap() + 2;
    let max_x = *all_pairs.iter().map(|(x, _y)| x).max().unwrap().max(&(max_y + 501));
    // println!("max ({}, {})", max_x, max_y);
    let mut grid = Grid::new(max_x+1, max_y+1, Cell::Air);
    grid[(500, 0)] = Cell::Source;
    for line in lines {
        let first_iter = line.iter();
        let second_iter = line.iter().skip(1);
        for (&(x1, y1), &(x2, y2)) in first_iter.zip(second_iter).into_iter() {
            let cells: Vec<_> = if x1 == x2 {
                if y1 < y2 {
                    repeat(x1).zip(y1..=y2).into_iter().collect()
                } else {
                    repeat(x1).zip(y2..=y1).into_iter().collect()
                }
            } else {
                if x1 < x2 {
                    (x1..=x2).zip(repeat(y1)).collect()
                } else {
                    (x2..=x1).zip(repeat(y1)).collect()
                }
            };
            for xy in cells {
                grid[xy] =Cell::Rock;
            }
        }
    }
    grid
}

fn draw_floor(grid: &mut Grid<Cell>) {
    let y = grid.height() - 1;
    for x in (500 - y - 1)..(500 + y + 1) {
        grid[(x, y)] = Cell::Rock;
    }
}

fn drop_sand(grid: &mut Grid<Cell>) -> Option<(usize, usize)> {
    let mut x = 500;
    let mut y = 0;
    loop {
        if x >= grid.width() || y >= grid.height() || x == 0 || grid[(x,y)] == Cell::Sand {
            return None
        }
        for (dx, dy) in [(0, 1), (-1, 1), (1, 1), (0, 0)] {
            if dx == 0 && dy == 0 {
                // eagle has landed
                grid[(x,y)] = Cell::Sand;
                return Some((x,y));
            }
            let nx = ((x as i32) + dx) as usize;
            let ny = y + dy;
            if nx == grid.width() || ny == grid.height() || nx == 0 {
                // falling off the edge
                return None;
            }
            if grid[(nx, ny)] == Cell::Air {
                x = nx;
                y = ny;
                break;
            }
        }
    }
}

fn count_drops(grid: &mut Grid<Cell>) -> usize {
    for i in (0 as usize).. {
        if drop_sand(grid) == None {
            return i;
        // } else {
        //     println!("iter {}\n{}", i, grid);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<String> {
        split_str(r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9")
    }
    #[test]
    fn test_p1() {
        let mut grid = grid_from_lines(&test_data());
        assert_eq!(count_drops(&mut grid), 24);
    }
    #[test]
    fn test_p2() {
        let mut grid = grid_from_lines(&test_data());
        draw_floor(&mut grid);
        assert_eq!(count_drops(&mut grid), 93);
    }
}
