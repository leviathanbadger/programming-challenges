use super::machine_spec::MachineSpec;

fn does_set_solve_machine(machine_spec: &MachineSpec, indices: &[usize]) -> bool {
    indices.iter().fold(0u32, |acc, btn_idx| {
        acc ^ machine_spec.button_wiring_schematics[*btn_idx]
    }) == machine_spec.indicator_lights
}

fn advance_combination(indices: &mut [usize], choose_count: usize) -> bool {
    let count = indices.len();

    for rank in 0..count {
        let next_idx = count - 1 - rank;
        if indices[next_idx] < choose_count - 1 - rank {
            let val = indices[next_idx] + 1;
            for q in 0..count - next_idx {
                indices[next_idx + q] = val + q;
            }
            return true;
        }
    }

    false
}

fn can_solve_machine_with_button_count(machine_spec: &MachineSpec, count: usize) -> bool {
    let btn_count = machine_spec.button_wiring_schematics.len();
    let mut indices = Vec::from_fn(count, |idx| idx);

    loop {
        if does_set_solve_machine(machine_spec, &indices) {
            return true;
        }

        if !advance_combination(&mut indices, btn_count) {
            return false;
        }
    }
}

pub(super) fn compute_minimum_buttons_pressed(machine_spec: &MachineSpec) -> Option<u32> {
    if machine_spec.indicator_lights == 0 {
        return Some(0);
    }

    for count in 1..=machine_spec.button_wiring_schematics.len() {
        if can_solve_machine_with_button_count(machine_spec, count) {
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
            assert_eq!(compute_minimum_buttons_pressed(&machine_spec).unwrap(), 2);
        }

        {
            let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
            let machine_spec = parse_machine_spec(input).unwrap();
            assert_eq!(compute_minimum_buttons_pressed(&machine_spec).unwrap(), 3);
        }

        {
            let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
            let machine_spec = parse_machine_spec(input).unwrap();
            assert_eq!(compute_minimum_buttons_pressed(&machine_spec).unwrap(), 2);
        }
    }
}
