mod types;
mod input;

use anyhow::*;
use types::DialRotation;
use input::INPUT;

fn count_dial_rotations(start: i32, dial_max: i32, count_val: i32, instructions: &[DialRotation]) -> Result<u32> {
    let mut count = 0;

    let mut current = start;
    for instr in instructions {
        current += match instr {
            DialRotation::Left { by: amount } => -*amount,
            DialRotation::Right { by: amount } => *amount
        };
        while current < 0 {
            current += dial_max;
        }
        current = current % dial_max;
        if current == count_val {
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
    use types::day1_dial_rotations;

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
