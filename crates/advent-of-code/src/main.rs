mod day1;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    println!("Day 1: {}", day1::compute()?);

    Ok(())
}
