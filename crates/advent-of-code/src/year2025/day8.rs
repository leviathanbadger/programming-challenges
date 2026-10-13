mod circuit;
mod input;
mod point;
mod point_dist;

use anyhow::{Result, anyhow};
use circuit::{Circuit, connect_all_closest_points, connect_closest_points};
use input::get_input;
use point::Point3d;

/// Computes the product of the point count in the top largest_count circuits.
/// Assumes circuits is pre-sorted in descending order by the point count.
fn product_largest_circuits(circuits: &[Circuit], largest_count: usize) -> Result<u32> {
    circuits
        .get(..largest_count)
        .ok_or_else(|| anyhow!("Tried to take the product of top {largest_count} circuit sizes, but only found {} circuits", circuits.len()))?
        .iter()
        .map(|c| c.point_indices.len())
        .try_fold(1u32, |acc, next| acc.checked_mul(next as u32))
        .ok_or_else(|| anyhow!("Overflow calculating product"))
}

/// Computes the product of the x-coordinates of the two points specified in the last connection of the specified circuit.
fn product_last_connection_x_coords(points: &[Point3d], circuit: &Circuit) -> Result<i64> {
    let connection = circuit
        .connections
        .last()
        .ok_or_else(|| anyhow!("The specified circuit has no connections"))?;

    let p1 = points
        .get(connection.0)
        .ok_or_else(|| anyhow!("The connection references an out-of-bounds from point"))?;
    let p2 = points
        .get(connection.1)
        .ok_or_else(|| anyhow!("The connection references an out-of-bounds to point"))?;

    i64::from(p1.x)
        .checked_mul(i64::from(p2.x))
        .ok_or_else(|| anyhow!("Overflow multiplying connection point x-coordinates"))
}

pub(super) fn compute_part1() -> Result<u32> {
    let points = get_input()?;
    let circuits = connect_closest_points(&points, 1000)?;
    product_largest_circuits(&circuits, 3)
}

pub(super) fn compute_part2() -> Result<i64> {
    let points = get_input()?;
    let circuit = connect_all_closest_points(&points)?;
    product_last_connection_x_coords(&points, &circuit)
}

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
    fn test_part1_sample() {
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

    #[test]
    fn test_part2_sample() {
        let points = parse_puzzle_input(SAMPLE_INPUT).unwrap();
        let circuit = connect_all_closest_points(&points).unwrap();
        let product = product_last_connection_x_coords(&points, &circuit).unwrap();
        assert_eq!(product, 25272);
    }
}
