mod compute_winding;
mod input;
mod point;
mod rect_size;

use anyhow::{Result, anyhow};
use input::get_input;
use rect_size::{
    largest_rectangle_from_bounding_corner_points, largest_rectangle_from_corner_points,
};

pub(super) fn compute_part1() -> Result<u64> {
    let points = get_input()?;
    let (_, _, area) = largest_rectangle_from_corner_points(&points)
        .ok_or_else(|| anyhow!("Somehow there were no input points..."))?;
    Ok(area)
}

pub(super) fn compute_part2() -> Result<u64> {
    let points = get_input()?;
    let (_, _, area) = largest_rectangle_from_bounding_corner_points(&points)
        .ok_or_else(|| anyhow!("Somehow there were no input points..."))?;
    Ok(area)
}

#[cfg(test)]
mod tests {
    use super::input::parse_puzzle_input;
    use super::point::point;
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
        let (p1, p2, area) = largest_rectangle_from_corner_points(&points).unwrap();
        assert_eq!(area, 50);

        let winners = [(point!(2, 5), point!(11, 1)), (point!(2, 3), point!(11, 7))];
        assert!(
            winners
                .iter()
                .any(|&(a, b)| { (p1 == a && p2 == b) || (p1 == b && p2 == a) })
        );
    }

    #[test]
    fn test_part2_sample() {
        let points = parse_puzzle_input(SAMPLE_INPUT).unwrap();
        let (p1, p2, area) = largest_rectangle_from_bounding_corner_points(&points).unwrap();
        assert_eq!(area, 24);

        let winners = [(point!(2, 3), point!(9, 5))];
        assert!(
            winners
                .iter()
                .any(|&(a, b)| { (p1 == a && p2 == b) || (p1 == b && p2 == a) })
        );
    }
}
