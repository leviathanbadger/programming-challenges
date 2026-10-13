use std::str::FromStr;

use super::compute_winding::{Winding, compute_winding};
use super::point::Point;
use anyhow::Result;

pub(super) fn get_input() -> Result<Vec<Point>> {
    let input_str = include_str!("input.txt");
    parse_puzzle_input(input_str)
}

/// Parses the puzzle input - a sequence of 2D points. One point per line,
/// each point using comma-separated axes (like "x,y"). The points are interpreted as
/// a self-closing polygon, and the points are reversed if the polygon would be counterclockwise.
pub(super) fn parse_puzzle_input(text: &str) -> Result<Vec<Point>> {
    let mut points = text
        .lines()
        .map(Point::from_str)
        .collect::<Result<Vec<_>>>()?;

    if compute_winding(&points) == Winding::Counterclockwise {
        points.reverse();
    }

    Ok(points)
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::super::point::point;
    use super::*;

    #[test]
    fn test_parse_puzzle_input() {
        let coords = parse_puzzle_input(
            "162,817
             57 ,618
            906,360
            592, 479
            352,   342",
        )
        .unwrap();

        assert_eq!(coords.len(), 5);
        assert_eq!(coords[0], point!(162, 817));
        assert_eq!(coords[1], point!(57, 618));
        assert_eq!(coords[2], point!(906, 360));
        assert_eq!(coords[3], point!(592, 479));
        assert_eq!(coords[4], point!(352, 342));
    }

    #[test]
    fn test_parse_puzzle_input_invalid_number() {
        let result = parse_puzzle_input("162,i");
        assert_matches!(result, Err(e) if e.to_string().contains("invalid digit"));
    }

    #[test]
    fn test_parse_puzzle_input_one_number() {
        let result = parse_puzzle_input("162");
        assert_matches!(result, Err(e) if e.to_string().contains("Point must be made with exactly two values"));
    }

    #[test]
    fn test_parse_puzzle_input_three_numbers() {
        let result = parse_puzzle_input("162,25,812");
        assert_matches!(result, Err(e) if e.to_string().contains("Point must be made with exactly two values"));
    }
}
