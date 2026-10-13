use super::point::Point;

pub(super) fn rect_size(p1: Point, p2: Point) -> u64 {
    let wid = (i64::from(p2.x) - i64::from(p1.x)).unsigned_abs() + 1;
    let hit = (i64::from(p2.y) - i64::from(p1.y)).unsigned_abs() + 1;
    wid * hit
}

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
}
