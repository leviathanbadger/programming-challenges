use std::iter::Peekable;

use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IdRange {
    start: u64,
    end: u64,
}

impl IntoIterator for IdRange {
    type Item = u64;
    type IntoIter = std::ops::RangeInclusive<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

pub(crate) struct CombineOverlaps {
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

pub(crate) trait CombineOverlapsExt: Iterator + Sized {
    fn combine_overlaps(self) -> CombineOverlaps;
}

impl<I> CombineOverlapsExt for I
where
    I: Iterator<Item = IdRange>,
{
    fn combine_overlaps(self) -> CombineOverlaps {
        let mut items = self.collect::<Vec<_>>();
        items.sort_by_key(|item| item.start);
        CombineOverlaps {
            iter: items.into_iter().peekable(),
        }
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

pub(crate) fn parse_id_ranges(text: &str) -> Result<Vec<IdRange>> {
    text.split(',')
        .map(|text| text.trim())
        .map(parse_id_range)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_overlaps_flatten_overlapping_id_ranges() {
        let input =
            parse_id_ranges("82-85,5-6,3-10,20-25,17-22,50-55,53-59,58-62,70-75,75-80,82-85")
                .unwrap();
        assert_eq!(input.len(), 11);

        let flattened = input.into_iter().combine_overlaps().collect::<Vec<_>>();
        let [first, second, third, fourth, fifth]: [IdRange; _] =
            flattened.try_into().expect("Expected exactly 5 IdRanges");

        assert_eq!(first, IdRange { start: 3, end: 10 });
        assert_eq!(second, IdRange { start: 17, end: 25 });
        assert_eq!(third, IdRange { start: 50, end: 62 });
        assert_eq!(fourth, IdRange { start: 70, end: 80 });
        assert_eq!(fifth, IdRange { start: 82, end: 85 });
    }

    #[test]
    fn test_combine_overlaps_empty_iter() {
        let input: [IdRange; _] = [];

        let flattened = input.into_iter().combine_overlaps().collect::<Vec<_>>();
        assert_eq!(flattened.len(), 0);
    }

    #[test]
    fn test_combine_overlaps_single_iter() {
        let single = IdRange { start: 25, end: 50 };
        let input: [IdRange; _] = [single];

        let flattened = input.into_iter().combine_overlaps().collect::<Vec<_>>();
        assert_eq!(flattened.len(), 1);
        assert_eq!(flattened[0], single);
    }
}
