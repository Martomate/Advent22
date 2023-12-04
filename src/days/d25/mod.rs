use std::{collections::VecDeque, fmt::Display, iter::Sum};

#[derive(Debug, PartialEq, Eq)]
pub struct Snafu(i64);

impl TryFrom<&str> for Snafu {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut res: i64 = 0;
        for c in s.chars() {
            res *= 5;
            let n = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => return Err("invalid input"),
            };
            res += n;
        }
        Ok(Snafu(res))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut left = self.0;
        let mut stack = VecDeque::new();
        while left != 0 {
            let d = (left + 2) % 5 - 2;
            left = (left - d) / 5;

            stack.push_back(d);
        }
        while let Some(d) = stack.pop_back() {
            write!(
                f,
                "{}",
                match d {
                    2 => '2',
                    1 => '1',
                    0 => '0',
                    -1 => '-',
                    -2 => '=',
                    _ => panic!("invalid input"),
                }
            )?;
        }
        Ok(())
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res: i64 = 0;
        for n in iter {
            res += n.0
        }
        Snafu(res)
    }
}

fn run_program(lines: &[&str]) -> String {
    let res: Snafu = lines
        .iter()
        .map(|&line| Snafu::try_from(line).unwrap())
        .sum();
    format!("{}", res)
}

pub struct Day;

impl super::Runner for Day {
    type T = String;

    fn run(input: &str, basic: bool) -> Self::T {
        let lines: Vec<_> = input.lines().collect();
        if basic {
            run_program(&lines)
        } else {
            "No part 2 for this day".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = include_str!("ex1.txt");
    const BIG_EXAMPLE: &str = include_str!("ex2.txt");

    #[test]
    fn parses_small_numbers() {
        assert_eq!(Snafu::try_from("0"), Ok(Snafu(0)));
        assert_eq!(Snafu::try_from("1"), Ok(Snafu(1)));
        assert_eq!(Snafu::try_from("2"), Ok(Snafu(2)));
        assert_eq!(Snafu::try_from("1="), Ok(Snafu(3)));
        assert_eq!(Snafu::try_from("1-"), Ok(Snafu(4)));
        assert_eq!(Snafu::try_from("10"), Ok(Snafu(5)));
        assert_eq!(Snafu::try_from("11"), Ok(Snafu(6)));
        assert_eq!(Snafu::try_from("12"), Ok(Snafu(7)));
        assert_eq!(Snafu::try_from("2="), Ok(Snafu(8)));
        assert_eq!(Snafu::try_from("2-"), Ok(Snafu(9)));
        assert_eq!(Snafu::try_from("20"), Ok(Snafu(10)));
        assert_eq!(Snafu::try_from("1=0"), Ok(Snafu(15)));
        assert_eq!(Snafu::try_from("1-0"), Ok(Snafu(20)));
    }

    #[test]
    fn parses_big_numbers() {
        assert_eq!(Snafu::try_from("1=11-2"), Ok(Snafu(2022)));
        assert_eq!(Snafu::try_from("1-0---0"), Ok(Snafu(12345)));
        assert_eq!(Snafu::try_from("1121-1110-1=0"), Ok(Snafu(314159265)));
    }

    #[test]
    fn part_1_small() {
        let lines: Vec<_> = SMALL_EXAMPLE.split('\n').collect();
        assert_eq!(run_program(&lines), "2=-1=0");
    }

    #[test]
    fn part_1_big() {
        let lines: Vec<_> = BIG_EXAMPLE.split('\n').collect();
        assert_eq!(run_program(&lines), "2=0=02-0----2-=02-10");
    }

    // Note: there was no part 2 on this day since it was the last day
}
