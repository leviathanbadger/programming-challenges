mod input;
mod types;

use input::get_input;

use anyhow::Result;
use types::IdRange;

/// Checks if the specified id is in the passed-in ID ranges. Assumes fresh_id_ranges is presorted by IdRange#start.
fn is_ingredient_fresh(id: u64, fresh_id_ranges: &[IdRange]) -> bool {
    let result = fresh_id_ranges.binary_search_by_key(&id, |range| range.start);
    match result {
        Ok(_) => true,
        Err(idx) => {
            if idx == 0 {
                false
            } else {
                fresh_id_ranges[idx - 1].end >= id
            }
        }
    }
}

fn count_fresh_ingredients(fresh_id_ranges: &[IdRange], all_ingredient_ids: &[u64]) -> usize {
    all_ingredient_ids
        .iter()
        .filter(|id| is_ingredient_fresh(**id, fresh_id_ranges))
        .count()
}

pub fn compute() -> Result<usize> {
    let (fresh_id_ranges, available_ids) = get_input()?;
    Ok(count_fresh_ingredients(&fresh_id_ranges, &available_ids))
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::types::{parse_id_ranges, parse_puzzle_input};

    #[test]
    fn test_count_fresh_ingredients_sample() {
        let (ranges, all_ids) = parse_puzzle_input(
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

        let count = count_fresh_ingredients(&ranges, &all_ids);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_is_ingredient_fresh() {
        let fresh_id_ranges = parse_id_ranges("3-5 10-14 16-20 12-18").unwrap();
        assert_eq!(fresh_id_ranges.len(), 2);

        assert!(!is_ingredient_fresh(2, &fresh_id_ranges));
        assert!(is_ingredient_fresh(3, &fresh_id_ranges));
        assert!(is_ingredient_fresh(5, &fresh_id_ranges));
        assert!(!is_ingredient_fresh(6, &fresh_id_ranges));
        assert!(!is_ingredient_fresh(9, &fresh_id_ranges));
        assert!(is_ingredient_fresh(10, &fresh_id_ranges));
        assert!(is_ingredient_fresh(18, &fresh_id_ranges));
        assert!(is_ingredient_fresh(20, &fresh_id_ranges));
        assert!(!is_ingredient_fresh(21, &fresh_id_ranges));
    }
}
