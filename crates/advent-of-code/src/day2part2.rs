mod input;
mod types;

use anyhow::Result;
use input::get_input;
use shared_utils::CheckedSum;
use types::{CombineOverlapsExt, IdRange};

// Using u8 slice instead of &str to avoid unicode shenanigans, because input should theoretically only be ascii digits
fn is_valid_id_bits(bits: &[u8]) -> bool {
    let has_repeated_section = (1..=(bits.len() / 2))
        .filter(|len| bits.len().is_multiple_of(*len))
        .map(|check| {
            let num_copies = bits.len() / check;
            (0..num_copies)
                .map(|idx| &bits[(idx * check)..][..check])
                .collect::<Vec<_>>()
        })
        .any(|check| {
            let (first, rest) = check.split_first().unwrap();
            rest.iter().all(|sec| sec.eq(first))
        });

    !has_repeated_section
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
        .checked_sum()
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
        let valid_ids: [u64; _] = [7, 13, 19, 95, 115, 1012, 222220, 1188511890];

        for id in valid_ids {
            assert!(is_valid_id(id), "id: {id}");
        }
    }

    #[test]
    fn test_is_valid_id_invalid() {
        let invalid_ids: [u64; _] = [
            11, 22, 88, 111, 1010, 1111, 222222, 336336, 99259925, 123123123, 2121212121,
        ];

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
        assert_eq!(sum, 4174379265);
    }
}
