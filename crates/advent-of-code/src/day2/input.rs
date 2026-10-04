use super::types::{DialRotation, parse_rotations};
use anyhow::Result;

pub fn get_input() -> Result<Vec<DialRotation>> {
    parse_rotations(include_str!("input.txt"))
}
