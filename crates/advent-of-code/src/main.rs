#![feature(cmp_minmax)]

mod year2025;

use anyhow::Result;

use year2025::year2025;

fn main() -> Result<()> {
    year2025()?;

    Ok(())
}
