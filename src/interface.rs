pub use anyhow::Result;

pub trait Day {
    fn part01(&self) -> Result<()>;
    fn part02(&self) -> Result<()>;
}
