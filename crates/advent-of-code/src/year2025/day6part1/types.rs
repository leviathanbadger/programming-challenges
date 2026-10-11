use std::str::FromStr;

use anyhow::{Result, anyhow};
use num_traits::{CheckedAdd, CheckedMul, One, Zero};
use shared_utils::CheckedSum;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum MathOperation {
    Multiplication,
    Addition,
}

impl MathOperation {
    fn perform<T>(&self, values: &[T]) -> Result<T>
    where
        T: CheckedAdd + CheckedMul + Zero + One + Copy,
    {
        match *self {
            MathOperation::Addition => values.iter().copied().checked_sum(),
            MathOperation::Multiplication => values
                .iter()
                .try_fold(T::one(), |accum, next| accum.checked_mul(next))
                .ok_or_else(|| anyhow!("Overflow occurred multiplying all numbers")),
        }
    }
}

impl FromStr for MathOperation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(anyhow!("Can't parse MathOperation from &str: \"{s}\""));
        }
        let bit = s.as_bytes()[0];
        match bit {
            b'*' => Ok(MathOperation::Multiplication),
            b'+' => Ok(MathOperation::Addition),
            _ => Err(anyhow!("Can't parse MathOperation from u8: {bit}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct MathProblem {
    numbers: Vec<u64>,
    op: MathOperation,
}

impl MathProblem {
    pub fn solve(&self) -> Result<u64> {
        self.op.perform(&self.numbers)
    }
}

pub(super) fn parse_puzzle_input(text: &str) -> Result<Vec<MathProblem>> {
    let lines: Vec<_> = text.trim().lines().collect();
    if lines.len() <= 2 {
        return Err(anyhow!(
            "Could not parse input. There must be at least one number line and exactly one op line"
        ));
    }

    let numbers_lines = &lines[..lines.len() - 1];
    let ops_line = lines[lines.len() - 1];

    let numbers = numbers_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n_str| n_str.parse::<u64>().map_err(|err| anyhow!(err)))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<Vec<_>>>>()?;

    if numbers.iter().any(|line| line.len() != numbers[0].len()) {
        return Err(anyhow!(
            "Could not parse input. All numbers lines must contain the same number of numbers."
        ));
    }

    let transposed_numbers = (0..numbers[0].len())
        .map(|col| numbers.iter().map(|n| n[col]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ops = ops_line
        .split_whitespace()
        .map(FromStr::from_str)
        .collect::<Result<Vec<MathOperation>>>()?;

    if ops.len() != numbers[0].len() {
        return Err(anyhow!(
            "Could not parse input. Op line must contain the same number of operations as numbers in each of the numbers lines."
        ));
    }

    let zipped = transposed_numbers
        .into_iter()
        .zip(ops)
        .map(|(numbers, op)| MathProblem { numbers, op })
        .collect::<Vec<_>>();

    debug_assert_eq!(zipped.len(), numbers[0].len());
    debug_assert_eq!(zipped[0].numbers.len(), numbers.len());

    Ok(zipped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_puzzle_input_transpose_numbers_and_solve() {
        let problems = parse_puzzle_input(
            "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ",
        )
        .unwrap();

        assert_eq!(problems.len(), 4);
        assert_eq!(
            problems[0],
            MathProblem {
                op: MathOperation::Multiplication,
                numbers: vec![123, 45, 6]
            }
        );
        assert_eq!(
            problems[1],
            MathProblem {
                op: MathOperation::Addition,
                numbers: vec![328, 64, 98]
            }
        );
        assert_eq!(
            problems[2],
            MathProblem {
                op: MathOperation::Multiplication,
                numbers: vec![51, 387, 215]
            }
        );
        assert_eq!(
            problems[3],
            MathProblem {
                op: MathOperation::Addition,
                numbers: vec![64, 23, 314]
            }
        );

        assert_eq!(problems[0].solve().unwrap(), 33210);
        assert_eq!(problems[1].solve().unwrap(), 490);
        assert_eq!(problems[2].solve().unwrap(), 4243455);
        assert_eq!(problems[3].solve().unwrap(), 401);
    }
}
