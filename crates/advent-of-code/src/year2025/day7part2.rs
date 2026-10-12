mod input;
mod types;

use anyhow::{Result, anyhow};
use input::get_input;
use shared_utils::CheckedSum;
use types::TachyonManifoldPart;

macro_rules! checked_add_timelines {
    ( $mutval:expr ; $firstval:expr $(, $val:expr)* ) => {
        {
            let mutref = &mut $mutval;
            *mutref = (*mutref).checked_add($firstval)
                $( .and_then(|accum| accum.checked_add($val)) )*
                .ok_or_else(|| anyhow!("Overflow adding timelines"))?;
        }
    };
}

/// Assumptions: all input lines are the same length, no input lines start or end with a splitter
fn count_timelines(input: &[Vec<TachyonManifoldPart>]) -> Result<u64> {
    if input.is_empty() {
        return Ok(0);
    }

    let mut prev = vec![0u64; input[0].len()];
    let mut next = vec![0u64; input[0].len()];

    for line in input {
        for (idx, part) in line.iter().enumerate() {
            match *part {
                TachyonManifoldPart::Empty => {
                    checked_add_timelines!(next[idx]; prev[idx]);
                }
                TachyonManifoldPart::Start => {
                    checked_add_timelines!(next[idx]; prev[idx], 1);
                }
                TachyonManifoldPart::Splitter => {
                    checked_add_timelines!(next[idx - 1]; prev[idx]);
                    checked_add_timelines!(next[idx + 1]; prev[idx]);
                }
            }
        }

        std::mem::swap(&mut prev, &mut next);
        next.fill(0);
    }

    prev.into_iter().checked_sum()
}

pub(super) fn compute() -> Result<u64> {
    let puzzle_input = get_input()?;
    count_timelines(&puzzle_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::types::parse_puzzle_input;

    #[test]
    fn test_sample() {
        let puzzle_input = parse_puzzle_input(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        )
        .unwrap();

        let count = count_timelines(&puzzle_input).unwrap();
        assert_eq!(count, 40);
    }
}
