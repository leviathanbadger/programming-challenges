mod input;
mod types;

use std::collections::HashSet;

use anyhow::Result;
use input::get_input;
use types::TachyonManifoldPart;

/// Assumptions: all input lines are the same length, no input lines start or end with a splitter
fn count_tachyon_splits(input: &[Vec<TachyonManifoldPart>]) -> u32 {
    let mut prev = HashSet::new();
    let mut next = HashSet::new();
    let mut split_count = 0;

    for line in input {
        for (idx, part) in line.iter().enumerate() {
            match *part {
                TachyonManifoldPart::Empty => {
                    if prev.contains(&idx) {
                        next.insert(idx);
                    }
                }
                TachyonManifoldPart::Start => {
                    next.insert(idx);
                }
                TachyonManifoldPart::Splitter => {
                    if prev.contains(&idx) {
                        split_count += 1;
                        next.insert(idx - 1);
                        next.insert(idx + 1);
                    }
                }
            }
        }

        std::mem::swap(&mut prev, &mut next);
        next.clear();
    }

    split_count
}

pub(super) fn compute() -> Result<u32> {
    let puzzle_input = get_input()?;
    Ok(count_tachyon_splits(&puzzle_input))
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

        let count = count_tachyon_splits(&puzzle_input);
        assert_eq!(count, 21);
    }
}
