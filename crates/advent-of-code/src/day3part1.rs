mod input;

use anyhow::{Result, anyhow};

use input::get_input;
use shared_utils::CheckedSum;

fn optimal_batteries_in_bank<T: Ord + Copy>(bank: &[T]) -> Option<(T, T)> {
    if bank.len() < 2 {
        return None;
    }

    let mut max = bank[0];
    let mut max_idx = 0_usize;
    for (idx, &val) in bank.iter().enumerate().skip(1).take(bank.len() - 2) {
        if val > max {
            max = val;
            max_idx = idx;
        }
    }

    let max_after = *bank[(max_idx + 1)..].iter().max()?;

    Some((max, max_after))
}

fn compute_max_joltage(bank_str: &str) -> Result<u32> {
    let bank_bits = bank_str.as_bytes();
    if bank_bits.iter().any(|chr| !chr.is_ascii_digit()) {
        return Err(anyhow!(
            "Can't compute joltage from string with non-ascii-digit characters."
        ));
    }

    let (first, last) = optimal_batteries_in_bank(bank_bits).ok_or_else(|| {
        anyhow!("Couldn't compute optimal batteries for bank str: \"{bank_str}\"")
    })?;

    Ok(u32::from((first - b'0') * 10 + (last - b'0')))
}

fn compute_max_joltage_sum(banks: &[&str]) -> Result<u32> {
    banks
        .iter()
        .copied()
        .map(compute_max_joltage)
        .collect::<Result<Vec<_>>>()? //TODO MAYBE: add variant to checked_sum that allows iterating over Result or Option, to avoid collecting
        .into_iter()
        .checked_sum()
}

pub(crate) fn compute() -> Result<u32> {
    let banks = get_input();
    compute_max_joltage_sum(&banks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimal_batteries_in_bank_produces_correct_results() {
        {
            let result = optimal_batteries_in_bank("987654321111111".as_bytes());
            assert_eq!(result, Some((b'9', b'8')));
        }

        {
            let result = optimal_batteries_in_bank("811111111111119".as_bytes());
            assert_eq!(result, Some((b'8', b'9')));
        }

        {
            let result = optimal_batteries_in_bank("234234234234278".as_bytes());
            assert_eq!(result, Some((b'7', b'8')));
        }

        {
            let result = optimal_batteries_in_bank("818181911112111".as_bytes());
            assert_eq!(result, Some((b'9', b'2')));
        }

        {
            let result = optimal_batteries_in_bank(&[1, 6, 2, 8, 2]);
            assert_eq!(result, Some((8, 2)));
        }

        {
            let result = optimal_batteries_in_bank("5".as_bytes());
            assert_eq!(result, None);
        }
    }

    #[test]
    fn test_compute_max_joltage_produces_correct_results() {
        assert_eq!(compute_max_joltage("987654321111111").unwrap(), 98);
        assert_eq!(compute_max_joltage("811111111111119").unwrap(), 89);
        assert_eq!(compute_max_joltage("234234234234278").unwrap(), 78);
        assert_eq!(compute_max_joltage("818181911112111").unwrap(), 92);
    }

    #[test]
    fn test_compute_max_joltage_sum_sample() {
        let banks: [&str; _] = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        assert_eq!(compute_max_joltage_sum(&banks).unwrap(), 357);
    }
}
