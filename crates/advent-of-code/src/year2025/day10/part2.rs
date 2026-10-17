use shared_utils::CheckedSum;

use super::machine_spec::MachineSpec;

fn compute_max_button_presses(machine_spec: &MachineSpec) -> Vec<usize> {
    let btn_count = machine_spec.button_wiring_schematics.len();
    Vec::from_fn(btn_count, |btn_idx| {
        let btn = machine_spec.button_wiring_schematics[btn_idx];
        machine_spec
            .joltage_requirements
            .iter()
            .enumerate()
            .map(|(light_idx, joltage_req)| {
                if btn & (1 << light_idx) == 0 {
                    usize::MAX
                } else {
                    *joltage_req as usize
                }
            })
            .min()
            .unwrap()
    })
}

fn fill_from_left_with_max(
    btn_presses: &mut [usize],
    max_btn_presses: &[usize],
    start_idx: usize,
    fill_count: usize,
) -> bool {
    let mut remaining_count = fill_count;
    for (cell_btn_presses, cell_max_presses) in btn_presses
        .iter_mut()
        .zip(max_btn_presses.iter())
        .skip(start_idx)
    {
        let presses = remaining_count.min(*cell_max_presses);
        *cell_btn_presses = presses;
        remaining_count -= presses;
    }
    remaining_count == 0
}

fn does_set_solve_machine(
    machine_spec: &MachineSpec,
    btn_presses: &[usize],
    joltage: &mut [u32],
) -> bool {
    joltage.fill(0);

    for (btn_idx, btn) in machine_spec.button_wiring_schematics.iter().enumerate() {
        for (pos, jolt) in joltage.iter_mut().enumerate() {
            if btn & (1 << pos) != 0 {
                *jolt += btn_presses[btn_idx] as u32;
                if *jolt > machine_spec.joltage_requirements[pos] {
                    return false;
                }
            }
        }
    }

    joltage == machine_spec.joltage_requirements
}

fn advance_combination(btn_presses: &mut [usize], max_btn_presses: &[usize]) -> bool {
    let btn_count = btn_presses.len();

    let mut last_unfilled_idx = None;
    let mut reset_count = 0;
    for next_idx in (0..btn_count).rev() {
        reset_count += btn_presses[next_idx];
        if btn_presses[next_idx] < max_btn_presses[next_idx] {
            last_unfilled_idx = Some(next_idx);
            break;
        }
    }

    if last_unfilled_idx.is_none() {
        return false;
    }
    let last_unfilled_idx = last_unfilled_idx.unwrap();

    for next_idx in (0..last_unfilled_idx).rev() {
        if btn_presses[next_idx] > 0 {
            btn_presses[next_idx] -= 1;
            return fill_from_left_with_max(
                btn_presses,
                max_btn_presses,
                next_idx + 1,
                reset_count + 1,
            );
        }
    }

    false
}

fn can_solve_machine_with_button_count(
    machine_spec: &MachineSpec,
    count: usize,
    max_btn_presses: &[usize],
) -> bool {
    let light_count = machine_spec.light_count;
    let btn_count = machine_spec.button_wiring_schematics.len();

    // Scratch buffer allocated once, used for all does_set_solve_machine
    let mut joltage = vec![0; light_count];

    let mut btn_presses = vec![0; btn_count];
    if !fill_from_left_with_max(&mut btn_presses, max_btn_presses, 0, count) {
        return false;
    }

    loop {
        if does_set_solve_machine(machine_spec, &btn_presses, &mut joltage) {
            return true;
        }

        if !advance_combination(&mut btn_presses, max_btn_presses) {
            return false;
        }
    }
}

pub(super) fn compute_minimum_buttons_pressed_for_joltage(
    machine_spec: &MachineSpec,
) -> Option<u32> {
    if machine_spec.joltage_requirements.iter().all(|j| *j == 0) {
        return Some(0);
    }

    // Calculate once for all button computation checks
    let max_btn_presses = compute_max_button_presses(machine_spec);
    let max_btn_presses_total = max_btn_presses.iter().sum();

    for count in 1..=max_btn_presses_total {
        if can_solve_machine_with_button_count(machine_spec, count, &max_btn_presses) {
            return Some(count as u32);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::super::machine_spec::parse_machine_spec;

    use super::*;

    #[test]
    fn test_compute_max_button_presses() {
        {
            let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
            let machine_spec = parse_machine_spec(input).unwrap();
            let max_btn_presses = compute_max_button_presses(&machine_spec);
            assert_eq!(&max_btn_presses, &[7, 5, 4, 4, 3, 3]);
        }

        {
            let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
            let machine_spec = parse_machine_spec(input).unwrap();
            let max_btn_presses = compute_max_button_presses(&machine_spec);
            assert_eq!(&max_btn_presses, &[2, 7, 2, 5, 2]);
        }

        {
            let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
            let machine_spec = parse_machine_spec(input).unwrap();
            let max_btn_presses = compute_max_button_presses(&machine_spec);
            assert_eq!(&max_btn_presses, &[5, 5, 5, 11]);
        }

        {
            let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {0,11,11,5,10,5}";
            let machine_spec = parse_machine_spec(input).unwrap();
            let max_btn_presses = compute_max_button_presses(&machine_spec);
            assert_eq!(&max_btn_presses, &[0, 0, 0, 11]);
        }
    }

    #[test]
    fn test_fill_from_left_with_max() {
        {
            // Starting with 1 to ensure the function clears later cells back to 0
            let mut btn_presses = vec![1; 3];
            assert!(fill_from_left_with_max(&mut btn_presses, &[10, 3, 5], 0, 7));
            assert_eq!(&btn_presses, &[7, 0, 0]);
        }

        {
            let mut btn_presses = vec![0; 4];
            assert!(fill_from_left_with_max(
                &mut btn_presses,
                &[1, 0, 2, 5],
                0,
                8
            ));
            assert_eq!(&btn_presses, &[1, 0, 2, 5]);
        }

        {
            let mut btn_presses = vec![0; 5];
            assert!(fill_from_left_with_max(
                &mut btn_presses,
                &[1, 0, 2, 5, 3],
                0,
                5
            ));
            assert_eq!(&btn_presses, &[1, 0, 2, 2, 0]);
        }

        {
            let mut btn_presses = vec![0; 3];
            assert!(!fill_from_left_with_max(&mut btn_presses, &[3, 2, 1], 0, 7));
        }

        {
            // Starting with 1 to ensure the function clears later cells back to 0 but leave earlier cells alone
            let mut btn_presses = vec![1; 5];
            assert!(fill_from_left_with_max(
                &mut btn_presses,
                &[10, 3, 5, 1, 8],
                2,
                8
            ));
            assert_eq!(&btn_presses, &[1, 1, 5, 1, 2]);
        }

        {
            // Starting with 1 to ensure the function clears later cells back to 0 but leave earlier cells alone
            let mut btn_presses = vec![1; 5];
            assert!(fill_from_left_with_max(
                &mut btn_presses,
                &[10, 3, 5, 1, 8],
                2,
                2
            ));
            assert_eq!(&btn_presses, &[1, 1, 2, 0, 0]);
        }
    }

    #[test]
    fn test_advance_combination_one() {
        let mut btn_presses = vec![1, 0, 0, 0];
        let max_btn_presses = vec![3, 1, 0, 2];

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[0, 1, 0, 0]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[0, 0, 0, 1]);

        assert!(!advance_combination(&mut btn_presses, &max_btn_presses));
    }

    #[test]
    fn test_advance_combination_two() {
        let mut btn_presses = vec![2, 0, 0, 0];
        let max_btn_presses = vec![3, 1, 0, 2];

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[1, 1, 0, 0]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[1, 0, 0, 1]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[0, 1, 0, 1]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[0, 0, 0, 2]);

        assert!(!advance_combination(&mut btn_presses, &max_btn_presses));
    }

    #[test]
    fn test_advance_combination_three() {
        let mut btn_presses = vec![3, 0, 0, 0];
        let max_btn_presses = vec![3, 1, 0, 2];

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[2, 1, 0, 0]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[2, 0, 0, 1]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[1, 1, 0, 1]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[1, 0, 0, 2]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[0, 1, 0, 2]);

        assert!(!advance_combination(&mut btn_presses, &max_btn_presses));
    }

    #[test]
    fn test_advance_combination_four() {
        let mut btn_presses = vec![3, 1, 0, 0];
        let max_btn_presses = vec![3, 1, 0, 2];

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[3, 0, 0, 1]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[2, 1, 0, 1]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[2, 0, 0, 2]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[1, 1, 0, 2]);

        assert!(!advance_combination(&mut btn_presses, &max_btn_presses));
    }

    #[test]
    fn test_advance_combination_backtrack() {
        let mut btn_presses = vec![3, 0, 0, 0];
        let max_btn_presses = vec![3, 3, 0, 1];

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[2, 1, 0, 0]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[2, 0, 0, 1]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[1, 2, 0, 0]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[1, 1, 0, 1]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[0, 3, 0, 0]);

        assert!(advance_combination(&mut btn_presses, &max_btn_presses));
        assert_eq!(&btn_presses, &[0, 2, 0, 1]);

        assert!(!advance_combination(&mut btn_presses, &max_btn_presses));
    }

    #[test]
    fn test_does_set_solve_machine() {
        {
            let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
            let machine_spec = parse_machine_spec(input).unwrap();
            let mut joltage = vec![0u32; machine_spec.light_count];
            assert!(does_set_solve_machine(
                &machine_spec,
                &[1, 3, 0, 3, 1, 2],
                &mut joltage
            ));
        }

        {
            let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
            let machine_spec = parse_machine_spec(input).unwrap();
            let mut joltage = vec![0u32; machine_spec.light_count];
            assert!(does_set_solve_machine(
                &machine_spec,
                &[2, 5, 0, 5, 0],
                &mut joltage
            ));
        }

        {
            let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
            let machine_spec = parse_machine_spec(input).unwrap();
            let mut joltage = vec![0u32; machine_spec.light_count];
            assert!(does_set_solve_machine(
                &machine_spec,
                &[5, 0, 5, 1],
                &mut joltage
            ));
        }
    }

    #[test]
    fn test_compute_minimum_buttons_pressed_for_joltage() {
        {
            let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
            let machine_spec = parse_machine_spec(input).unwrap();
            assert_eq!(
                compute_minimum_buttons_pressed_for_joltage(&machine_spec).unwrap(),
                10
            );
        }

        {
            let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
            let machine_spec = parse_machine_spec(input).unwrap();
            assert_eq!(
                compute_minimum_buttons_pressed_for_joltage(&machine_spec).unwrap(),
                12
            );
        }

        {
            let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
            let machine_spec = parse_machine_spec(input).unwrap();
            assert_eq!(
                compute_minimum_buttons_pressed_for_joltage(&machine_spec).unwrap(),
                11
            );
        }
    }
}
