use std::cmp::minmax;

use crate::year2025::day9::compute_winding::Winding;

use super::{compute_winding::compute_winding_discontiguous, point::Point};

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

/// Assumes the passed-in points describe a clockwise self-closing, non-intersecting polygon, where each point pair moves in only one axis.
/// Returns whether or not the rectangle made from a pair of the points (p1_idx, p2_idx) used as opposite corners is enclosed in the bounds of the polygon.
fn is_rect_is_in_bounds(points: &[Point], mut p1_idx: usize, mut p2_idx: usize) -> bool {
    [p1_idx, p2_idx] = minmax(p1_idx, p2_idx); // In case p2_idx < p1_idx
    let [minx, maxx] = minmax(points[p1_idx].x, points[p2_idx].x);
    let [miny, maxy] = minmax(points[p1_idx].y, points[p2_idx].y);

    // If any edge crosses into or out of the described rectangle, then the rectangle is out of bounds
    let has_crossing = (0..points.len())
        .filter(|idx| {
            *idx != p1_idx
                && *idx != p2_idx
                && (*idx + 1) % points.len() != p1_idx
                && (*idx + 1) % points.len() != p2_idx
        })
        .map(|idx| (points[idx], points[(idx + 1) % points.len()]))
        .any(|(first, second)| {
            (first.x == second.x
                && first.x > minx
                && first.x < maxx
                && ((first.y <= miny) != (second.y <= miny)
                    || (first.y >= maxy) != (second.y >= maxy)))
                || (first.y == second.y
                    && first.y > miny
                    && first.y < maxy
                    && ((first.x <= minx) != (second.x <= minx)
                        || (first.x >= maxx) != (second.x >= maxx)))
        });
    if has_crossing {
        return false;
    }

    let winding =
        compute_winding_discontiguous(points, p1_idx, p2_idx).unwrap_or(Winding::Indeterminate);
    // Note: Winding::Indeterminate could imply a line, or it could imply an overflow error or index out of bounds error (see above)
    // TODO MAYBE: distinguish between the two? Probably not, if an error's occurred then the input data is wildly out of the supported problem domain
    winding != Winding::Counterclockwise
}

/// Assumes the passed-in points describe a clockwise self-closing, non-intersecting polygon, where each point pair moves in only one axis.
/// Returns the largest rectangle (in area) that can be made from a pair of the points used as
/// opposite corners without the described rectangle leaving the bounds of the polygon.
pub(super) fn largest_rectangle_from_bounding_corner_points(
    points: &[Point],
) -> Option<(Point, Point, u64)> {
    if points.len() <= 4 {
        return largest_rectangle_from_corner_points(points);
    }

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
        .fold(None, |acc, (p1_idx, p2_idx, area)| {
            if let Some((_, _, acc_area)) = acc
                && acc_area >= area
            {
                return acc;
            }

            if !is_rect_is_in_bounds(points, p1_idx, p2_idx) {
                return acc;
            }

            Some((points[p1_idx], points[p2_idx], area))
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

    #[test]
    fn test_is_rect_is_in_bounds() {
        //Testing only edge cases since this method is tested with more complex sample input in day9.rs

        {
            let points: [Point; _] = [point!(2, 2)];
            assert!(is_rect_is_in_bounds(&points, 0, 0));
        }

        {
            let points: [Point; _] = [point!(2, 2), point!(4, 2)];
            assert!(is_rect_is_in_bounds(&points, 0, 1));
        }

        {
            let points: [Point; _] = [
                point!(2, 2),
                point!(3, 2),
                point!(4, 2),
                point!(5, 2),
                point!(6, 2),
            ];
            assert!(is_rect_is_in_bounds(&points, 0, 4));
            assert!(is_rect_is_in_bounds(&points, 4, 0));
        }

        {
            let points: [Point; _] = [
                point!(1, 1),
                point!(2, 1),
                point!(2, 9),
                point!(9, 9),
                point!(9, 1),
                point!(10, 1),
                point!(10, 10),
                point!(1, 10),
            ];
            assert!(!is_rect_is_in_bounds(&points, 1, 3));
            assert!(!is_rect_is_in_bounds(&points, 3, 1));
            assert!(is_rect_is_in_bounds(&points, 1, 7));
            assert!(is_rect_is_in_bounds(&points, 7, 1));
        }
    }
}
