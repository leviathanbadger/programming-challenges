use super::types::{IdRange, parse_id_ranges};
use anyhow::Result;

pub(crate) fn get_input() -> Result<Vec<IdRange>> {
    parse_id_ranges(include_str!("input.txt"))
}
