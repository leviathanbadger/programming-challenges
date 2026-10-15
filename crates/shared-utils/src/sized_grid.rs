/// Represents a 2D grid with a fixed size (width and height).
/// All grid cells are nonnegative, but indexing is done with i32 because
/// get and get_mut return an Option, which allows easily sampling edge behavior.
#[derive(Clone)]
pub struct SizedGrid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

#[derive(Debug)]
pub enum GridBuildError<E> {
    SizeOverflow,
    UnsupportedDimensions { width: usize, height: usize },
    Factory(E),
}

impl<T> SizedGrid<T> {
    /// Create a new SizedGrid with the specified width and height, prefilled with the specified initial value.
    /// Width and height must be < i32::MAX - 1, and width * height cannot overflow usize; otherwise panics.
    pub fn new(width: usize, height: usize, initial: T) -> Self
    where
        T: Clone,
    {
        let size = width.checked_mul(height).unwrap_or_else(|| {
            panic!("Can't create grid with size {width}, {height} - size would overflow!")
        });

        if width >= (i32::MAX - 1) as usize || height >= (i32::MAX - 1) as usize {
            panic!("Can't create a grid with any dimension >= {}", i32::MAX - 1);
        }

        SizedGrid {
            width,
            height,
            data: vec![initial; size],
        }
    }

    /// Create a new SizedGrid with the specified width and height, prefilled using the specified factory function.
    /// Width and height must be < i32::MAX - 1, and width * height cannot overflow usize; otherwise panics.
    pub fn from_fn<F>(width: usize, height: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let size = width.checked_mul(height).unwrap_or_else(|| {
            panic!("Can't create grid with size {width}, {height} - size would overflow!")
        });

        if width >= (i32::MAX - 1) as usize || height >= (i32::MAX - 1) as usize {
            panic!("Can't create a grid with any dimension >= {}", i32::MAX - 1);
        }

        let mut data = Vec::with_capacity(size);
        for y in 0..height {
            for x in 0..width {
                data.push(f(x, y));
            }
        }

        SizedGrid {
            width,
            height,
            data,
        }
    }

    /// Create a new SizedGrid with the specified width and height, prefilled using the specified factory function.
    /// Width and height must be < i32::MAX - 1, and width * height cannot overflow usize; otherwise panics.
    pub fn try_from_fn<F, E>(
        width: usize,
        height: usize,
        mut f: F,
    ) -> Result<Self, GridBuildError<E>>
    where
        F: FnMut(usize, usize) -> Result<T, E>,
    {
        let size = width
            .checked_mul(height)
            .ok_or(GridBuildError::SizeOverflow)?;

        if width >= (i32::MAX - 1) as usize || height >= (i32::MAX - 1) as usize {
            return Err(GridBuildError::UnsupportedDimensions { width, height });
        }

        let mut data = Vec::with_capacity(size);
        for y in 0..height {
            for x in 0..width {
                data.push(f(x, y).map_err(|e| GridBuildError::Factory(e))?);
            }
        }

        Ok(SizedGrid {
            width,
            height,
            data,
        })
    }

    /// The grid's width. Readonly after creation.
    pub fn width(&self) -> usize {
        self.width
    }

    /// The grid's height. Readonly after creation.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a reference to the value currently contained in the specified cell,
    /// or None if the index is out of bounds.
    pub fn get(&self, col: i32, row: i32) -> Option<&T> {
        if col < 0 || row < 0 || col as usize >= self.width || row as usize >= self.height {
            None
        } else {
            Some(&self.data[(row as usize) * self.width + (col as usize)])
        }
    }

    /// Returns a the value currently contained in the specified cell,
    /// or the specified default if the index is out of bounds.
    pub fn get_or_default(&self, col: i32, row: i32, default: T) -> T
    where
        T: Copy,
    {
        *self.get(col, row).unwrap_or(&default)
    }

    /// Returns a mutable reference to the value currently contained in the specified cell,
    /// or None if the index is out of bounds.
    pub fn get_mut(&mut self, col: i32, row: i32) -> Option<&mut T> {
        if col < 0 || row < 0 || col as usize >= self.width || row as usize >= self.height {
            None
        } else {
            Some(&mut self.data[(row as usize) * self.width + (col as usize)])
        }
    }

    /// Returns a count of the 8 surrounding cells that match a predicate, using
    /// the specified edge_value when sampling indices out-of-bounds.
    pub fn count_surrounding_matches<F>(&self, x: i32, y: i32, mut f: F, edge_value: T) -> u32
    where
        F: FnMut(T) -> bool,
        T: Copy,
    {
        let mut one_if_true = |val: T| if f(val) { 1 } else { 0 };

        if x <= -2 || y <= -2 || x >= (self.width + 1) as i32 || y >= (self.width + 1) as i32 {
            return one_if_true(edge_value) * 8;
        }

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

impl<T> std::fmt::Debug for SizedGrid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SizedGrid")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

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

        assert_matches!(grid.get(i32::MIN, 0), None);
        assert_matches!(grid.get(0, i32::MIN), None);

        assert_eq!(grid.get_or_default(3, 3, -1), 0);
        assert_eq!(grid.get_or_default(7, 1, -1), 0);
        assert_eq!(grid.get_or_default(9, 0, -1), 0);
        assert_eq!(grid.get_or_default(0, 9, -1), 0);

        assert_eq!(grid.get_or_default(-1, 0, -1), -1);
        assert_eq!(grid.get_or_default(0, -1, -1), -1);
        assert_eq!(grid.get_or_default(10, 0, -1), -1);
        assert_eq!(grid.get_or_default(0, 10, -1), -1);

        assert_matches!(grid.get_or_default(i32::MIN, 0, -1), -1);
        assert_matches!(grid.get_or_default(0, i32::MIN, -1), -1);

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

        assert_eq!(grid.count_surrounding_matches(2, 1, |v| v % 2 == 0, 0), 6);

        assert_eq!(
            grid.count_surrounding_matches(i32::MIN, 0, |v| v % 2 == 0, 0),
            8
        );
        assert_eq!(
            grid.count_surrounding_matches(i32::MAX, 0, |v| v % 2 == 0, 0),
            8
        );
        assert_eq!(
            grid.count_surrounding_matches(0, i32::MIN, |v| v % 2 == 0, 0),
            8
        );
        assert_eq!(
            grid.count_surrounding_matches(0, i32::MAX, |v| v % 2 == 0, 0),
            8
        );
    }

    #[test]
    #[should_panic(expected = "size would overflow!")]
    fn test_sized_grid_new_overflow() {
        let _grid = SizedGrid::new(1usize << (usize::BITS - 1), 2, 0u8);
    }

    #[test]
    #[should_panic(expected = "size would overflow!")]
    fn test_sized_grid_from_fn_overflow() {
        let _grid = SizedGrid::from_fn(1usize << (usize::BITS - 1), 2, |_x, _y| 0u8);
    }

    #[test]
    fn test_sized_grid_try_from_fn_overflow() {
        let grid_result =
            SizedGrid::try_from_fn(1usize << (usize::BITS - 1), 2, |_x, _y| Ok::<_, !>(0u8));
        assert_matches!(grid_result, Err(GridBuildError::SizeOverflow));
    }

    #[test]
    #[should_panic(expected = "dimension >=")]
    fn test_sized_grid_new_overflow_x() {
        let _grid = SizedGrid::new((i32::MAX - 1) as usize, 1, 0u8);
    }

    #[test]
    #[should_panic(expected = "dimension >=")]
    fn test_sized_grid_new_overflow_y() {
        let _grid = SizedGrid::new(1, (i32::MAX - 1) as usize, 0u8);
    }

    #[test]
    #[should_panic(expected = "dimension >=")]
    fn test_sized_grid_from_fn_overflow_x() {
        let _grid = SizedGrid::from_fn((i32::MAX - 1) as usize, 1, |_x, _y| 0u8);
    }

    #[test]
    #[should_panic(expected = "dimension >=")]
    fn test_sized_grid_from_fn_overflow_y() {
        let _grid = SizedGrid::from_fn(1, (i32::MAX - 1) as usize, |_x, _y| 0u8);
    }

    #[test]
    fn test_sized_grid_try_from_fn_overflow_x() {
        let grid_result =
            SizedGrid::try_from_fn((i32::MAX - 1) as usize, 1, |_x, _y| Ok::<_, !>(0u8));
        assert_matches!(
            grid_result,
            Err(GridBuildError::UnsupportedDimensions { .. })
        );
    }

    #[test]
    fn test_sized_grid_try_from_fn_overflow_y() {
        let grid_result =
            SizedGrid::try_from_fn(1, (i32::MAX - 1) as usize, |_x, _y| Ok::<_, !>(0u8));
        assert_matches!(
            grid_result,
            Err(GridBuildError::UnsupportedDimensions { .. })
        );
    }
}
