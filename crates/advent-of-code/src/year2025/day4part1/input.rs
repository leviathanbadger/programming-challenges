use super::types::{PrintingCell, SizedGrid, parse_printing_grid};
use anyhow::Result;

pub fn get_input() -> Result<SizedGrid<PrintingCell>> {
    let input_str = include_str!("input.txt");
    parse_printing_grid(input_str)
}
