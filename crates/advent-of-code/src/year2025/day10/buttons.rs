use anyhow::{Result, anyhow};

use super::machine_spec::MachineSpec;

fn compute_combination_count(mut n: usize, m: usize) -> Result<u64> {
    if n > m {
        return Err(anyhow!("Choose count must be <= total count"));
    }

    if n > m / 2 {
        n = m - n;
    }
    let mut product = 1u64;
    for q in (m - n + 1)..=m {
        product = product
            .checked_mul(q as u64)
            .ok_or_else(|| anyhow!("Overflow computing combination count"))?;
    }
    for q in 1..=n {
        product = product
            .checked_div(q as u64)
            .ok_or_else(|| anyhow!("Overflow computing combination count"))?;
    }

    Ok(product)
}

fn does_set_solve_machine(machine_spec: &MachineSpec, indices: &[usize]) -> bool {
    indices.iter().fold(0u32, |acc, btn_idx| {
        acc ^ machine_spec.button_wiring_schematics[*btn_idx]
    }) == machine_spec.indicator_lights
}

fn can_solve_machine_with_button_count(machine_spec: &MachineSpec, count: usize) -> Result<bool> {
    let btn_count = machine_spec.button_wiring_schematics.len();
    let permutation_count = compute_combination_count(count, btn_count)?;
    let mut indices = Vec::from_fn(count, |idx| idx);

    for _ in 0..permutation_count {
        if does_set_solve_machine(machine_spec, &indices) {
            return Ok(true);
        }

        for rank in 0..count {
            let next_idx = count - 1 - rank;
            if indices[next_idx] < btn_count - 1 - rank {
                let val = indices[next_idx] + 1;
                for q in 0..count - next_idx {
                    indices[next_idx + q] = val + q;
                }
                break;
            }
        }
    }

    Ok(false)
}

pub(super) fn compute_minimum_buttons_pressed(machine_spec: &MachineSpec) -> Result<Option<u32>> {
    if machine_spec.indicator_lights == 0 {
        return Ok(Some(0));
    }

    for count in 1..=machine_spec.button_wiring_schematics.len() {
        if can_solve_machine_with_button_count(machine_spec, count)? {
            return Ok(Some(count as u32));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::super::machine_spec::parse_machine_spec;

    use super::*;

    #[test]
    fn test_compute_combination_count() {
        assert_eq!(compute_combination_count(0, 3).unwrap(), 1);
        assert_eq!(compute_combination_count(1, 3).unwrap(), 3);
        assert_eq!(compute_combination_count(2, 3).unwrap(), 3);
        assert_eq!(compute_combination_count(3, 3).unwrap(), 1);

        assert_eq!(compute_combination_count(0, 4).unwrap(), 1);
        assert_eq!(compute_combination_count(1, 4).unwrap(), 4);
        assert_eq!(compute_combination_count(2, 4).unwrap(), 6);
        assert_eq!(compute_combination_count(3, 4).unwrap(), 4);
        assert_eq!(compute_combination_count(4, 4).unwrap(), 1);

        assert_eq!(compute_combination_count(0, 5).unwrap(), 1);
        assert_eq!(compute_combination_count(1, 5).unwrap(), 5);
        assert_eq!(compute_combination_count(2, 5).unwrap(), 10);
        assert_eq!(compute_combination_count(3, 5).unwrap(), 10);
        assert_eq!(compute_combination_count(4, 5).unwrap(), 5);
        assert_eq!(compute_combination_count(5, 5).unwrap(), 1);

        assert_eq!(compute_combination_count(0, 6).unwrap(), 1);
        assert_eq!(compute_combination_count(1, 6).unwrap(), 6);
        assert_eq!(compute_combination_count(2, 6).unwrap(), 15);
        assert_eq!(compute_combination_count(3, 6).unwrap(), 20);
        assert_eq!(compute_combination_count(4, 6).unwrap(), 15);
        assert_eq!(compute_combination_count(5, 6).unwrap(), 6);
        assert_eq!(compute_combination_count(6, 6).unwrap(), 1);
    }

    #[test]
    fn test_does_set_solve_machine() {
        {
            let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
            let machine_spec = parse_machine_spec(input).unwrap();
            assert!(does_set_solve_machine(&machine_spec, &[1, 3]));
            assert!(does_set_solve_machine(&machine_spec, &[4, 5]));
        }

        {
            let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
            let machine_spec = parse_machine_spec(input).unwrap();
            assert!(does_set_solve_machine(&machine_spec, &[2, 3, 4]));
        }

        {
            let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
            let machine_spec = parse_machine_spec(input).unwrap();
            assert!(does_set_solve_machine(&machine_spec, &[1, 2]));
        }
    }

    #[test]
    fn test_compute_minimum_buttons_pressed() {
        {
            let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
            let machine_spec = parse_machine_spec(input).unwrap();
            assert_eq!(
                compute_minimum_buttons_pressed(&machine_spec)
                    .unwrap()
                    .unwrap(),
                2
            );
        }

        {
            let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
            let machine_spec = parse_machine_spec(input).unwrap();
            assert_eq!(
                compute_minimum_buttons_pressed(&machine_spec)
                    .unwrap()
                    .unwrap(),
                3
            );
        }

        {
            let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
            let machine_spec = parse_machine_spec(input).unwrap();
            assert_eq!(
                compute_minimum_buttons_pressed(&machine_spec)
                    .unwrap()
                    .unwrap(),
                2
            );
        }
    }
}
