mod circuit;
mod input;
mod point;
mod point_dist;

use anyhow::{Result, anyhow};
use circuit::{Circuit, connect_closest_points};
use input::get_input;

fn product_largest_circuits(circuits: &[Circuit], largest_count: usize) -> Result<u32> {
    let counts = circuits
        .iter()
        .map(|c| c.point_indices.len())
        .take(largest_count)
        .collect::<Vec<_>>();

    let counts_len = counts.len();
    if counts_len != largest_count {
        return Err(anyhow!(
            "Tried to take the product of top {largest_count} circuit sizes, but only found {counts_len} circuits"
        ));
    }

    counts
        .iter()
        .try_fold(1u32, |acc, next| acc.checked_mul(*next as u32))
        .ok_or_else(|| anyhow!("Overflow calculating product"))
}

pub(super) fn compute_part1() -> Result<u32> {
    let points = get_input()?;
    let circuits = connect_closest_points(&points, 1000)?;
    product_largest_circuits(&circuits, 3)
}

// pub(super) fn compute_part2() -> Result<u32> {
//     let points = get_input()?;
//     todo!();
// }

#[cfg(test)]
mod tests {
    use super::point::parse_puzzle_input;

    use super::*;

    const SAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_sample() {
        let points = parse_puzzle_input(SAMPLE_INPUT).unwrap();
        let circuits = connect_closest_points(&points, 10).unwrap();

        let circuit_sizes = circuits
            .iter()
            .map(|ct| ct.point_indices.len())
            .collect::<Vec<_>>();
        assert_eq!(&circuit_sizes, &[5, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1]);

        let product = product_largest_circuits(&circuits, 3).unwrap();
        assert_eq!(product, 40);
    }
}
