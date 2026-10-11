mod day1part1;
mod day1part2;
mod day2part1;
mod day2part2;
mod day3part1;
mod day3part2;
mod day4part1;
mod day4part2;
mod day5part1;
mod day5part2;
mod day6part1;
mod day6part2;

use anyhow::Result;

pub fn year2025() -> Result<()> {
    println!("Year 2025");
    println!("Day 1, part 1: {}", day1part1::compute()?);
    println!("Day 1, part 2: {}", day1part2::compute()?);
    println!("Day 2, part 1: {}", day2part1::compute()?);
    println!("Day 2, part 2: {}", day2part2::compute()?);
    println!("Day 3, part 1: {}", day3part1::compute()?);
    println!("Day 3, part 2: {}", day3part2::compute()?);
    println!("Day 4, part 1: {}", day4part1::compute()?);
    println!("Day 4, part 2: {}", day4part2::compute()?);
    println!("Day 5, part 1: {}", day5part1::compute()?);
    println!("Day 5, part 2: {}", day5part2::compute()?);
    println!("Day 6, part 1: {}", day6part1::compute()?);
    println!("Day 6, part 2: {}", day6part2::compute()?);

    Ok(())
}
