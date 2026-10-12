use anyhow::{Result, anyhow};
use num_traits::Zero;
use std::{
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Point3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Add for Point3d {
    type Output = Point3d;

    fn add(self, rhs: Self) -> Self::Output {
        Point3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point3d {
    type Output = Point3d;

    fn sub(self, rhs: Self) -> Self::Output {
        Point3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Zero for Point3d {
    fn zero() -> Self {
        Point3d { x: 0, y: 0, z: 0 }
    }

    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0
    }
}

impl FromStr for Point3d {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(',').map(str::trim).map(i32::from_str);
        if let (Some(x), Some(y), Some(z), None) =
            (nums.next(), nums.next(), nums.next(), nums.next())
        {
            Ok(Point3d {
                x: x?,
                y: y?,
                z: z?,
            })
        } else {
            Err(anyhow!("Point3d must be made with exactly three values"))
        }
    }
}

pub(super) fn parse_puzzle_input(text: &str) -> Result<Vec<Point3d>> {
    text.lines().map(Point3d::from_str).collect()
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_parse_puzzle_input() {
        let coords = parse_puzzle_input(
            "162,817,812
             57 ,618  , -57
            906,360,560
            592, 479,940
            352,   342,300",
        )
        .unwrap();

        assert_eq!(coords.len(), 5);
        assert_eq!(
            coords[0],
            Point3d {
                x: 162,
                y: 817,
                z: 812
            }
        );
        assert_eq!(
            coords[1],
            Point3d {
                x: 57,
                y: 618,
                z: -57
            }
        );
        assert_eq!(
            coords[2],
            Point3d {
                x: 906,
                y: 360,
                z: 560
            }
        );
        assert_eq!(
            coords[3],
            Point3d {
                x: 592,
                y: 479,
                z: 940
            }
        );
        assert_eq!(
            coords[4],
            Point3d {
                x: 352,
                y: 342,
                z: 300
            }
        );
    }

    #[test]
    fn test_parse_puzzle_input_invalid_number() {
        let result = parse_puzzle_input("162,i,812");
        assert_matches!(result, Err(e) if e.to_string().contains("invalid digit"));
    }

    #[test]
    fn test_parse_puzzle_input_two_numbers() {
        let result = parse_puzzle_input("162,812");
        assert_matches!(result, Err(e) if e.to_string().contains("Point3d must be made with exactly three values"));
    }

    #[test]
    fn test_parse_puzzle_input_four_numbers() {
        let result = parse_puzzle_input("162,25,812,89");
        assert_matches!(result, Err(e) if e.to_string().contains("Point3d must be made with exactly three values"));
    }
}
