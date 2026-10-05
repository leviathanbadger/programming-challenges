use anyhow::{Result, anyhow};
use num_traits::{CheckedAdd, Zero};

/// A trait to provide a checked_sum operation that fails on overflow,
/// instead of panicking in debug and giving incorrect results in release.
pub trait CheckedSum: Iterator {
    /// Items are summed in iteration order, and fails fast with an anyhow Result::Err
    /// the first time adding a value causes an overflow. If all additions succeed,
    /// the sum is returned in a Result::Ok. The sum of an empty iterator is 0.
    fn checked_sum(self) -> Result<Self::Item>;
}

impl<I> CheckedSum for I
where
    I: Iterator<Item: CheckedAdd + Zero>,
{
    fn checked_sum(mut self) -> Result<Self::Item> {
        self.try_fold(Self::Item::zero(), |acc, next| acc.checked_add(&next))
            .ok_or_else(|| anyhow!("Overflow occurred summing values"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::assert_matches;

    #[test]
    fn test_sum_empty_i32() {
        let nums: [i32; _] = [];

        assert_eq!(nums.into_iter().checked_sum().unwrap(), 0);
    }

    #[test]
    fn test_sum_i32() {
        let nums: [i32; _] = [1, 2, 3, 4, 5];

        assert_eq!(nums.into_iter().checked_sum().unwrap(), 15);
    }

    #[test]
    fn test_sum_u8() {
        let nums: [u8; _] = [1, 2, 3, 4, 5];

        assert_eq!(nums.into_iter().checked_sum().unwrap(), 15_u8);
    }

    #[test]
    fn test_sum_overflow_u8() {
        let nums: [u8; _] = [225, 12, 91, 1];

        let result = nums.into_iter().checked_sum();
        assert_matches!(result, Err(err) if err.to_string().eq("Overflow occurred summing values"));
    }
}
