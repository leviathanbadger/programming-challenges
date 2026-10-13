#[derive(Debug, Clone, Copy)]
pub enum DialRotation {
    Left { by: i32 },
    Right { by: i32 },
}

#[doc(hidden)]
pub(crate) const fn parse_rotation(rot_str: &str) -> DialRotation {
    let bytes = rot_str.as_bytes();

    if bytes.len() < 2 {
        panic!("Empty/malformed string!");
    }

    let first = bytes[0] as char;

    let mut by_amount: i32 = 0;
    let mut idx = 1;
    while idx < bytes.len() {
        let bit = bytes[idx];
        if !bit.is_ascii_digit() {
            panic!("Expected decimal digit")
        }
        let chrval = (bit - b'0') as i32;

        idx += 1;
        by_amount = by_amount
            .checked_mul(10)
            .expect("Rotation amount exceeds i32::MAX");
        by_amount = by_amount
            .checked_add(chrval)
            .expect("Rotation amount exceeds i32::MAX");
    }

    match first {
        'L' => DialRotation::Left { by: by_amount },
        'R' => DialRotation::Right { by: by_amount },
        _ => panic!("Unrecognized direction"),
    }
}

macro_rules! day1_dial_rotations {
    [ $( $rotation: ident )* ] => {
        [
            $({
                const ROTATION: $crate::year2025::day1part1::types::DialRotation = $crate::year2025::day1part1::types::parse_rotation(stringify!($rotation));
                ROTATION
            }),*
        ]
    };
}

pub(super) use day1_dial_rotations;

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    #[should_panic(expected = "Empty/malformed string!")]
    fn test_parse_rotation_rejects_empty() {
        parse_rotation("");
    }

    #[test]
    #[should_panic(expected = "Empty/malformed string!")]
    fn test_parse_rotation_rejects_malformed() {
        parse_rotation("q");
    }

    #[test]
    #[should_panic(expected = "Empty/malformed string!")]
    fn test_parse_rotation_rejects_malformed_no_number() {
        parse_rotation("L");
    }

    #[test]
    #[should_panic(expected = "Expected decimal digit")]
    fn test_parse_rotation_rejects_malformed_number() {
        parse_rotation("Rz");
    }

    #[test]
    #[should_panic(expected = "Unrecognized direction")]
    fn test_parse_rotation_rejects_malformed_direction() {
        parse_rotation("A24");
    }

    #[test]
    fn test_parse_rotation_accepts_valid() {
        {
            let rot = parse_rotation("L24");
            assert_matches!(rot, DialRotation::Left { by: 24 });
        }

        {
            let rot = parse_rotation("L0099265");
            assert_matches!(rot, DialRotation::Left { by: 99265 });
        }

        {
            let rot = parse_rotation("R0");
            assert_matches!(rot, DialRotation::Right { by: 0 });
        }

        {
            let rot = parse_rotation("R90");
            assert_matches!(rot, DialRotation::Right { by: 90 });
        }
    }
}
