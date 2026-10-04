use std::iter::Peekable;

use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IdRange {
    start: u64,
    end: u64,
}

impl IntoIterator for IdRange {
    type Item = u64;
    type IntoIter = <std::ops::RangeInclusive<u64> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

// TODO: there has _got_ to be a better way to implement extension methods for iterators.
// Does Rust really not have any syntax sugar for this?
pub(crate) struct CombineOverlaps<TItem> {
    iter: Peekable<std::vec::IntoIter<TItem>>,
}

impl Iterator for CombineOverlaps<IdRange> {
    type Item = IdRange;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(mut item) => {
                while let Some(next_item) = self.iter.next_if(|&n| n.start <= item.end) {
                    item.end = u64::max(item.end, next_item.end)
                }

                Some(item)
            }
            None => None,
        }
    }
}

pub(crate) trait CombineOverlapsExt: Iterator + Sized {
    fn combine_overlaps(self) -> CombineOverlaps<Self::Item>;
}

impl<I> CombineOverlapsExt for I
where
    I: Iterator<Item = IdRange>,
{
    fn combine_overlaps(self) -> CombineOverlaps<Self::Item> {
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
        Err(anyhow!("start must be less than end"))
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
    fn test_flatten_overlapping_id_ranges() {
        let input = parse_id_ranges("5-6,3-10,20-25,17-22").unwrap();
        assert_eq!(input.len(), 4);

        let flattened = input.into_iter().combine_overlaps().collect::<Vec<_>>();
        let [first, second]: [IdRange; 2] =
            flattened.try_into().expect("Expected exactly 2 IdRanges");

        assert_eq!(first, IdRange { start: 3, end: 10 });
        assert_eq!(second, IdRange { start: 17, end: 25 });
    }
}
