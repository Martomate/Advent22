use advent22::days;
use clap::{Parser, ValueEnum};

fn main() {
    let cli = Cli::parse();

    let res = days::d20::run_example(cli.example == Example::Big, cli.part == Part::Advanced);

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
