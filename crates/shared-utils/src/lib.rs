#![warn(missing_docs)]

//! Contains useful traits and utility functions for use in the advent-of-code crate.
//! Yes, this could have been part of the same module, but it's separate specifically
//! to help me learn how to combine code from multiple crates. Together we can rule
//! the galaxy, as library and binary!

mod checked_sum;

pub use checked_sum::CheckedSum;
