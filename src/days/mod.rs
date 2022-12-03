pub mod day01;
pub mod day02;
pub mod day03;

use crate::interface::Day;

pub fn all_days() -> Vec<Box<dyn Day>> {
    vec![
        Box::new(day01::Day01 {}),
        Box::new(day02::Day02 {}),
        Box::new(day03::Day03 {}),
    ]
}
