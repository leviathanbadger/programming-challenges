use std::fmt::Debug;

use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone, Copy)]
pub enum DialRotation {
    Left { by: i32 },
    Right { by: i32 },
}

#[doc(hidden)]
pub(crate) fn parse_rotation(rot_str: &str) -> Result<DialRotation> {
    let bytes = rot_str.as_bytes();

    if bytes.len() < 2 {
        return Err(anyhow!("Empty/malformed string!"));
    }

    let first = bytes[0] as char;

    let mut by_amount: i32 = 0;
    let mut idx = 1;
    while idx < bytes.len() {
        let bit = bytes[idx];
        if !bit.is_ascii_digit() {
            return Err(anyhow!("Expected decimal digit"));
        }
        let chrval = (bit - b'0') as i32;

        idx += 1;
        by_amount = by_amount
            .checked_mul(10)
            .ok_or_else(|| anyhow!("Rotation amount exceeds i32::MAX"))?;
        by_amount = by_amount
            .checked_add(chrval)
            .ok_or_else(|| anyhow!("Rotation amount exceeds i32::MAX"))?;
    }

    if by_amount == 0 {
        return Err(anyhow!("Rotations must be >0 turn"));
    }

    match first {
        'L' => Ok(DialRotation::Left { by: by_amount }),
        'R' => Ok(DialRotation::Right { by: by_amount }),
        _ => Err(anyhow!("Unrecognized direction")),
    }
}

pub(crate) fn parse_rotations(input: &str) -> Result<Vec<DialRotation>> {
    input
        .split_whitespace()
        .map(|token| {
            parse_rotation(token).with_context(|| format!("Faulty rotation: \"{}\"", token))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_parse_rotation_rejects_invalid() {
        {
            let result = parse_rotation("");
            assert_matches!(result, Err(err) if err.to_string().eq("Empty/malformed string!"));
        }

        {
            let result = parse_rotation("q");
            assert_matches!(result, Err(err) if err.to_string().eq("Empty/malformed string!"));
        }

        {
            let result = parse_rotation("L");
            assert_matches!(result, Err(err) if err.to_string().eq("Empty/malformed string!"));
        }

        {
            let result = parse_rotation("Rz");
            assert_matches!(result, Err(err) if err.to_string().eq("Expected decimal digit"));
        }

        {
            let result = parse_rotation("A24");
            assert_matches!(result, Err(err) if err.to_string().eq("Unrecognized direction"));
        }

        {
            let result = parse_rotation("R0");
            assert_matches!(result, Err(err) if err.to_string().eq("Rotations must be >0 turn"));
        }
    }

    #[test]
    fn test_parse_rotation_accepts_valid() {
        {
            let result = parse_rotation("L24");
            assert_matches!(result, Ok(DialRotation::Left { by: 24 }));
        }

        {
            let result = parse_rotation("L0099265");
            assert_matches!(result, Ok(DialRotation::Left { by: 99265 }));
        }

        {
            let result = parse_rotation("R5");
            assert_matches!(result, Ok(DialRotation::Right { by: 5 }));
        }

        {
            let result = parse_rotation("R90");
            assert_matches!(result, Ok(DialRotation::Right { by: 90 }));
        }
    }
}
