use anyhow::Result;

/// Represents a 2D grid with a fixed size (width and height).
/// All grid cells are nonnegative, but indexing is done with i32 because
/// get and get_mut return an Option, which allows easily sampling edge behavior.
#[derive(Clone)]
pub struct SizedGrid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> SizedGrid<T> {
    /// Create a new SizedGrid with the specified width and height, prefilled with the specified initial value.
    pub fn new(width: usize, height: usize, initial: T) -> Self
    where
        T: Clone,
    {
        SizedGrid {
            width,
            height,
            data: vec![initial; width * height],
        }
    }

    /// Create a new SizedGrid with the specified width and height, prefilled using the specified factory function.
    pub fn try_from_fn<F>(width: usize, height: usize, mut f: F) -> Result<Self>
    where
        F: FnMut(usize, usize) -> Result<T>,
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
