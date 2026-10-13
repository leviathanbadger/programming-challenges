use std::{
    cmp::{Reverse, minmax},
    collections::HashMap,
};

use super::point::Point3d;
use super::point_dist::get_closest_points;
use anyhow::{Result, anyhow};

#[derive(Debug)]
pub(super) struct Circuit {
    pub point_indices: Vec<usize>,
    pub connections: Vec<(usize, usize)>,
}

/// Computes the connected circuits in the specified points slice after attempting connect_count connections
/// If a pair would join two points in an already-connected circuit, the pair is ignored and no connection is created.
/// On success, returns a Vec<Circuit> sorted by the number of points in each circuit in descending order.
/// The circuit's "connections" will be sorted by the order in which connections are added to each circuit.
pub(super) fn connect_closest_points(
    points: &[Point3d],
    connect_count: usize,
) -> Result<Vec<Circuit>> {
    let pairs = get_closest_points(points)?;
    let mut circuit_ids = Vec::from_fn(points.len(), |idx| idx);
    let mut connected_pairs = vec![];

    if connect_count > pairs.len() {
        return Err(anyhow!(
            "Requested {connect_count} connection attempts, but only {} exist",
            pairs.len()
        ));
    }

    for pair in pairs.into_iter().take(connect_count) {
        let first_c = circuit_ids[pair.from_idx];
        let second_c = circuit_ids[pair.to_idx];
        if first_c == second_c {
            continue;
        }

        connected_pairs.push((pair.from_idx, pair.to_idx));
        let [new_c, old_c] = minmax(first_c, second_c);
        for cid in &mut circuit_ids {
            if *cid == old_c {
                *cid = new_c;
            }
        }

        //Stop early if all points have joined the same circuit. There will be no further connections.
        if connected_pairs.len() == points.len() - 1 {
            break;
        }
    }

    let mut map = HashMap::<usize, Vec<usize>>::new();
    for (idx, &circuit_id) in circuit_ids.iter().enumerate() {
        map.entry(circuit_id).or_default().push(idx);
    }

    let mut results = map
        .into_values()
        .map(|point_indices| {
            let connections = connected_pairs
                .iter()
                .cloned()
                .filter(|(lhs, _)| point_indices.contains(lhs))
                .collect();
            Circuit {
                point_indices,
                connections,
            }
        })
        .collect::<Vec<_>>();

    results.sort_by_key(|r| Reverse(r.point_indices.len()));

    Ok(results)
}

/// Computes the connected circuit in the specified points slice after attempting all connections in ascending order of point distance.
/// If a pair would join two points in an already-connected circuit, the pair is ignored and no connection is created.
/// On success, returns the resultant Circuit. The circuit's "connections" will be sorted by the order in which connections were added.
pub(super) fn connect_all_closest_points(points: &[Point3d]) -> Result<Circuit> {
    if points.is_empty() {
        return Err(anyhow!(
            "Can't connect all points - no cluster can be created with no points!"
        ));
    }

    let mut circuits = connect_closest_points(points, (points.len() * (points.len() - 1)) / 2)?;
    Ok(circuits.remove(0))
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::super::point::parse_puzzle_input;
    use super::*;

    #[test]
    fn test_connect_closest_points_connect_count_0() {
        let points = parse_puzzle_input(
            "1,2,3
4,5,6
7,8,9",
        )
        .unwrap();
        let circuits = connect_closest_points(&points, 0).unwrap();
        assert_eq!(circuits.len(), 3);
        assert_eq!(circuits[0].point_indices.len(), 1);
        assert_eq!(circuits[1].point_indices.len(), 1);
        assert_eq!(circuits[2].point_indices.len(), 1);
    }

    #[test]
    fn test_connect_all_closest_points_point_count_0() {
        let points: [Point3d; 0] = [];
        let result = connect_all_closest_points(&points);
        assert_matches!(result, Err(e) if e.to_string().contains("no cluster can be created with no points"));
    }
}
