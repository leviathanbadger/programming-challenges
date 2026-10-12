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
    #[allow(dead_code)]
    // I'm anticipating that this will be needed for part 2. I'll consider removing it if it becomes apparent it won't be.
    pub connections: Vec<(usize, usize)>,
}

pub(super) fn connect_closest_points(
    points: &[Point3d],
    connect_count: usize,
) -> Result<Vec<Circuit>> {
    let pairs = get_closest_points(points)?;
    let mut circuit_ids = Vec::from_fn(points.len(), |idx| idx);
    let mut connected_pairs = vec![];
    let mut connected_count = 0;

    for pair in pairs {
        let first_c = circuit_ids[pair.from_idx];
        let second_c = circuit_ids[pair.to_idx];
        if first_c != second_c {
            connected_pairs.push((pair.from_idx, pair.to_idx));
            let [new_c, old_c] = minmax(first_c, second_c);
            for cid in &mut circuit_ids {
                if *cid == old_c {
                    *cid = new_c;
                }
            }
        }

        connected_count += 1;
        if connected_count == connect_count {
            break;
        }
    }

    if connected_count < connect_count {
        return Err(anyhow!(
            "Could not make {connect_count} connections; all points are in a single circuit after only {connected_count}!"
        ));
    }

    // TODO MAYBE: add itertools and use into_grouping_map_by(...).aggregate(...)

    let mut map = HashMap::<usize, Vec<usize>>::new();
    for (idx, _) in points.iter().enumerate() {
        let entry = map.entry(circuit_ids[idx]).or_default();
        entry.push(idx);
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
