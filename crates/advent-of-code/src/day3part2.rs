mod input;

use anyhow::{Result, anyhow};

use input::get_input;
use shared_utils::CheckedSum;

fn optimal_batteries_in_bank<T: Ord + Copy, const N: usize>(bank: &[T]) -> Option<[T; N]> {
    const {
        assert!(
            N >= 1 && std::mem::size_of::<T>() * N <= 1024,
            "N must be >= 1, and total array size must be <= 1024 bytes."
        )
    }

    if bank.len() < N {
        return None;
    }

    let mut candidates = bank;

    Some(std::array::from_fn(|n| {
        let mut max = candidates[0];
        let mut max_idx = 0_usize;
        let remaining = N - n - 1;
        for (idx, &val) in (1..).zip(&candidates[1..candidates.len() - remaining]) {
            if val > max {
                max = val;
                max_idx = idx;
            }
        }

        candidates = &candidates[max_idx + 1..];

        max
    }))
}

fn compute_max_joltage(bank_str: &str) -> Result<u64> {
    let bank_bits = bank_str.as_bytes();
    if bank_bits.iter().any(|chr| !chr.is_ascii_digit()) {
        return Err(anyhow!(
            "Can't compute joltage from string with non-ascii-digit characters."
        ));
    }

    let arr = optimal_batteries_in_bank::<_, 12>(bank_bits).ok_or_else(|| {
        anyhow!("Couldn't compute optimal batteries for bank str: \"{bank_str}\"")
    })?;

    Ok(arr
        .iter()
        .fold(0, |acc, next| acc * 10 + u64::from(*next - b'0')))
}

fn compute_max_joltage_sum(banks: &[&str]) -> Result<u64> {
    banks.iter().copied().map(compute_max_joltage).checked_sum()
}

pub(crate) fn compute() -> Result<u64> {
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
            assert_eq!(result, Some(*b"98"));
        }

        {
            let result = optimal_batteries_in_bank("811111111111119".as_bytes());
            assert_eq!(result, Some(*b"89"));
        }

        {
            let result = optimal_batteries_in_bank("234234234234278".as_bytes());
            assert_eq!(result, Some(*b"78"));
        }

        {
            let result = optimal_batteries_in_bank("818181911112111".as_bytes());
            assert_eq!(result, Some(*b"92"));
        }

        {
            let result = optimal_batteries_in_bank("987654321111111".as_bytes());
            assert_eq!(result, Some(*b"987654321111"));
        }

        {
            let result = optimal_batteries_in_bank("811111111111119".as_bytes());
            assert_eq!(result, Some(*b"811111111119"));
        }

        {
            let result = optimal_batteries_in_bank("234234234234278".as_bytes());
            assert_eq!(result, Some(*b"434234234278"));
        }

        {
            let result = optimal_batteries_in_bank("818181911112111".as_bytes());
            assert_eq!(result, Some(*b"888911112111"));
        }

        {
            let result = optimal_batteries_in_bank("2468".as_bytes());
            assert_eq!(result, Some(*b"8"));
        }

        {
            let result = optimal_batteries_in_bank("3965".as_bytes());
            assert_eq!(result, Some(*b"3965"));
        }

        {
            let result = optimal_batteries_in_bank(&[1, 6, 2, 8, 2]);
            assert_eq!(result, Some([8, 2]));
        }

        {
            let result = optimal_batteries_in_bank::<_, 2>("5".as_bytes());
            assert_eq!(result, None);
        }

        {
            let result = optimal_batteries_in_bank::<_, 50>("234234234234278".as_bytes());
            assert_eq!(result, None);
        }
    }

    #[test]
    fn test_compute_max_joltage_produces_correct_results() {
        assert_eq!(
            compute_max_joltage("987654321111111").unwrap(),
            987654321111_u64
        );
        assert_eq!(
            compute_max_joltage("811111111111119").unwrap(),
            811111111119_u64
        );
        assert_eq!(
            compute_max_joltage("881111111111119").unwrap(),
            881111111119_u64
        );
        assert_eq!(
            compute_max_joltage("234234234234278").unwrap(),
            434234234278_u64
        );
        assert_eq!(
            compute_max_joltage("818181911112111").unwrap(),
            888911112111_u64
        );
    }

    #[test]
    fn test_compute_max_joltage_sum_sample() {
        let banks: [&str; _] = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        assert_eq!(compute_max_joltage_sum(&banks).unwrap(), 3121910778619_u64);
    }
}
