mod input;
mod types;

use anyhow::Result;
use input::get_input;
use shared_utils::CheckedSum;
use types::TachyonManifoldPart;

/// Assumptions: all input lines are the same length, no input lines start or end with a splitter
fn count_timelines(input: &[Vec<TachyonManifoldPart>]) -> Result<u64> {
    let mut prev = vec![0u64; input[0].len()];
    let mut next = vec![0u64; input[0].len()];

    for line in input {
        for (idx, part) in line.iter().enumerate() {
            match *part {
                TachyonManifoldPart::Empty => {
                    next[idx] += prev[idx];
                }
                TachyonManifoldPart::Start => {
                    next[idx] += prev[idx] + 1;
                }
                TachyonManifoldPart::Splitter => {
                    next[idx - 1] += prev[idx];
                    next[idx + 1] += prev[idx];
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
