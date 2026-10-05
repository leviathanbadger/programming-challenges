mod day1part1;
mod day1part2;
mod day2part1;
mod day2part2;
mod day3part1;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Day 1, part 1: {}", day1part1::compute()?);
    println!("Day 1, part 2: {}", day1part2::compute()?);
    println!("Day 2, part 1: {}", day2part1::compute()?);
    println!("Day 2, part 2: {}", day2part2::compute()?);
    println!("Day 3, part 1: {}", day3part1::compute()?);

    Ok(())
}
