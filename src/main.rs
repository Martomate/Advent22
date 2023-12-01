use std::fs;

use advent22::days;
use clap::{Parser, ValueEnum};

use days::*;

fn read_example(day: u8, big: bool) -> String {
    let path = if big {
        format!("src/days/d{day}/ex2.txt")
    } else {
        format!("src/days/d{day}/ex1.txt")
    };
    fs::read_to_string(path).unwrap()
}

struct Context {
    big: bool,
    basic: bool,
    day: u8,
}

impl Context {
    fn run<R: Runner>(&self, _: R) {
        let input = read_example(self.day, self.big);
        let result = R::run(&input, self.basic);
        println!("{}", result)
    }
}

fn main() {
    let cli = Cli::parse();

    let c = Context {
        big: cli.example == Example::Big,
        basic: cli.part == Part::Basic,
        day: cli.day,
    };

    match cli.day {
        1 => c.run(d1::Day),
        2 => c.run(d2::Day),
        3 => c.run(d3::Day),
        4 => c.run(d4::Day),
        5 => c.run(d5::Day),
        6 => c.run(d6::Day),
        7 => c.run(d7::Day),
        8 => c.run(d8::Day),
        9 => c.run(d9::Day),
        10 => c.run(d10::Day),
        11 => c.run(d11::Day),
        12 => c.run(d12::Day),
        13 => c.run(d13::Day),
        14 => c.run(d14::Day),
        15 => c.run(d15::Day),
        16 => c.run(d16::Day),
        17 => c.run(d17::Day),
        18 => c.run(d18::Day),
        19 => c.run(d19::Day),
        20 => c.run(d20::Day),
        21 => c.run(d21::Day),
        22 => c.run(d22::Day),
        23 => c.run(d23::Day),
        24..=25 => unimplemented!(),
        _ => println!("Not a valid day")
    };
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
    #[arg()]
    day: u8,

    #[arg(value_enum)]
    example: Example,

    #[arg(value_enum)]
    part: Part,
}
