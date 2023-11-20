mod crypto;

use std::num::ParseIntError;

fn parse_input(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split('\n').map(|l| l.parse::<i64>()).collect()
}

const EXAMPLE_1: &str = include_str!("ex1.txt");
const EXAMPLE_2: &str = include_str!("ex2.txt");

pub fn run_example(big: bool, part2: bool) -> i64 {
    let input = if big { EXAMPLE_2 } else { EXAMPLE_1 };

    let seq = parse_input(input).unwrap();

    if part2 {
        crypto::decrypt(seq, 811589153, 10)
    } else {
        crypto::decrypt(seq, 1, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_small() {
        assert_eq!(run_example(false, false), 3);
    }

    #[test]
    fn part_1_big() {
        assert_eq!(run_example(true, false), 11037);
    }

    #[test]
    fn part_2_small() {
        assert_eq!(run_example(false, true), 1623178306);
    }

    #[test]
    fn part_2_big() {
        assert_eq!(run_example(true, true), 3033720253914);
    }
}
