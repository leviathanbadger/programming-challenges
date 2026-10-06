static INPUT_RAW: &str = include_str!("input.txt");

pub(super) fn get_input() -> Vec<&'static str> {
    INPUT_RAW.split_whitespace().collect::<Vec<_>>()
}
