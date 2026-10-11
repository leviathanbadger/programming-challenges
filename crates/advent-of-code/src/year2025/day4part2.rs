mod input;
mod types;

use anyhow::Result;
use input::get_input;
use types::{PrintingCell, SizedGrid};

fn is_accessible_paper(grid: &SizedGrid<PrintingCell>, x: i32, y: i32) -> bool {
    grid.get_or_default(x, y, PrintingCell::Empty) == PrintingCell::Paper
        && grid.count_surrounding_matches(
            x,
            y,
            |val| val == PrintingCell::Paper,
            PrintingCell::Empty,
        ) < 4
}

fn count_accessible_paper_rolls_recursive(grid: &mut SizedGrid<PrintingCell>) -> u32 {
    let mut count = 0;
    let mut found_accessible = true;

    while found_accessible {
        let mut accessible_positions = vec![];
        found_accessible = false;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if is_accessible_paper(grid, x as i32, y as i32) {
                    count += 1;
                    found_accessible = true;
                    accessible_positions.push((x as i32, y as i32));
                }
            }
        }
        for (x, y) in accessible_positions {
            *grid.get_mut(x, y).expect(
                "Accessible cell should have been within grid bounds. This should be impossible",
            ) = PrintingCell::Empty;
        }
    }

    count
}

pub fn compute() -> Result<u32> {
    let mut grid = get_input()?;
    Ok(count_accessible_paper_rolls_recursive(&mut grid))
}

#[cfg(test)]
mod tests {
    use super::types::parse_printing_grid;

    use super::*;

    #[test]
    fn test_count_accessible_paper_rolls_recursive_sample() {
        let input_str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let mut grid = parse_printing_grid(input_str).unwrap();
        assert_eq!(count_accessible_paper_rolls_recursive(&mut grid), 43);
    }

    #[test]
    fn test_is_accessible_paper_empty() {
        let input_str = "...\n@.@\n@..";
        let grid = parse_printing_grid(input_str).unwrap();
        assert!(!is_accessible_paper(&grid, 1, 1));
    }

    #[test]
    fn test_is_accessible_paper_paper() {
        {
            let input_str = "...\n@@@\n@..";
            let grid = parse_printing_grid(input_str).unwrap();
            assert!(is_accessible_paper(&grid, 1, 1));
        }

        {
            let input_str = "@@.\n@@@\n...";
            let grid = parse_printing_grid(input_str).unwrap();
            assert!(!is_accessible_paper(&grid, 1, 1));
        }

        {
            let input_str = "@@.\n@@@\n@..";
            let grid = parse_printing_grid(input_str).unwrap();
            assert!(!is_accessible_paper(&grid, 1, 1));
        }
    }

    #[test]
    fn test_is_accessible_paper_paper_boundary() {
        {
            let input_str = "@@\n@@";
            let grid = parse_printing_grid(input_str).unwrap();
            assert!(is_accessible_paper(&grid, 0, 0));
        }

        {
            let input_str = "@@@\n@@.";
            let grid = parse_printing_grid(input_str).unwrap();
            assert!(!is_accessible_paper(&grid, 1, 0));
        }

        {
            let input_str = "@@\n@@";
            let grid = parse_printing_grid(input_str).unwrap();
            assert!(is_accessible_paper(&grid, 1, 1));
        }

        {
            let input_str = "@@@\n@@.";
            let grid = parse_printing_grid(input_str).unwrap();
            assert!(!is_accessible_paper(&grid, 1, 1));
        }
    }

    #[test]
    fn test_is_accessible_paper_paper_outside() {
        let input_str = "@@\n@@";
        let grid = parse_printing_grid(input_str).unwrap();
        assert!(!is_accessible_paper(&grid, -1, 0));
    }
}
