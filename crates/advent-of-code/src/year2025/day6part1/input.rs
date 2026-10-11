use super::types::{MathProblem, parse_puzzle_input};
use anyhow::Result;

pub(super) fn get_input() -> Result<Vec<MathProblem>> {
    let input_str = include_str!("input.txt");
    parse_puzzle_input(input_str)
}
