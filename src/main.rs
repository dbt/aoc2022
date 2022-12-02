use advent2022::days::all_days;
use anyhow::Result;
use std::time::Instant;

fn main() -> Result<()> {
    for d in all_days() {
        {
            print!("{} part 1: ", d.day());
            let i1 = Instant::now();
            let res1 = d.part01();
            let dur1 = i1.elapsed();
            println!("Elapsed time: {:?}", dur1);
            res1?;
            println!()
        }

        print!("{} part 2: ", d.day());
        let i2 = Instant::now();
        let res2 = d.part02();
        let dur2 = i2.elapsed();
        println!("Elapsed time: {:?}", dur2);
        res2?;
        println!()
    }
    Ok(())
}
