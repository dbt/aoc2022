pub use anyhow::Result;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub trait Day {
    fn day(&self) -> &'static str;
    fn part01(&self) -> Result<()>;
    fn part02(&self) -> Result<()>;
}

pub fn read_lines(path: &str) -> Result<Vec<String>> {
    let f = File::open(path)?;
    let lines: io::Result<Vec<String>> = BufReader::new(f).lines().collect();
    Ok(lines?
        .into_iter()
        .map(|s| s.trim_end().to_string())
        .collect())
}

pub trait DebugExt: Iterator {
    fn debug(self, msg: &str) -> Self
    where
        Self: Clone + Sized,
        Self::Item: std::fmt::Display + std::fmt::Debug,
    {
        let copy = self.clone();
        let v: Vec<Self::Item> = copy.collect();
        println!("{}: {:?}", msg, v);
        return self;
    }
}

impl<I: Iterator> DebugExt for I {}
