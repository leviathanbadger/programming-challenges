mod day1part1;
mod day1part2;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Day 1, part 1: {}", day1part1::compute()?);
    println!("Day 1, part 2: {}", day1part2::compute()?);

    Ok(())
}
