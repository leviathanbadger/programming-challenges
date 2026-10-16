use anyhow::{Result, anyhow};

fn parse_indicator_lights(text: &str) -> Result<(usize, u32)> {
    if text.len() < 3 || !text.starts_with("[") || !text.ends_with("]") {
        return Err(anyhow!("Could not parse indicator lights from \"{text}\""));
    }

    let lights = text.as_bytes()[1..text.len() - 1]
        .iter()
        .copied()
        .map(|value| match value {
            b'.' => Ok(false),
            b'#' => Ok(true),
            _ => Err(anyhow!("Could not parse IndicatorLight from u8 {value}")),
        })
        .collect::<Result<Vec<_>>>()?;

    if lights.len() > 32 {
        return Err(anyhow!("Can't parse indicator lights - max length is 32"));
    }

    Ok((
        text.len() - 2,
        lights
            .iter()
            .rev()
            .fold(0u32, |acc, next| (acc << 1) | (*next as u32)),
    ))
}

/// Assumes text is at least 2 characters long
fn remove_first_last(text: &str) -> &str {
    debug_assert!(text.len() >= 2);

    let mut chrs = text.chars();
    chrs.next();
    chrs.next_back();
    chrs.as_str()
}

fn parse_button_wiring_schematic(text: &str, light_count: usize) -> Result<u32> {
    if text.len() < 3 || !text.starts_with("(") || !text.ends_with(")") {
        return Err(anyhow!(
            "Could not parse button wiring schematic from \"{text}\""
        ));
    }

    remove_first_last(text)
        .split(',')
        .map(|num_str| num_str.trim().parse::<usize>().map_err(|e| anyhow!(e)))
        .try_fold(0u32, |acc, next_result| {
            next_result.and_then(|btn_idx| {
                if btn_idx >= light_count {
                    Err(anyhow!(
                        "Button index refers to nonexistant indicator light!"
                    ))
                } else if acc & (1 << btn_idx) != 0 {
                    Err(anyhow!(
                        "Duplicate button index {btn_idx} - ambiguous desired behavior, failing fast"
                    ))
                } else {
                    Ok(acc | (1 << btn_idx))
                }
            })
        })
}

fn parse_joltage_requirements(text: &str, light_count: usize) -> Result<Vec<u32>> {
    if text.len() < 3 || !text.starts_with("{") || !text.ends_with("}") {
        return Err(anyhow!(
            "Could not parse joltage requirements from \"{text}\""
        ));
    }

    let joltage_requirements = remove_first_last(text)
        .split(',')
        .map(|num_str| num_str.trim().parse::<u32>().map_err(|e| anyhow!(e)))
        .collect::<Result<Vec<_>>>()?;

    if joltage_requirements.len() != light_count {
        Err(anyhow!(
            "Joltage requirements must contain a number for each light."
        ))
    } else {
        Ok(joltage_requirements)
    }
}

#[derive(Debug)]
pub(super) struct MachineSpec {
    #[allow(
        dead_code,
        reason = "May be used in part 2; regardless, needed to correctly reproduce input"
    )]
    pub light_count: usize,
    pub indicator_lights: u32,
    pub button_wiring_schematics: Vec<u32>,
    #[allow(dead_code, reason = "Will be used in part 2 presumably")]
    pub joltage_requirements: Vec<u32>,
}

pub(super) fn parse_machine_spec(line: &str) -> Result<MachineSpec> {
    // TODO MAYBE; don't split on ' ', actually parse a syntax; but this will work for
    // the input and sample input, so maybe just bake it in as a requirement
    let segments = line.split(' ').collect::<Vec<_>>();
    match segments.as_slice() {
        [first, middle @ .., last] => {
            if middle.is_empty() {
                return Err(anyhow!(
                    "There must be at least one button schematic for each machine spec."
                ));
            }

            let (light_count, indicator_lights) = parse_indicator_lights(first)?;
            let joltage_requirements = parse_joltage_requirements(last, light_count)?;

            let button_wiring_schematics = middle
                .iter()
                .map(|text| parse_button_wiring_schematic(text, light_count))
                .collect::<Result<_>>()?;

            Ok(MachineSpec {
                light_count,
                indicator_lights,
                button_wiring_schematics,
                joltage_requirements,
            })
        }
        _ => Err(anyhow!("Could not parse machine spec. Line: {line}")),
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_parse_indicator_lights_valid() {
        let valid_input: [(&str, (usize, u32)); _] = [
            ("[.]", (1, 0)),
            ("[#]", (1, 1)),
            ("[.....]", (5, 0)),
            ("[#....]", (5, 0b1)),
            ("[....#]", (5, 0b10000)),
            ("[.#..]", (4, 0b0010)),
            ("[#.##]", (4, 0b1101)),
            ("[.##.]", (4, 0b0110)),
            ("[...#.]", (5, 0b01000)),
            ("[.###.#]", (6, 0b101110)),
            ("[####.###.#####.##.##.#.#.####.##]", (32, 0xDEADBEEF)),
        ];

        for (text, expected) in valid_input {
            let result = parse_indicator_lights(text).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_indicator_lights_malformed() {
        let malformed_input: [&str; _] = [
            "", "[", "]", "q", "(.)", "..#.#", "[..", "..]", "[]", "[z]", "[[]", "[]]", "[..\\#]",
            " [.]", "[.] ", " [.] ",
        ];

        for text in malformed_input {
            let result = parse_indicator_lights(text);
            assert_matches!(result, Err(e) if e.to_string().contains("Could not parse"));
        }
    }

    #[test]
    fn test_parse_indicator_lights_overflow() {
        let result = parse_indicator_lights("[.....##...####........#.###...#..]");
        assert_matches!(result, Err(e) if e.to_string().contains("max length is 32"));
    }

    #[test]
    fn test_parse_button_wiring_schematic_valid() {
        let valid_input: [((&str, usize), u32); _] = [
            (("(0)", 1), 0b1),
            (("(0)", 5), 0b1),
            (("(3)", 5), 0b1000),
            (("(  1 )", 5), 0b10),
            (("(0,1,3,4)", 5), 0b11011),
            (("(0,2,3,4)", 5), 0b11101),
            (("(2,3)", 5), 0b1100),
            (("(0,4)", 5), 0b10001),
            (("(0,1,2)", 5), 0b111),
            (("(1,2,3,4)", 5), 0b11110),
            (("(4, 3, 2, 1)", 5), 0b11110),
            (("( 0   , 2,3  ,4 )", 5), 0b11101),
        ];

        for ((text, count), expected) in valid_input {
            let result = parse_button_wiring_schematic(text, count).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_button_wiring_schematic_malformed() {
        let malformed_input: [&str; _] = [
            "", "(", ")", "q", "[0]", "0,2,4", "(0,2", "0,2)", "()", "(z)", "(()", "())", "(0\\1)",
            " (6)", "(5) ", " (4) ", "(-1)", "(0, -1)",
        ];

        for text in malformed_input {
            let result = parse_button_wiring_schematic(text, 10);
            assert_matches!(result, Err(e) if e.to_string().contains("Could not parse button wiring") || e.to_string().contains("invalid digit"));
        }
    }

    #[test]
    fn test_parse_button_wiring_schematic_out_of_range() {
        let result = parse_button_wiring_schematic("(0, 3, 4, 5)", 5);
        assert_matches!(result, Err(e) if e.to_string().contains("nonexistant indicator light"));
    }

    #[test]
    fn test_parse_button_wiring_schematic_duplicate() {
        let result = parse_button_wiring_schematic("(0, 3, 4, 3)", 5);
        assert_matches!(result, Err(e) if e.to_string().contains("Duplicate button index"));
    }

    #[test]
    fn test_parse_joltage_requirements_valid() {
        let valid_input: [((&str, usize), &[u32]); _] = [
            (("{25}", 1), &[25]),
            (("{   97 }", 1), &[97]),
            (("{  1, 2   ,3}", 3), &[1, 2, 3]),
            (("{3,5,4,7}", 4), &[3, 5, 4, 7]),
            (("{7,5,12,7,2}", 5), &[7, 5, 12, 7, 2]),
            (("{10,11,11,5,10,5}", 6), &[10, 11, 11, 5, 10, 5]),
        ];

        for ((text, count), expected) in valid_input {
            let result = parse_joltage_requirements(text, count).unwrap();
            assert_eq!(&result, expected);
        }
    }

    #[test]
    fn test_parse_joltage_requirements_malformed() {
        let malformed_input: [&str; _] = [
            "", "{", "}", "q", "(0)", "0,2,4", "{0,2", "0,2}", "{}", "{z}", "{{}", "{}}", "{0\\1}",
            " {6}", "{5} ", " {4} ", "{-1}", "{0, -1}",
        ];

        for text in malformed_input {
            let result = parse_joltage_requirements(text, 10);
            assert_matches!(result, Err(e) if e.to_string().contains("Could not parse joltage") || e.to_string().contains("invalid digit"));
        }
    }

    #[test]
    fn test_parse_joltage_requirements_incorrect_count() {
        {
            let result = parse_joltage_requirements("{25, 11}", 3);
            assert_matches!(result, Err(e) if e.to_string().contains("number for each light"));
        }

        {
            let result = parse_joltage_requirements("{25, 11, 67, 16}", 3);
            assert_matches!(result, Err(e) if e.to_string().contains("number for each light"));
        }
    }

    #[test]
    fn test_parse_machine_spec() {
        {
            let machine_spec =
                parse_machine_spec("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap();
            assert_eq!(machine_spec.light_count, 4);
            assert_eq!(machine_spec.indicator_lights, 0b0110);
            assert_eq!(
                &machine_spec.button_wiring_schematics,
                &[0b1000, 0b1010, 0b100, 0b1100, 0b101, 0b11]
            );
            assert_eq!(&machine_spec.joltage_requirements, &[3, 5, 4, 7]);
        }

        {
            let machine_spec =
                parse_machine_spec("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
                    .unwrap();
            assert_eq!(machine_spec.light_count, 5);
            assert_eq!(machine_spec.indicator_lights, 0b01000);
            assert_eq!(
                &machine_spec.button_wiring_schematics,
                &[0b11101, 0b1100, 0b10001, 0b111, 0b11110]
            );
            assert_eq!(&machine_spec.joltage_requirements, &[7, 5, 12, 7, 2]);
        }

        {
            let machine_spec = parse_machine_spec(
                "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
            )
            .unwrap();
            assert_eq!(machine_spec.light_count, 6);
            assert_eq!(machine_spec.indicator_lights, 0b101110);
            assert_eq!(
                &machine_spec.button_wiring_schematics,
                &[0b11111, 0b11001, 0b110111, 0b110]
            );
            assert_eq!(&machine_spec.joltage_requirements, &[10, 11, 11, 5, 10, 5]);
        }
    }
}
