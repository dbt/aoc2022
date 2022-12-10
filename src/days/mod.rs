pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;

use crate::interface::Day;

pub fn all_days() -> Vec<Box<dyn Day>> {
    vec![
        Box::new(day01::Day01 {}),
        Box::new(day02::Day02 {}),
        Box::new(day03::Day03 {}),
        Box::new(day04::Day04 {}),
        Box::new(day05::Day05 {}),
        Box::new(day06::D {}),
        Box::new(day07::D {}),
        Box::new(day08::D {}),
        Box::new(day09::D {}),
        Box::new(day10::D {}),
    ]
}
