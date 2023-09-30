mod crypto;

use clap::{Parser, ValueEnum};
use std::num::ParseIntError;

fn main() {
    let cli = Cli::parse();

    let res = run_example(cli.example, cli.part);

    println!("Result: {}", res);
}

#[derive(ValueEnum, Clone, PartialEq, Eq)]
enum Example {
    #[value(alias = "s")]
    Small,

    #[value(alias = "b")]
    Big,
}

#[derive(ValueEnum, Clone, PartialEq, Eq)]
enum Part {
    #[value(alias = "b", alias = "1")]
    Basic,

    #[value(alias = "a", alias = "2")]
    Advanced,
}

#[derive(Parser)]
struct Cli {
    #[arg(value_enum)]
    example: Example,

    #[arg(value_enum)]
    part: Part,
}

fn parse_input(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split('\n').map(|l| l.parse::<i64>()).collect()
}

const EXAMPLE_1: &str = include_str!("ex1.txt");
const EXAMPLE_2: &str = include_str!("ex2.txt");

fn run_example(ex: Example, part: Part) -> i64 {
    let input = match ex {
        Example::Small => EXAMPLE_1,
        Example::Big => EXAMPLE_2,
    };

    let seq = parse_input(input).unwrap();

    match part {
        Part::Basic => crypto::decrypt(seq, 1, 1),
        Part::Advanced => crypto::decrypt(seq, 811589153, 10),
    }
}

#[cfg(test)]
mod tests {
    use crate::{run_example, Example, Part};

    #[test]
    fn part_1_small() {
        assert_eq!(run_example(Example::Small, Part::Basic), 3);
    }

    #[test]
    fn part_1_big() {
        assert_eq!(run_example(Example::Big, Part::Basic), 11037);
    }

    #[test]
    fn part_2_small() {
        assert_eq!(run_example(Example::Small, Part::Advanced), 1623178306);
    }

    #[test]
    fn part_2_big() {
        assert_eq!(run_example(Example::Big, Part::Advanced), 3033720253914);
    }
}
