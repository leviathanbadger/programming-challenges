use std::{str::FromStr, vec};

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
        match s {
            "*" => Ok(MathOperation::Multiplication),
            "+" => Ok(MathOperation::Addition),
            _ => Err(anyhow!("Can't parse MathOperation from &str: \"{s}\"")),
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

fn parse_number_vertical(lines: &[&str], col: usize) -> Option<u64> {
    let mut found_digit = false;
    let mut accum = 0u64;

    for bit_opt in lines.iter().map(|line| line.as_bytes().get(col)) {
        if let Some(bit) = bit_opt
            && bit.is_ascii_digit()
        {
            found_digit = true;
            accum *= 10;
            accum += u64::from(bit - b'0');
        }
    }

    if found_digit { Some(accum) } else { None }
}

fn window_option<I, T>(iter: I) -> Vec<Vec<T>>
where
    I: Iterator<Item = Option<T>>,
{
    let mut all = vec![];
    let mut curr = vec![];

    for item in iter {
        match item {
            None => {
                if !curr.is_empty() {
                    all.push(std::mem::take(&mut curr));
                }
            }
            Some(val) => {
                curr.push(val);
            }
        }
    }

    if !curr.is_empty() {
        all.push(curr);
    }

    all
}

pub(super) fn parse_puzzle_input(text: &str) -> Result<Vec<MathProblem>> {
    let lines: Vec<_> = text.trim_end().lines().collect();
    if lines.len() < 2 {
        return Err(anyhow!(
            "Could not parse input. There must be at least one number line and exactly one op line"
        ));
    }

    let numbers_lines = &lines[..lines.len() - 1];
    let ops_line = lines[lines.len() - 1];

    let max_width = numbers_lines
        .iter()
        .map(|line| line.len())
        .max()
        .ok_or_else(|| anyhow!("Could not get max width for parsing vertical numbers"))?;
    let all_numbers = (0..max_width).map(|col| parse_number_vertical(numbers_lines, col));
    let windowed_numbers = window_option(all_numbers);

    let ops = ops_line
        .split_whitespace()
        .map(FromStr::from_str)
        .collect::<Result<Vec<MathOperation>>>()?;

    if ops.len() != windowed_numbers.len() {
        return Err(anyhow!(
            "Could not parse input. Op line must contain the same number of operations as groups of vertical lines."
        ));
    }

    let zipped = windowed_numbers
        .into_iter()
        .zip(ops)
        .map(|(numbers, op)| MathProblem { numbers, op })
        .collect::<Vec<_>>();

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
                numbers: vec![1, 24, 356]
            }
        );
        assert_eq!(
            problems[1],
            MathProblem {
                op: MathOperation::Addition,
                numbers: vec![369, 248, 8]
            }
        );
        assert_eq!(
            problems[2],
            MathProblem {
                op: MathOperation::Multiplication,
                numbers: vec![32, 581, 175]
            }
        );
        assert_eq!(
            problems[3],
            MathProblem {
                op: MathOperation::Addition,
                numbers: vec![623, 431, 4]
            }
        );

        assert_eq!(problems[0].solve().unwrap(), 8544);
        assert_eq!(problems[1].solve().unwrap(), 625);
        assert_eq!(problems[2].solve().unwrap(), 3253600);
        assert_eq!(problems[3].solve().unwrap(), 1058);
    }
}
