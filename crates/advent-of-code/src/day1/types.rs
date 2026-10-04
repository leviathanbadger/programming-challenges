#[derive(Debug, Clone, Copy)]
pub enum DialRotation {
    Left {by: i32},
    Right {by: i32}
}

#[doc(hidden)]
pub(crate) const fn parse_rotation(rot_str: &str) -> DialRotation {
    let bytes = rot_str.as_bytes();

    if bytes.len() == 0 {
        panic!("Empty string!");
    }

    let first = bytes[0] as char;

    let mut by_amount: i32 = 0;
    let mut idx = 1;
    while idx < bytes.len() {
        let bit = bytes[idx];
        idx += 1;
        by_amount *= 10 as i32;
        by_amount += (bit - b'0') as i32;
    }

    match first {
        'L' => DialRotation::Left { by: by_amount },
        'R' => DialRotation::Right { by: by_amount },
        _ => panic!("Unrecognized direction")
    }
}

macro_rules! day1_dial_rotations {
    [ $( $rotation: ident )* ] => {
        [
            $({
                const ROTATION: DialRotation = $crate::day1::types::parse_rotation(stringify!($rotation));
                ROTATION
            }),*
        ]
    };
}

pub(crate) use day1_dial_rotations;
