use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum TachyonManifoldPart {
    Empty,
    Start,
    Splitter,
}

impl TryFrom<u8> for TachyonManifoldPart {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(TachyonManifoldPart::Empty),
            b'S' => Ok(TachyonManifoldPart::Start),
            b'^' => Ok(TachyonManifoldPart::Splitter),
            _ => Err(anyhow!("Can't parse TachyonManifoldPart from u8: {value}")),
        }
    }
}

pub(super) fn parse_puzzle_input(text: &str) -> Result<Vec<Vec<TachyonManifoldPart>>> {
    let mut puzzle_input = text
        .lines()
        .map(|line| {
            line.bytes()
                .map(TachyonManifoldPart::try_from)
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    if puzzle_input
        .iter()
        .any(|line| line.len() != puzzle_input[0].len())
    {
        return Err(anyhow!(
            "All puzzle input lines must be the same length, and >= 3 characters wide."
        ));
    }

    if puzzle_input.is_empty() || puzzle_input[0].len() < 3 {
        return Err(anyhow!(
            "There must be at least one puzzle input line, and the puzzle must be >= 3 characters wide."
        ));
    }

    if puzzle_input.iter().any(|line| {
        line[0] == TachyonManifoldPart::Splitter
            || line[line.len() - 1] == TachyonManifoldPart::Splitter
    }) {
        return Err(anyhow!(
            "Puzzle input lines cannot start or end with a splitter."
        ));
    }

    puzzle_input.retain(|line| line.iter().any(|val| *val != TachyonManifoldPart::Empty));

    Ok(puzzle_input)
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_parse_puzzle_input_invalid_characters() {
        let result = parse_puzzle_input(
            "..S..
.....
..^..
...z.
.^.^.
.....",
        );
        assert_matches!(result, Err(err) if err.to_string().contains("Can't parse"));
    }

    #[test]
    fn test_parse_puzzle_input_mismatched_line_length() {
        {
            let result = parse_puzzle_input(
                "..S..
.....
..^..
....
.^.^.
.....",
            );
            assert_matches!(result, Err(err) if err.to_string().contains("All puzzle input lines must be the same length"));
        }

        {
            let result = parse_puzzle_input(
                "..S..
.....
..^..
......
.^.^.
.....",
            );
            assert_matches!(result, Err(err) if err.to_string().contains("All puzzle input lines must be the same length"));
        }
    }

    #[test]
    fn test_parse_puzzle_input_too_small() {
        {
            let result = parse_puzzle_input("..");
            assert_matches!(result, Err(err) if err.to_string().contains("must be >= 3 characters wide"));
        }

        {
            let result = parse_puzzle_input("\n");
            assert_matches!(result, Err(err) if err.to_string().contains("must be >= 3 characters wide"));
        }

        {
            let result = parse_puzzle_input("");
            assert_matches!(result, Err(err) if err.to_string().contains("must be at least one puzzle input line"));
        }
    }

    #[test]
    fn test_parse_puzzle_input_start_end_splitter() {
        {
            let result = parse_puzzle_input(
                "..S..
.....
..^..
.....
.^.^.
....^",
            );
            assert_matches!(result, Err(err) if err.to_string().contains("Puzzle input lines cannot start or end with a splitter"));
        }

        {
            let result = parse_puzzle_input(
                "..S..
.....
..^..
^....
.^.^.
.....",
            );
            assert_matches!(result, Err(err) if err.to_string().contains("Puzzle input lines cannot start or end with a splitter"));
        }
    }

    #[test]
    fn test_parse_puzzle_input_valid() {
        let puzzle_input = parse_puzzle_input(
            "..S..
.....
..^..
.....
.^.^.
.....",
        )
        .unwrap();

        // Assert empty lines are removed
        assert_eq!(puzzle_input.len(), 3);
        assert_eq!(puzzle_input[0].len(), 5);
        assert_eq!(puzzle_input[1].len(), 5);
        assert_eq!(puzzle_input[2].len(), 5);
    }
}
