pub mod day01;

use crate::interface::Day;

pub fn all_days() -> Vec<Box<dyn Day>> {
    vec![Box::new(day01::Day01 {})]
}
