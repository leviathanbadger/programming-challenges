use super::point::{Point3d, parse_puzzle_input};
use anyhow::Result;

pub(super) fn get_input() -> Result<Vec<Point3d>> {
    let input_str = include_str!("input.txt");
    parse_puzzle_input(input_str)
}
