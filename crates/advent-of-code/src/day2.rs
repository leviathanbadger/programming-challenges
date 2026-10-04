mod input;
mod types;

use anyhow::{Result, anyhow};
use input::get_input;
use types::DialRotation;

fn count_dial_rotations(start: i32, dial_max: i32, instructions: &[DialRotation]) -> Result<u32> {
    if dial_max < 1 {
        return Err(anyhow!("dial_max must be >= 1"));
    }
    if start < 0 || start >= dial_max {
        return Err(anyhow!("start must be within [0, dial_max)"));
    }

    let dial_max_i64 = i64::from(dial_max);

    let mut count = 0_u32;

    let mut current = start as i64;
    for instr in instructions {
        let add_amt = match instr {
            DialRotation::Left { by: amount } => -i64::from(*amount),
            DialRotation::Right { by: amount } => i64::from(*amount),
        };
        let prev = current;
        current += add_amt;
        let next_rot = current.div_euclid(dial_max_i64);
        current = current.rem_euclid(dial_max_i64);

        let mut add_count = (next_rot).abs() as u32;
        if prev == 0 && next_rot < 0 {
            add_count -= 1
        }
        if current == 0 && next_rot <= 0 {
            add_count += 1;
        }

        count += add_count;
    }

    Ok(count)
}

pub fn compute() -> Result<u32> {
    let input = get_input()?;
    count_dial_rotations(50, 100, &input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;
    use types::parse_rotations;

    #[test]
    fn test_invalid_input() {
        let instructions = parse_rotations("L1 R1").unwrap();

        {
            let result = count_dial_rotations(50, 0, &instructions);
            assert_matches!(result, Err(err) if err.to_string().contains("dial_max"));
        }

        {
            let result = count_dial_rotations(50, -1, &instructions);
            assert_matches!(result, Err(err) if err.to_string().contains("dial_max"));
        }

        {
            let result = count_dial_rotations(-1, 100, &instructions);
            assert_matches!(result, Err(err) if err.to_string().contains("start"));
        }

        {
            let result = count_dial_rotations(50, 30, &instructions);
            assert_matches!(result, Err(err) if err.to_string().contains("start"));
        }
    }

    #[test]
    fn test_empty_instructions() {
        let instructions = parse_rotations("").unwrap();

        let count = count_dial_rotations(50, 100, &instructions).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_land_bidi() {
        let instructions = parse_rotations("L50 R100").unwrap();

        let count = count_dial_rotations(50, 100, &instructions).unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_land_full_revolution() {
        let instructions = parse_rotations("R410 L260 L430 R230").unwrap();

        let count = count_dial_rotations(50, 100, &instructions).unwrap();
        assert_eq!(count, 14);
    }

    #[test]
    fn test_sample_dial_rotations() {
        let instructions = parse_rotations("L68 L30 R48 L5 R60 L55 L1 L99 R14 L82").unwrap();

        let count = count_dial_rotations(50, 100, &instructions).unwrap();
        assert_eq!(count, 6);
    }
}
