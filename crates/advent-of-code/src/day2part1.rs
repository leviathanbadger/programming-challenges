mod input;
mod types;

use anyhow::{Result, anyhow};
use input::get_input;

use crate::day2part1::types::{CombineOverlapsExt, IdRange};

// Assuming u8 slice instead of &str to avoid unicode shenanigans, because input should theoretically only be ascii digits
fn is_valid_id_bits(bits: &[u8]) -> bool {
    if bits.len() % 2 == 1 {
        return true;
    }

    let sequence_len = bits.len() / 2;
    let start = &bits[0..sequence_len];
    let end = &bits[sequence_len..];

    start != end
}
fn is_valid_id(num: u64) -> bool {
    is_valid_id_bits(num.to_string().as_bytes())
}

fn sum_invalid_ids(ranges: &[IdRange]) -> Result<u64> {
    ranges
        .iter()
        .copied()
        .combine_overlaps()
        .flatten()
        .filter(|id| !is_valid_id(*id)) //TODO MAYBE: use shared memory location for all of the number->string conversions to avoid heap fragmentation? Is it worth it?
        .try_fold(0_u64, |acc, next| acc.checked_add(next))
        .ok_or_else(|| anyhow!("Sum caused overflow"))
}

pub(crate) fn compute() -> Result<u64> {
    let input = get_input()?;

    sum_invalid_ids(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::types::parse_id_ranges;

    #[test]
    fn test_is_valid_id_valid() {
        let valid_ids: [u64; _] = [7, 13, 19, 95, 115, 1012, 222220, 123123123, 1188511890];

        for id in valid_ids {
            assert!(is_valid_id(id), "id: {id}");
        }
    }

    #[test]
    fn test_is_valid_id_invalid() {
        let invalid_ids: [u64; _] = [11, 22, 88, 1010, 1111, 222222, 336336, 99259925];

        for id in invalid_ids {
            assert!(!is_valid_id(id), "id: {id}");
        }
    }

    #[test]
    fn test_sample_id_ranges() {
        let input = parse_id_ranges(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        ).unwrap();

        let sum = sum_invalid_ids(&input).unwrap();
        assert_eq!(sum, 1227775554);
    }
}
