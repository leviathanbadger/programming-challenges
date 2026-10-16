use anyhow::anyhow;
use num_traits::{CheckedAdd, Zero};

/// A trait to provide a checked_sum operation that fails on overflow,
/// instead of panicking in debug and giving incorrect results in release.
pub trait CheckedSum<Kind>: Iterator {
    /// The type of the accumulated sum.
    ///
    /// For iterators over numeric values (with provided CheckedAdd implementations), this is the item type.
    /// For iterators over Results of numeric values, this is the result's success type,
    /// For iterators over Options of numeric values, this is the option's Some type.
    type Sum;
    /// The return type. In most cases, anyhow::Result<Self::Sum>. When the iterator item
    /// is an Option, this will be Option<Self::Sum>.
    type Output;

    /// Items are summed in iteration order, and fails fast with an anyhow Result::Err (or None if iterating over Options)
    /// the first time adding a value causes an overflow. If all additions succeed,
    /// the sum is returned in a Result::Ok (or Some). The sum of an empty iterator is 0.
    ///
    /// When iterating over Results (instead of plain numeric values) the first Err
    /// is converted to an anyhow Err and passed back instead. No additional items are
    /// consumed.
    fn checked_sum(self) -> Self::Output;
}

pub struct Plain;
impl<I> CheckedSum<Plain> for I
where
    I: Iterator<Item: CheckedAdd + Zero>,
{
    type Sum = Self::Item;
    type Output = anyhow::Result<Self::Sum>;

    fn checked_sum(mut self) -> Self::Output {
        self.try_fold(Self::Sum::zero(), |acc, next| {
            acc.checked_add(&next)
                .ok_or_else(|| anyhow!("Overflow occurred summing values"))
        })
    }
}

pub struct FallibleResult;
impl<I, V, E> CheckedSum<FallibleResult> for I
where
    I: Iterator<Item = Result<V, E>>,
    V: CheckedAdd + Zero,
    E: Into<anyhow::Error>,
{
    type Sum = V;
    type Output = anyhow::Result<Self::Sum>;

    fn checked_sum(mut self) -> Self::Output {
        self.try_fold(Self::Sum::zero(), |acc, next_result| {
            let next = next_result.map_err(Into::into)?;
            acc.checked_add(&next)
                .ok_or_else(|| anyhow!("Overflow occurred summing values"))
        })
    }
}

pub struct FallibleOption;
impl<I, V> CheckedSum<FallibleOption> for I
where
    I: Iterator<Item = Option<V>>,
    V: CheckedAdd + Zero,
{
    type Sum = V;
    type Output = Option<Self::Sum>;

    fn checked_sum(mut self) -> Self::Output {
        self.try_fold(Self::Sum::zero(), |acc, next_opt| {
            next_opt.and_then(|next| acc.checked_add(&next))
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

    #[test]
    fn test_sum_options_success() {
        let nums = (1..=5).map(Some);

        let sum = nums.into_iter().checked_sum();
        assert_matches!(sum, Some(15));
    }

    #[test]
    fn test_sum_options_error() {
        let nums = (1..).map(|n| if n == 4 { None } else { Some(n) });

        let sum = nums.into_iter().checked_sum();
        assert_matches!(sum, None);
        // Also tests that the function short-circuits because the iterator is otherwise infinite
    }

    #[test]
    fn test_sum_options_empty() {
        let nums = (1..1).map(Some);

        let sum = nums.into_iter().checked_sum().unwrap();
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_sum_options_error_overflow() {
        let nums = (1..).map(Some);

        let result = nums.into_iter().checked_sum();
        assert_matches!(result, None);
        // Also tests that the function short-circuits because the iterator is otherwise infinite
    }
}
