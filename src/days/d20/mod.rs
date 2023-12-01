mod crypto;

use std::num::ParseIntError;

fn parse_input(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split('\n').map(|l| l.parse::<i64>()).collect()
}

pub fn run_example(input: &str, part2: bool) -> i64 {
    let seq = parse_input(input).unwrap();

    if part2 {
        crypto::decrypt(seq, 811589153, 10)
    } else {
        crypto::decrypt(seq, 1, 1)
    }
}

pub struct Day;

impl super::Runner for Day {
    type T = i64;
    
    fn run(input: &str, basic: bool) -> Self::T {
        run_example(input, !basic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("ex1.txt");
    const EXAMPLE_2: &str = include_str!("ex2.txt");
    
    #[test]
    fn part_1_small() {
        assert_eq!(run_example(EXAMPLE_1, false), 3);
    }

    #[test]
    fn part_1_big() {
        assert_eq!(run_example(EXAMPLE_2, false), 11037);
    }

    #[test]
    fn part_2_small() {
        assert_eq!(run_example(EXAMPLE_1, true), 1623178306);
    }

    #[test]
    fn part_2_big() {
        assert_eq!(run_example(EXAMPLE_2, true), 3033720253914);
    }
}
