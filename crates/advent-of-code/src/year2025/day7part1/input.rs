use super::types::{TachyonManifoldPart, parse_puzzle_input};
use anyhow::Result;

pub(super) fn get_input() -> Result<Vec<Vec<TachyonManifoldPart>>> {
    let input_str = include_str!("input.txt");
    parse_puzzle_input(input_str)
}
