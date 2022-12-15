use std::{str::FromStr, ops::RangeInclusive, collections::HashSet};

use adventools::prelude::*;
use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;

pub struct D {}

impl DayParsed for D {}

impl Day for D {
    fn number(&self) -> u8 {
        15
    }
    fn part01(&self) -> Result<()> {
        let sensors = self.input_as::<SensorBeaconPair>()?;
        println!("{}", total_occluded(2000000, &sensors));
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let sensors = self.input_as::<SensorBeaconPair>()?;
        let signal = find_missing_beacon(4000000, 4000000, &sensors).unwrap();
        println!("{}", signal);
        Ok(())
    }
}

struct SensorBeaconPair {
    sx: i32,
    sy: i32,
    bx: i32,
    by: i32,
    coverage: i32,
}

impl FromStr for SensorBeaconPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): .* beacon .* x=(-?\d+), y=(-?\d+)").unwrap();
        }
        let caps = RE.captures(s).ok_or_else(|| anyhow!("Invalid input: '{}'", s))?;
        let sx: i32 = caps[1].parse()?;
        let sy: i32 = caps[2].parse()?;
        let bx: i32 = caps[3].parse()?;
        let by: i32 = caps[4].parse()?;
        Ok(SensorBeaconPair { sx, sy, bx, by, coverage: dist((sx, sy), (bx, by)) })
    }
}

fn dist((sx, sy): (i32, i32), (dx, dy): (i32, i32)) -> i32 {
    (sx - dx).abs() + (sy - dy).abs()

}

impl SensorBeaconPair {
    fn occluded_range(&self, y: i32) -> RangeInclusive<i32>{
        let ydist = (self.sy - y).abs();
        if ydist > self.coverage {
            return 0..=-1
        }
        let spread = (ydist - self.coverage).abs();
        let range = (self.sx - spread)..=(self.sx + spread);
        range
    }

    fn covered(&self, x: i32, y: i32) -> bool {
        let range = dist((self.sx, self.sy), (x, y));
        range <= self.coverage
    }
}

fn total_occluded(y: i32, v: &Vec<SensorBeaconPair>) -> usize {
    let beacons: HashSet<_> = v.iter().filter_map(|sb| if sb.by == y { Some(sb.bx) } else { None }).collect();
    let set: HashSet<_> = v.iter()
        .flat_map(|sb| sb.occluded_range(y))
        .filter(|x| !beacons.contains(x))
        .collect();
    set.len()
}

fn find_missing_beacon(max_x: i32, max_y: i32, sensors: &Vec<SensorBeaconPair>) -> Option<i64> {
    for y in 0..=max_y {
        let relevant: Vec<_> = sensors.iter().filter(|s| (y - s.sy).abs() < s.coverage).collect();
        // if y % 100 == 0 {
        //     println!("{}, {}, {}", y, relevant.len(), sensors.len());
        // }
        let mut ranges: Vec<_> = relevant.iter().map(|sb| sb.occluded_range(y)).collect();
        ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
        let mut potential = 0;
        let rangestr = format!("{:?}", ranges);
        for r in ranges {
            if r.contains(&potential) {
                potential = r.end() + 1;
            } else if r.start() > &potential && potential < max_x {
                // println!("{}, ({}, {})", rangestr, potential, y);
                return Some(4000000 as i64 * (potential as i64) + y as i64);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> Vec<String> {
        split_str(r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3")
    }
    #[test]
    fn test_p1() {
        let sensors = parse_lines::<SensorBeaconPair>(&test_data()).unwrap();
        assert_eq!(total_occluded(10, &sensors), 26);
    }
    #[test]
    fn test_p2() {
        let sensors = parse_lines::<SensorBeaconPair>(&test_data()).unwrap();
        assert_eq!(find_missing_beacon(20, 20, &sensors), Some(56000011));
    }
}
