mod input;
mod types;

use input::get_input;

use anyhow::Result;
use shared_utils::CheckedSum;
use types::MathProblem;

fn solve_and_sum(problems: &[MathProblem]) -> Result<u64> {
    problems.iter().map(|p| p.solve()).checked_sum()
}

pub fn compute() -> Result<u64> {
    let math_problems = get_input()?;
    solve_and_sum(&math_problems)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::types::parse_puzzle_input;

    #[test]
    fn test_sample() {
        let math_problems = parse_puzzle_input(
            "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ",
        )
        .unwrap();

        let sum = solve_and_sum(&math_problems).unwrap();
        assert_eq!(sum, 4277556);
    }
}
