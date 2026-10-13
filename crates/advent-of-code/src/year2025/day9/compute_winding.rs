use anyhow::{Result, anyhow};

use super::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Winding {
    Counterclockwise,
    Indeterminate,
    Clockwise,
}

/// Computes the winding of the polygon described by the passed-in points; positive Y = down.
/// Overflows from the math calculations result in Winding::Indeterminate.
pub(super) fn compute_winding(points: &[Point]) -> Winding {
    let sum = points
        .iter()
        .enumerate()
        .try_fold(0i64, |acc, (curr_idx, &curr)| {
            let next = points[(curr_idx + 1) % points.len()];
            i64::from(next.x)
                .checked_sub(i64::from(curr.x))
                .and_then(|xdiff| {
                    i64::from(next.y)
                        .checked_add(i64::from(curr.y))
                        .and_then(|ysum| xdiff.checked_mul(ysum))
                })
                .and_then(|val| acc.checked_add(val))
        });

    match sum {
        None | Some(0) => Winding::Indeterminate,
        Some(x) if x < 0 => Winding::Clockwise,
        _ => Winding::Counterclockwise,
    }
}

pub(super) fn compute_winding_discontiguous(
    points: &[Point],
    start_idx: usize,
    end_idx: usize,
) -> Result<Winding> {
    if start_idx >= points.len() || end_idx >= points.len() {
        return Err(anyhow!("One or more indices out of bounds"));
    }

    if end_idx >= start_idx {
        return Ok(compute_winding(&points[start_idx..=end_idx]));
    }

    let start_points = &points[start_idx..];
    let end_points = &points[..=end_idx];
    // TODO MAYBE - if performance is a problem, implement special case here instead of copying the ranges to a new collection
    let points_view = start_points
        .iter()
        .chain(end_points.iter())
        .copied()
        .collect::<Vec<_>>();
    Ok(compute_winding(&points_view))
}

#[cfg(test)]
mod tests {
    use super::super::point::point;
    use super::*;

    #[test]
    fn test_compute_winding() {
        {
            let points: [Point; _] = [];
            assert_eq!(compute_winding(&points), Winding::Indeterminate);
        }

        {
            let points: [Point; _] = [point!(2, 2)];
            assert_eq!(compute_winding(&points), Winding::Indeterminate);
        }

        {
            let points: [Point; _] = [point!(2, 2), point!(4, 4)];
            assert_eq!(compute_winding(&points), Winding::Indeterminate);
        }

        {
            let points: [Point; _] = [point!(2, 2), point!(4, 2)];
            assert_eq!(compute_winding(&points), Winding::Indeterminate);
        }

        {
            let points: [Point; _] = [point!(2, 2), point!(4, 2), point!(4, 4)];
            assert_eq!(compute_winding(&points), Winding::Clockwise);
        }

        {
            let points: [Point; _] = [point!(2, 2), point!(4, 2), point!(4, 4), point!(2, 4)];
            assert_eq!(compute_winding(&points), Winding::Clockwise);
        }

        {
            let points: [Point; _] = [point!(2, 2), point!(4, 2), point!(4, 0)];
            assert_eq!(compute_winding(&points), Winding::Counterclockwise);
        }

        {
            let points: [Point; _] = [point!(2, 2), point!(4, 2), point!(4, 0), point!(2, 0)];
            assert_eq!(compute_winding(&points), Winding::Counterclockwise);
        }

        {
            let mut points: [Point; _] = [
                point!(7, 1),
                point!(11, 1),
                point!(11, 7),
                point!(9, 7),
                point!(9, 5),
                point!(2, 5),
                point!(2, 3),
                point!(7, 3),
            ];
            assert_eq!(compute_winding(&points), Winding::Clockwise);
            points.reverse();
            assert_eq!(compute_winding(&points), Winding::Counterclockwise);
        }
    }
}
