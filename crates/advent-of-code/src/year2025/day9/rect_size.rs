use std::{cmp::minmax, collections::BTreeSet};

use bitflags::bitflags;
use shared_utils::SizedGrid;

use super::point::{Point, point};
use anyhow::{Result, anyhow};

pub(super) fn rect_size(p1: Point, p2: Point) -> u64 {
    let wid = u64::from(p1.x.abs_diff(p2.x)) + 1;
    let hit = u64::from(p1.y.abs_diff(p2.y)) + 1;
    wid * hit
}

/// Returns the largest rectangle (in area) that can be made from a pair of the passed-in points used as
/// opposite corners. No guarantees are made on which pair is returned if multiple pairs have the same area.
pub(super) fn largest_rectangle_from_corner_points(
    points: &[Point],
) -> Option<(Point, Point, u64)> {
    match points {
        [] => None,
        [first] => Some((*first, *first, 1)),
        [first, second] => Some((*first, *second, rect_size(*first, *second))),
        _ => points[..points.len() - 1]
            .iter()
            .enumerate()
            .flat_map(|(first_idx, first)| {
                points[first_idx + 1..]
                    .iter()
                    .map(|second| (*first, *second, rect_size(*first, *second)))
            })
            .max_by_key(|(_, _, area)| *area),
    }
}

bitflags! {
    // TODO: terrible name, but I'm not sure what to call it. Whatever, it's internal code
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct ShapePart: u32 {
        const OUTSIDE    = 0b00000;
        const INSIDE     = 0b00001;
        const EDGE_UP    = 0b00010;
        const EDGE_DOWN  = 0b00100;
        const EDGE_LEFT  = 0b01000;
        const EDGE_RIGHT = 0b10000;
    }
}

fn compute_coordinate_compression_coordinates(points: &[Point]) -> (Vec<i32>, Vec<i32>) {
    let mut x_coords = BTreeSet::new();
    let mut y_coords = BTreeSet::new();

    x_coords.insert(i32::MIN);
    y_coords.insert(i32::MIN);

    for p in points {
        x_coords.insert(p.x);
        if let Some(xp1) = p.x.checked_add(1) {
            x_coords.insert(xp1);
        }
        y_coords.insert(p.y);
        if let Some(yp1) = p.y.checked_add(1) {
            y_coords.insert(yp1);
        }
    }

    (
        x_coords.into_iter().collect(),
        y_coords.into_iter().collect(),
    )
}

fn compress_point(p: Point, x_coords: &[i32], y_coords: &[i32]) -> Result<Point> {
    //Note: propagating error here instead of trying to be over-inclusive, because all points should
    //exactly match some compressed coordinates, and if they don't then something went wrong earlier.
    Ok(point!(
        x_coords.binary_search(&p.x).map_err(|_| anyhow!(
            "Could not compress point - x_coords does not contain exact x coordinate!"
        ))? as i32,
        y_coords.binary_search(&p.y).map_err(|_| anyhow!(
            "Could not compress point - y_coords does not contain exact y coordinate!"
        ))? as i32
    ))
}

fn compute_coordinate_compression_shape(
    points: &[Point],
    x_coords: &[i32],
    y_coords: &[i32],
) -> Result<SizedGrid<ShapePart>> {
    let mut grid = SizedGrid::new(x_coords.len(), y_coords.len(), ShapePart::OUTSIDE);

    let pairwise_compressed = points.iter().enumerate().map(|(curr_idx, &curr)| {
        let next_idx = (curr_idx + 1) % points.len();
        let next = points[next_idx];
        (
            compress_point(curr, x_coords, y_coords),
            compress_point(next, x_coords, y_coords),
        )
    });

    for (curr_result, next_result) in pairwise_compressed {
        let (curr, next) = (curr_result?, next_result?);
        if curr.x == next.x {
            if curr.y == next.y {
                *grid.get_mut(curr.x, curr.y).ok_or_else(|| {
                    anyhow!("Point outside compressed grid space. Should be impossible")
                })? |= ShapePart::INSIDE;
                continue; // No reasonable way to handle edges for duplicate points
            } else if curr.y < next.y {
                // Edge moving down
                for y in curr.y..=next.y {
                    *grid.get_mut(curr.x, y).ok_or_else(|| {
                        anyhow!("Point outside compressed grid space. Should be impossible")
                    })? |= ShapePart::EDGE_RIGHT | ShapePart::INSIDE;
                }
            } else {
                // curr.y > next.y
                // Edge moving up
                for y in next.y..=curr.y {
                    *grid.get_mut(curr.x, y).ok_or_else(|| {
                        anyhow!("Point outside compressed grid space. Should be impossible")
                    })? |= ShapePart::EDGE_LEFT | ShapePart::INSIDE;
                }
            }
        } else {
            // curr.y == next.y
            if curr.x < next.x {
                // Edge moving right
                for x in curr.x..=next.x {
                    *grid.get_mut(x, curr.y).ok_or_else(|| {
                        anyhow!("Point outside compressed grid space. Should be impossible")
                    })? |= ShapePart::EDGE_UP | ShapePart::INSIDE;
                }
            } else {
                // curr.x > next.x
                // Edge moving left
                for x in next.x..=curr.x {
                    *grid.get_mut(x, curr.y).ok_or_else(|| {
                        anyhow!("Point outside compressed grid space. Should be impossible")
                    })? |= ShapePart::EDGE_DOWN | ShapePart::INSIDE;
                }
            }
        }
    }

    for y in 0..grid.height() {
        let mut is_inside = false;
        for x in 0..grid.width() {
            let cell = grid.get_mut(x as i32, y as i32).unwrap();
            if cell.contains(ShapePart::EDGE_LEFT) {
                is_inside = true;
            }
            if cell.contains(ShapePart::EDGE_RIGHT) {
                is_inside = false;
            }

            if is_inside {
                *cell |= ShapePart::INSIDE;
            }
        }
    }

    Ok(grid)
}

fn is_rect_in_bounds_with_compression(
    points: &[Point],
    p1_idx: usize,
    p2_idx: usize,
    x_coords: &[i32],
    y_coords: &[i32],
    grid: &SizedGrid<ShapePart>,
) -> Result<bool> {
    let [minx, maxx] = minmax(points[p1_idx].x, points[p2_idx].x);
    let [miny, maxy] = minmax(points[p1_idx].y, points[p2_idx].y);
    let minp = compress_point(point!(minx, miny), x_coords, y_coords)?;
    let maxp = compress_point(point!(maxx, maxy), x_coords, y_coords)?;

    for x in minp.x..=maxp.x {
        for y in minp.y..=maxp.y {
            if !grid
                .get_or_default(x, y, ShapePart::OUTSIDE)
                .contains(ShapePart::INSIDE)
            {
                return Ok(false);
            }
        }
    }

    Ok(true)
}

/// Assumes the passed-in points describe a clockwise self-closing, non-intersecting polygon, where each point pair moves in only one axis.
/// Returns the largest rectangle (in area) that can be made from a pair of the points used as
/// opposite corners without the described rectangle leaving the bounds of the polygon.
pub(super) fn largest_rectangle_from_bounding_corner_points(
    points: &[Point],
) -> Result<Option<(Point, Point, u64)>> {
    if points.len() <= 4 {
        return Ok(largest_rectangle_from_corner_points(points));
    }

    let (x_coords, y_coords) = compute_coordinate_compression_coordinates(points);
    let inside_grid_shape = compute_coordinate_compression_shape(points, &x_coords, &y_coords)?;

    let is_rect_in_bounds = |points: &[Point], p1_idx: usize, p2_idx: usize| {
        is_rect_in_bounds_with_compression(
            points,
            p1_idx,
            p2_idx,
            &x_coords,
            &y_coords,
            &inside_grid_shape,
        )
    };

    points[..points.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(first_idx, first)| {
            points
                .iter()
                .enumerate()
                .skip(first_idx + 1)
                .map(move |(second_idx, second)| {
                    (first_idx, second_idx, rect_size(*first, *second))
                })
        })
        .try_fold(None, |acc, (p1_idx, p2_idx, area)| {
            if let Some((_, _, acc_area)) = acc
                && acc_area >= area
            {
                return Ok(acc);
            }

            if !is_rect_in_bounds(points, p1_idx, p2_idx)? {
                return Ok(acc);
            }

            Ok(Some((points[p1_idx], points[p2_idx], area)))
        })
}

#[cfg(test)]
mod tests {
    use super::super::point::point;
    use super::*;

    #[test]
    fn test_rect_size() {
        assert_eq!(rect_size(point!(2, 5), point!(9, 7)), 24);
        assert_eq!(rect_size(point!(7, 1), point!(11, 7)), 35);
        assert_eq!(rect_size(point!(7, 3), point!(2, 3)), 6);
        assert_eq!(rect_size(point!(2, 5), point!(11, 1)), 50);
    }

    #[test]
    fn test_largest_rectangle_from_corner_points() {
        //Testing only edge cases since this method is tested with more complex sample input in day9.rs

        {
            let points: [Point; 0] = [];
            assert!(largest_rectangle_from_corner_points(&points).is_none());
        }

        {
            let points: [Point; _] = [point!(2, 2)];
            let (p1, p2, area) = largest_rectangle_from_corner_points(&points).unwrap();
            assert_eq!(p1, points[0]);
            assert_eq!(p2, points[0]);
            assert_eq!(area, 1);
        }

        {
            let points: [Point; _] = [point!(2, 2), point!(4, 5)];
            let (p1, p2, area) = largest_rectangle_from_corner_points(&points).unwrap();
            assert_eq!(p1, points[0]);
            assert_eq!(p2, points[1]);
            assert_eq!(area, 12);
        }
    }

    macro_rules! make_is_rect_in_bounds {
        ( $points:expr ) => {{
            let (x_coords, y_coords) = compute_coordinate_compression_coordinates($points);
            let inside_grid_shape =
                compute_coordinate_compression_shape($points, &x_coords, &y_coords).unwrap();

            let is_rect_in_bounds = move |points: &[Point], p1_idx: usize, p2_idx: usize| {
                is_rect_in_bounds_with_compression(
                    points,
                    p1_idx,
                    p2_idx,
                    &x_coords,
                    &y_coords,
                    &inside_grid_shape,
                )
            };
            is_rect_in_bounds
        }};
    }

    #[test]
    fn test_is_rect_in_bounds_simple() {
        {
            let points: [Point; _] = [point!(2, 2)];
            let is_rect_in_bounds = make_is_rect_in_bounds!(&points);
            assert!(is_rect_in_bounds(&points, 0, 0).unwrap());
        }

        {
            let points: [Point; _] = [point!(2, 2), point!(4, 2)];
            let is_rect_in_bounds = make_is_rect_in_bounds!(&points);
            assert!(is_rect_in_bounds(&points, 0, 1).unwrap());
            assert!(is_rect_in_bounds(&points, 1, 0).unwrap());
        }
    }

    #[test]
    fn test_is_rect_in_bounds_colinear_line() {
        let points: [Point; _] = [
            point!(2, 2),
            point!(3, 2),
            point!(4, 2),
            point!(5, 2),
            point!(6, 2),
        ];

        let is_rect_in_bounds = make_is_rect_in_bounds!(&points);

        assert!(is_rect_in_bounds(&points, 0, 4).unwrap());
        assert!(is_rect_in_bounds(&points, 4, 0).unwrap());

        assert!(is_rect_in_bounds(&points, 1, 3).unwrap());
        assert!(is_rect_in_bounds(&points, 3, 1).unwrap());
    }

    #[test]
    fn test_is_rect_in_bounds_colinear_rectangle() {
        let points: [Point; _] = [
            point!(2, 2),
            point!(4, 2),
            point!(4, 4),
            point!(3, 4),
            point!(2, 4),
        ];

        let is_rect_in_bounds = make_is_rect_in_bounds!(&points);

        assert!(is_rect_in_bounds(&points, 0, 2).unwrap());
        assert!(is_rect_in_bounds(&points, 2, 0).unwrap());

        assert!(is_rect_in_bounds(&points, 2, 4).unwrap());
        assert!(is_rect_in_bounds(&points, 4, 2).unwrap());
    }

    #[test]
    fn test_is_rect_in_bounds_convex_edge_pairs() {
        let points: [Point; _] = [
            point!(2, 2),
            point!(6, 2),
            point!(6, 4),
            point!(4, 4),
            point!(4, 6),
            point!(2, 6),
        ];

        let is_rect_in_bounds = make_is_rect_in_bounds!(&points);

        let edge_pairs = (0..points.len()).map(|p1_idx| (p1_idx, (p1_idx + 1) % points.len()));
        for (p1, p2) in edge_pairs {
            assert!(is_rect_in_bounds(&points, p1, p2).unwrap());
            assert!(is_rect_in_bounds(&points, p2, p1).unwrap());
        }

        assert!(!is_rect_in_bounds(&points, 2, 4).unwrap());
        assert!(!is_rect_in_bounds(&points, 4, 2).unwrap());
    }

    #[test]
    fn test_is_rect_in_bounds_convex_with_rotation() {
        let mut points: [Point; _] = [
            point!(1, 1),
            point!(2, 1),
            point!(2, 9),
            point!(9, 9),
            point!(9, 1),
            point!(10, 1),
            point!(10, 10),
            point!(1, 10),
        ];
        let mut idx1 = 1;
        let mut idx2 = 3;
        let mut idx3 = 7;
        let mut idx4 = 0;
        let mut idx5 = 5;

        for _ in 0..points.len() {
            let is_rect_in_bounds = make_is_rect_in_bounds!(&points);

            assert!(!is_rect_in_bounds(&points, idx1, idx2).unwrap());
            assert!(!is_rect_in_bounds(&points, idx2, idx1).unwrap());

            assert!(is_rect_in_bounds(&points, idx1, idx3).unwrap());
            assert!(is_rect_in_bounds(&points, idx3, idx1).unwrap());

            assert!(!is_rect_in_bounds(&points, idx4, idx5).unwrap());
            assert!(!is_rect_in_bounds(&points, idx5, idx4).unwrap());

            points.rotate_right(1);
            idx1 = (idx1 + 1) % points.len();
            idx2 = (idx2 + 1) % points.len();
            idx3 = (idx3 + 1) % points.len();
            idx4 = (idx4 + 1) % points.len();
            idx5 = (idx5 + 1) % points.len();
        }
    }

    #[test]
    fn test_is_rect_in_bounds_adjacent_boundary() {
        let points: [Point; _] = [
            point!(0, 0),
            point!(1, 0),
            point!(1, 3),
            point!(2, 3),
            point!(2, 0),
            point!(3, 0),
            point!(3, 4),
            point!(0, 4),
        ];
        let is_rect_in_bounds = make_is_rect_in_bounds!(&points);
        assert!(is_rect_in_bounds(&points, 0, 6).unwrap());
        assert!(is_rect_in_bounds(&points, 6, 0).unwrap());
    }

    #[test]
    fn test_is_rect_in_bounds_one_tile_gap() {
        let points: [Point; _] = [
            point!(0, 0),
            point!(1, 0),
            point!(1, 3),
            point!(3, 3),
            point!(3, 0),
            point!(4, 0),
            point!(4, 4),
            point!(0, 4),
        ];
        let is_rect_in_bounds = make_is_rect_in_bounds!(&points);
        assert!(!is_rect_in_bounds(&points, 0, 6).unwrap());
        assert!(!is_rect_in_bounds(&points, 6, 0).unwrap());
    }
}
