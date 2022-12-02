pub mod day01;
pub mod day02;

use crate::interface::Day;

pub fn all_days() -> Vec<Box<dyn Day>> {
    vec![Box::new(day01::Day01 {}), Box::new(day02::Day02 {})]
}
