mod input;
mod types;

use anyhow::{Result, anyhow};
use input::INPUT;
use types::DialRotation;

fn count_dial_rotations(
    start: i32,
    dial_max: i32,
    count_val: i32,
    instructions: &[DialRotation],
) -> Result<u32> {
    if dial_max < 1 {
        return Err(anyhow!("dial_max must be >= 1"));
    }
    if start < 0 || start >= dial_max {
        return Err(anyhow!("start must be within [0, dial_max)"));
    }
    if count_val < 0 || count_val >= dial_max {
        return Err(anyhow!("count_val must be within [0, dial_max)"));
    }

    let mut count = 0;

    let mut current = start as i64;
    for instr in instructions {
        current += match instr {
            DialRotation::Left { by: amount } => -i64::from(*amount),
            DialRotation::Right { by: amount } => i64::from(*amount),
        };
        current = current.rem_euclid(dial_max as i64);
        if current == count_val as i64 {
            count += 1;
        }
    }

    Ok(count)
}

pub fn compute() -> Result<u32> {
    count_dial_rotations(50, 100, 0, INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;
    use types::day1_dial_rotations;

    #[test]
    fn test_invalid_input() {
        let instructions = day1_dial_rotations![
            L1
            R1
        ];

        {
            let result = count_dial_rotations(50, 0, 0, &instructions);
            assert_matches!(result, Err(err) if err.to_string().contains("dial_max"));
        }

        {
            let result = count_dial_rotations(50, -1, 0, &instructions);
            assert_matches!(result, Err(err) if err.to_string().contains("dial_max"));
        }

        {
            let result = count_dial_rotations(-1, 100, 0, &instructions);
            assert_matches!(result, Err(err) if err.to_string().contains("start"));
        }

        {
            let result = count_dial_rotations(50, 30, 0, &instructions);
            assert_matches!(result, Err(err) if err.to_string().contains("start"));
        }

        {
            let result = count_dial_rotations(50, 100, -1, &instructions);
            assert_matches!(result, Err(err) if err.to_string().contains("count_val"));
        }

        {
            let result = count_dial_rotations(50, 100, 100, &instructions);
            assert_matches!(result, Err(err) if err.to_string().contains("count_val"));
        }
    }

    #[test]
    fn test_empty_instructions() {
        let instructions = day1_dial_rotations![];

        let count = count_dial_rotations(50, 100, 0, &instructions).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_land_bidi() {
        let instructions = day1_dial_rotations![
            L50
            R100
        ];

        let count = count_dial_rotations(50, 100, 0, &instructions).unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_land_full_revolution() {
        let instructions = day1_dial_rotations![
            R410
            L260
            L430
            R230
        ];

        let count = count_dial_rotations(50, 100, 0, &instructions).unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_sample_dial_rotations() {
        let instructions = day1_dial_rotations![
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        ];

        let count = count_dial_rotations(50, 100, 0, &instructions).unwrap();
        assert_eq!(count, 3);
    }
}
