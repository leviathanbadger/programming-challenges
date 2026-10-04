mod day1;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Day 1: {}", day1::compute()?);

    Ok(())
}
