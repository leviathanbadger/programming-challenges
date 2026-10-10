use anyhow::{Result, anyhow};

#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum PrintingCell {
    Empty,
    Paper,
}

impl TryFrom<u8> for PrintingCell {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            b'@' => Ok(PrintingCell::Paper),
            b'.' => Ok(PrintingCell::Empty),
            _ => Err(anyhow!("Can't convert u8 {value} to PrintingCell")),
        }
    }
}

pub(super) struct SizedGrid<T: Sized + Copy + Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Sized + Copy + Clone> SizedGrid<T> {
    #[allow(dead_code)] // This is the intended way to create new grids, if this were moved to a library and reused
    pub fn new(width: usize, height: usize, initial: T) -> Self {
        SizedGrid {
            width,
            height,
            data: vec![initial; width * height],
        }
    }

    pub fn try_from_fn<F>(width: usize, height: usize, f: F) -> Result<Self>
    where
        F: Fn(usize, usize) -> Result<T>,
    {
        let mut data = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                data.push(f(x, y)?);
            }
        }

        Ok(SizedGrid {
            width,
            height,
            data,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, col: i32, row: i32) -> Option<&T> {
        if col < 0 || row < 0 || col as usize >= self.width || row as usize >= self.height {
            None
        } else {
            Some(&self.data[(row as usize) * self.width + (col as usize)])
        }
    }

    pub fn get_or_default(&self, col: i32, row: i32, default: T) -> T {
        *self.get(col, row).unwrap_or(&default)
    }

    #[allow(dead_code)] // This is the only way to modify grids, if this were moved to a library and reused
    pub fn get_mut(&mut self, col: i32, row: i32) -> Option<&mut T> {
        if col < 0 || row < 0 || col as usize >= self.width || row as usize >= self.height {
            None
        } else {
            Some(&mut self.data[(row as usize) * self.width + (col as usize)])
        }
    }

    pub fn count_surrounding_matches<F>(&self, x: i32, y: i32, f: F, edge_value: T) -> u32
    where
        F: Fn(T) -> bool,
    {
        let one_if_true = |val: T| if f(val) { 1 } else { 0 };
        one_if_true(self.get_or_default(x - 1, y - 1, edge_value))
            + one_if_true(self.get_or_default(x, y - 1, edge_value))
            + one_if_true(self.get_or_default(x + 1, y - 1, edge_value))
            + one_if_true(self.get_or_default(x - 1, y, edge_value))
            + one_if_true(self.get_or_default(x + 1, y, edge_value))
            + one_if_true(self.get_or_default(x - 1, y + 1, edge_value))
            + one_if_true(self.get_or_default(x, y + 1, edge_value))
            + one_if_true(self.get_or_default(x + 1, y + 1, edge_value))
    }
}

impl<T> std::fmt::Debug for SizedGrid<T>
where
    T: Sized + Copy + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SizedGrid")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

pub(super) fn parse_printing_grid(input_str: &str) -> Result<SizedGrid<PrintingCell>> {
    let lines = input_str.split_whitespace().collect::<Vec<_>>();
    let height = lines.len();
    let width = lines[0].len();

    if lines.iter().any(|line| line.len() != width) {
        return Err(anyhow!("One or more lines are not the correct length"));
    }

    SizedGrid::try_from_fn(width, height, |col, row| {
        PrintingCell::try_from(lines[row].as_bytes()[col])
    })
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_try_from_error() {
        let result = PrintingCell::try_from(b'z');
        assert_matches!(result, Err(err) if err.to_string().contains("Can't convert"));
    }

    #[test]
    fn test_parse_printing_grid_invalid_widths() {
        let result = parse_printing_grid("...\n.@.\n.@@@");
        assert_matches!(result, Err(err) if err.to_string().contains("not the correct length"));
    }

    #[test]
    fn test_parse_printing_grid_parse_error() {
        let result = parse_printing_grid("...\n.@.\n.@q");
        assert_matches!(result, Err(err) if err.to_string().contains("Can't convert"));
    }

    #[test]
    fn test_sized_grid_get_set() {
        let mut grid = SizedGrid::new(10, 10, 0);

        assert_matches!(grid.get(3, 3), Some(0));
        assert_matches!(grid.get(7, 1), Some(0));
        assert_matches!(grid.get(9, 0), Some(0));
        assert_matches!(grid.get(0, 9), Some(0));

        assert_matches!(grid.get(-1, 0), None);
        assert_matches!(grid.get(0, -1), None);
        assert_matches!(grid.get(10, 0), None);
        assert_matches!(grid.get(0, 10), None);

        assert_eq!(grid.get_or_default(3, 3, -1), 0);
        assert_eq!(grid.get_or_default(7, 1, -1), 0);
        assert_eq!(grid.get_or_default(9, 0, -1), 0);
        assert_eq!(grid.get_or_default(0, 9, -1), 0);

        assert_eq!(grid.get_or_default(-1, 0, -1), -1);
        assert_eq!(grid.get_or_default(0, -1, -1), -1);
        assert_eq!(grid.get_or_default(10, 0, -1), -1);
        assert_eq!(grid.get_or_default(0, 10, -1), -1);

        //TODO: even aside from the .unwrap(), this doesn't seem like a very readable pattern. Is there a more idiomatic way to do this?
        *grid.get_mut(2, 2).unwrap() = 25;
        assert_matches!(grid.get(2, 2), Some(25));
        assert_matches!(grid.get(3, 3), Some(0));

        for y in 0..10 {
            for x in 0..10 {
                *grid.get_mut(x, y).unwrap() = x * y;
            }
        }

        assert_matches!(grid.get(2, 2), Some(4));
        assert_matches!(grid.get(8, 3), Some(24));
        assert_matches!(grid.get(1, 6), Some(6));
        assert_matches!(grid.get(7, 7), Some(49));

        let count = grid.count_surrounding_matches(2, 1, |v| v % 2 == 0, 0);
        assert_eq!(count, 6);
    }
}
