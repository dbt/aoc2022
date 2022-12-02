use advent2022::days::all_days;
use anyhow::Result;

fn main() -> Result<()> {
    for d in all_days() {
        d.part01()?;
        d.part02()?;
    }
    Ok(())
}
