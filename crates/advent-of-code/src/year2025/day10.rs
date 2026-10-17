mod input;
mod machine_spec;
mod part1;
mod part2;

use anyhow::{Context, Result};
use input::get_input;
use machine_spec::MachineSpec;
use part1::compute_minimum_buttons_pressed;
use part2::compute_minimum_buttons_pressed_for_joltage;
use shared_utils::CheckedSum;

fn sum_minimum_buttons_pressed(machine_specs: &[MachineSpec]) -> Result<u32> {
    machine_specs
        .iter()
        .enumerate()
        .map(|(idx, machine)| {
            compute_minimum_buttons_pressed(machine)
                .with_context(|| format!("Machine {idx} could not be solved"))
        })
        .checked_sum()
}

fn sum_minimum_buttons_pressed_for_joltage(machine_specs: &[MachineSpec]) -> Result<u32> {
    machine_specs
        .iter()
        .enumerate()
        .map(|(idx, machine)| {
            let min_pressed = compute_minimum_buttons_pressed_for_joltage(machine)
                .with_context(|| format!("Machine {idx} could not be solved"))?;
            println!("Solved {idx}: {min_pressed}");
            Ok::<_, anyhow::Error>(min_pressed)
        })
        .checked_sum()
}

pub(super) fn compute_part1() -> Result<u32> {
    let machine_specs = get_input()?;
    sum_minimum_buttons_pressed(&machine_specs)
}

pub(super) fn compute_part2() -> Result<u32> {
    let machine_specs = get_input()?;
    sum_minimum_buttons_pressed_for_joltage(&machine_specs)
}

#[cfg(test)]
mod tests {
    use super::input::parse_puzzle_input;
    use super::*;

    const SAMPLE_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1_sample() {
        let machine_specs = parse_puzzle_input(SAMPLE_INPUT).unwrap();
        let sum = sum_minimum_buttons_pressed(&machine_specs).unwrap();
        assert_eq!(sum, 7);
    }

    #[test]
    fn test_part2_sample() {
        let machine_specs = parse_puzzle_input(SAMPLE_INPUT).unwrap();
        let sum = sum_minimum_buttons_pressed_for_joltage(&machine_specs).unwrap();
        assert_eq!(sum, 33);
    }
}
