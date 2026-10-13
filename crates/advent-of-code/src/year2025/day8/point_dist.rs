use super::point::Point3d;
use anyhow::{Result, anyhow};
use num_traits::pow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PointDist2 {
    pub from_idx: usize,
    pub to_idx: usize,
    pub dist2: u64,
}

/// Note: this method assumes all points have no axes < -1 billion or > 1 billion.
fn point_distance2(from_idx: usize, to_idx: usize, from_pt: Point3d, to_pt: Point3d) -> PointDist2 {
    let diff = to_pt - from_pt;
    PointDist2 {
        from_idx,
        to_idx,
        dist2: pow(u64::from(diff.x.unsigned_abs()), 2)
            + pow(u64::from(diff.y.unsigned_abs()), 2)
            + pow(u64::from(diff.z.unsigned_abs()), 2),
    }
}

/// Returns a vector containing each pair of points, sorted in ascending order
/// based on the linear distance between the points. Note: this method assumes
/// all points have no axes < -1 billion or > 1 billion.
pub(super) fn get_closest_points(points: &[Point3d]) -> Result<Vec<PointDist2>> {
    if points.len() > 5000 {
        return Err(anyhow!(
            "get_closest_points deliberately doesn't support inputs with > 5000 elements to avoid massive allocations."
        ));
    }

    let mut dists = points
        .iter()
        .enumerate()
        .flat_map(|(from_idx, from_pt)| {
            points
                .iter()
                .enumerate()
                .skip(from_idx + 1)
                .map(move |(to_idx, to_pt)| point_distance2(from_idx, to_idx, *from_pt, *to_pt))
        })
        .collect::<Vec<_>>();

    dists.sort_by_key(|dist| dist.dist2);

    Ok(dists)
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use num_traits::Zero;

    use super::super::point::parse_puzzle_input;
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
    fn test_get_closest_points_large_input() {
        let points = [Point3d::zero(); 5001];
        let result = get_closest_points(&points);
        assert_matches!(result, Err(e) if e.to_string().contains("get_closest_points deliberately doesn't support inputs with > 5000 elements to avoid massive allocations"));
    }

    #[test]
    fn test_get_closest_points_sample() {
        let points = parse_puzzle_input(SAMPLE_INPUT).unwrap();
        let closest_pairs = get_closest_points(&points).unwrap();
        assert_eq!(closest_pairs.len(), (points.len() * (points.len() - 1)) / 2);

        assert_eq!(closest_pairs[0].from_idx, 0);
        assert_eq!(closest_pairs[0].to_idx, 19);

        assert_eq!(closest_pairs[1].from_idx, 0);
        assert_eq!(closest_pairs[1].to_idx, 7);

        assert_eq!(closest_pairs[2].from_idx, 2);
        assert_eq!(closest_pairs[2].to_idx, 13);

        assert_eq!(closest_pairs[3].from_idx, 7);
        assert_eq!(closest_pairs[3].to_idx, 19);
    }
}
