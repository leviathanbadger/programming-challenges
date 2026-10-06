use anyhow::anyhow;
use num_traits::{CheckedAdd, Zero};

/// A trait to provide a checked_sum operation that fails on overflow,
/// instead of panicking in debug and giving incorrect results in release.
pub trait CheckedSum<Kind>: Iterator {
    /// The type of the accumulated sum.
    ///
    /// For iterators over numeric values (with provided CheckedAdd implementations), this is the item type.
    /// For iterators over Results of numeric values, this is the result's success type,
    type Sum;

    /// Items are summed in iteration order, and fails fast with an anyhow Result::Err
    /// the first time adding a value causes an overflow. If all additions succeed,
    /// the sum is returned in a Result::Ok. The sum of an empty iterator is 0.
    ///
    /// When iterating over Results (instead of plain numeric values) the first Err
    /// is converted to an anyhow Err and passed back instead. No additional items are
    /// consumed.
    fn checked_sum(self) -> anyhow::Result<Self::Sum>;
}

pub struct Plain;
impl<I> CheckedSum<Plain> for I
where
    I: Iterator<Item: CheckedAdd + Zero>,
{
    type Sum = Self::Item;

    fn checked_sum(mut self) -> anyhow::Result<Self::Sum> {
        self.try_fold(Self::Sum::zero(), |acc, next| {
            acc.checked_add(&next)
                .ok_or_else(|| anyhow!("Overflow occurred summing values"))
        })
    }
}

pub struct Fallible;
impl<I, V, E> CheckedSum<Fallible> for I
where
    I: Iterator<Item = Result<V, E>>,
    V: CheckedAdd + Zero,
    E: Into<anyhow::Error>,
{
    type Sum = V;

    fn checked_sum(mut self) -> anyhow::Result<Self::Sum> {
        self.try_fold(Self::Sum::zero(), |acc, next_result| {
            let next = next_result.map_err(Into::into)?;
            acc.checked_add(&next)
                .ok_or_else(|| anyhow!("Overflow occurred summing values"))
        })
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

    #[test]
    fn test_sum_results_success() {
        let nums = (1..=5).map(Result::<_, anyhow::Error>::Ok);

        let result = nums.into_iter().checked_sum();
        assert_matches!(result, Ok(15));
    }

    #[test]
    fn test_sum_results_error() {
        let nums = (1..).map(|n| {
            if n == 4 {
                Err(anyhow!("EEEEVIL!"))
            } else {
                Ok(n)
            }
        });

        let result = nums.into_iter().checked_sum();
        assert_matches!(result, Err(err) if err.to_string().eq("EEEEVIL!"));
        // Also tests that the function short-circuits because the iterator is otherwise infinite
    }

    #[test]
    fn test_sum_results_empty() {
        let nums = (1..1).map(Result::<u8, anyhow::Error>::Ok);

        let sum = nums.into_iter().checked_sum().unwrap();
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_sum_results_error_overflow() {
        let nums = (1..).map(Result::<u8, anyhow::Error>::Ok);

        let result = nums.into_iter().checked_sum();
        assert_matches!(result, Err(err) if err.to_string().eq("Overflow occurred summing values"));
        // Also tests that the function short-circuits because the iterator is otherwise infinite
    }
}
