use anyhow::{Result, anyhow};
use num_traits::CheckedAdd;

pub trait CheckedSum: Iterator {
    fn checked_sum(self) -> Result<Self::Item>;
}

impl<I> CheckedSum for I
where
    I: Iterator<Item: CheckedAdd + Default>,
{
    fn checked_sum(mut self) -> Result<Self::Item> {
        self.try_fold(Self::Item::default(), |acc, next| acc.checked_add(&next))
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
