mod day1;
mod day2;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Day 1: {}", day1::compute()?);
    println!("Day 2: {}", day2::compute()?);

    Ok(())
}
