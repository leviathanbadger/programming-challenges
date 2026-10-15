use anyhow::{Result, anyhow};
use num_traits::Zero;
use std::{
    ops::{Add, Sub},
    str::FromStr,
};

/// Represents a 2D point in space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Point {
    pub x: i32,
    pub y: i32,
}

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

macro_rules! point {
    ( $x:expr , $y:expr ) => {
        crate::year2025::day9::point::Point { x: ($x), y: ($y) }
    };
}

pub(super) use point;
