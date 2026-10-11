mod input;
mod types;

use input::get_input;

use anyhow::{Result, anyhow};
use shared_utils::CheckedSum;
use types::IdRange;

fn count_fresh_ingredients(fresh_id_ranges: &[IdRange]) -> Result<u64> {
    fresh_id_ranges
        .iter()
        .map(|range| range.end - range.start)
        .checked_sum()?
        .checked_add(fresh_id_ranges.len() as u64) // Ranges are end-inclusive; adjusting sum here so I don't have to do it incrementally for each range in the map
        .ok_or_else(|| anyhow!("Fresh ID count exceeds u64"))
}

pub fn compute() -> Result<u64> {
    let fresh_id_ranges = get_input()?;
    count_fresh_ingredients(&fresh_id_ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::types::parse_puzzle_input;

    #[test]
    fn test_count_fresh_ingredients_sample() {
        let ranges = parse_puzzle_input(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        )
        .unwrap();

        let count = count_fresh_ingredients(&ranges).unwrap();
        assert_eq!(count, 14);
    }
}
