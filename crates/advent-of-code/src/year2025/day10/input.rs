use anyhow::Result;

use super::machine_spec::{MachineSpec, parse_machine_spec};

pub(super) fn get_input() -> Result<Vec<MachineSpec>> {
    let input_str = include_str!("input.txt");
    parse_puzzle_input(input_str)
}

pub(super) fn parse_puzzle_input(text: &str) -> Result<Vec<MachineSpec>> {
    text.lines().map(parse_machine_spec).collect()
}
