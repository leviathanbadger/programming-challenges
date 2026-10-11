use super::types::{IdRange, parse_puzzle_input};

use anyhow::Result;

pub(crate) fn get_input() -> Result<Vec<IdRange>> {
    let input_str = include_str!("input.txt");
    parse_puzzle_input(input_str)
}
