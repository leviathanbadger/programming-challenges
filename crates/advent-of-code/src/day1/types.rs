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
        let chrval = (bit - b'0') as i32;
        if chrval < 0 || chrval > 9 {
            panic!("Expected decimal digit")
        }

        idx += 1;
        by_amount = by_amount.checked_mul(10).unwrap();
        by_amount = by_amount.checked_add(chrval).unwrap();
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
                const ROTATION: $crate::day1::types::DialRotation = $crate::day1::types::parse_rotation(stringify!($rotation));
                ROTATION
            }),*
        ]
    };
}

pub(crate) use day1_dial_rotations;
