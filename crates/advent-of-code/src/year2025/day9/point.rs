use anyhow::{Result, anyhow};
use num_traits::Zero;
use std::{
    ops::{Add, Sub},
    str::FromStr,
};

/// Represents a 2D point in space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[cfg(test)]
macro_rules! point {
    ( $x:expr , $y:expr ) => {
        crate::year2025::day9::point::Point { x: ($x), y: ($y) }
    };
}

#[cfg(test)]
pub(super) use point;

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Zero for Point {
    fn zero() -> Self {
        Point { x: 0, y: 0 }
    }

    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(',').map(str::trim).map(i32::from_str);
        if let (Some(x_result), Some(y_result), None) = (nums.next(), nums.next(), nums.next()) {
            let [x, y] = [x_result?, y_result?];
            Ok(Point { x, y })
        } else {
            Err(anyhow!("Point must be made with exactly two values"))
        }
    }
}

/// Parses the puzzle input - a sequence of 2D points. One point per line,
/// each point using comma-separated axes (like "x,y").
pub(super) fn parse_puzzle_input(text: &str) -> Result<Vec<Point>> {
    text.lines().map(Point::from_str).collect()
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

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
