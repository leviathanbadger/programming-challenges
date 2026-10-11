use std::iter::Peekable;

use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct IdRange {
    pub start: u64,
    pub end: u64,
}

struct CombineOverlaps {
    iter: Peekable<std::vec::IntoIter<IdRange>>,
}

impl Iterator for CombineOverlaps {
    type Item = IdRange;

    fn next(&mut self) -> Option<Self::Item> {
        let mut item = self.iter.next()?;

        while let Some(next_item) = self.iter.next_if(|&n| n.start <= item.end) {
            item.end = u64::max(item.end, next_item.end);
        }

        Some(item)
    }
}

fn parse_id_range(text: &str) -> Result<IdRange> {
    let (first, second) = text.split_once('-').context("Expected <start>-<end>")?;

    let start = first.parse::<u64>()?;
    let end = second.parse::<u64>()?;

    if start > end {
        Err(anyhow!(
            "start ({start}) must be less than or equal to end ({end})"
        ))
    } else {
        Ok(IdRange { start, end })
    }
}

pub(super) fn parse_id_ranges(text: &str) -> Result<Vec<IdRange>> {
    let mut raw = text
        .split_whitespace()
        .map(parse_id_range)
        .collect::<Result<Vec<_>>>()?;
    raw.sort_by_key(|item| item.start);

    Ok(CombineOverlaps {
        iter: raw.into_iter().peekable(),
    }
    .collect())
}

fn parse_ids(text: &str) -> Result<Vec<u64>> {
    text.split_whitespace()
        .map(|s| s.parse::<u64>().map_err(|err| anyhow!(err)))
        .collect()
}

pub(super) fn parse_puzzle_input(text: &str) -> Result<(Vec<IdRange>, Vec<u64>)> {
    let (id_ranges_str, ids_str) = text
        .split_once("\r\n\r\n")
        .or_else(|| text.split_once("\n\n"))
        .ok_or_else(|| anyhow!("Could not split puzzle input into id ranges and ids str"))?;

    Ok((parse_id_ranges(id_ranges_str)?, parse_ids(ids_str)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_id_ranges_sort_flatten_overlapping_id_ranges() {
        let ranges =
            parse_id_ranges("82-85 5-6 3-10 20-25 17-22 50-55 53-59 58-62 70-75 75-80 82-85")
                .unwrap();

        let [first, second, third, fourth, fifth]: [IdRange; _] =
            ranges.try_into().expect("Expected exactly 5 IdRanges");

        assert_eq!(first, IdRange { start: 3, end: 10 });
        assert_eq!(second, IdRange { start: 17, end: 25 });
        assert_eq!(third, IdRange { start: 50, end: 62 });
        assert_eq!(fourth, IdRange { start: 70, end: 80 });
        assert_eq!(fifth, IdRange { start: 82, end: 85 });
    }

    #[test]
    fn test_parse_id_ranges_empty_iter() {
        let ranges = parse_id_ranges("").unwrap();
        assert_eq!(ranges.len(), 0);
    }

    #[test]
    fn test_parse_id_ranges_single_iter() {
        let ranges = parse_id_ranges("25-50").unwrap();

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], IdRange { start: 25, end: 50 });
    }
}
