mod input;
mod point;
mod rect_size;

use anyhow::{Result, anyhow};
use input::get_input;
use rect_size::largest_rectangle_from_corner_points;

pub(super) fn compute_part1() -> Result<u64> {
    let points = get_input()?;
    let (_, _, area) = largest_rectangle_from_corner_points(&points)
        .ok_or_else(|| anyhow!("Somehow there were no input points..."))?;
    Ok(area)
}

// pub(super) fn compute_part2() -> Result<i64> {
//     let points = get_input()?;
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::point::parse_puzzle_input;
    use super::*;

    const SAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1_sample() {
        let points = parse_puzzle_input(SAMPLE_INPUT).unwrap();
        let (_, _, area) = largest_rectangle_from_corner_points(&points).unwrap();
        assert_eq!(area, 50);
        // Can't check points, there are multiple pairs with the same max area and max_by_key is an implementation detail
    }
}
