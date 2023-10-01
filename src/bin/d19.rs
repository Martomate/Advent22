use std::{
    io::{self, BufRead},
    time::Instant,
};

use advent22::days::d19;
use clap::{Parser, ValueEnum};

fn main() {
    let cli = Cli::parse();

    let steps = cli.steps;
    let perform_sum = cli.mode == Mode::Sum;

    let mut lines: Vec<String> = Vec::new();

    if let Some(example) = cli.example {
        let s = if example == 2 {
            include_str!("../days/d19/ex2.txt")
        } else {
            include_str!("../days/d19/ex1.txt")
        };

        for line in s.split('\n').map(|s| s.to_owned()) {
            lines.push(line);
        }
    } else {
        let stdin = io::stdin();

        for l in stdin.lock().lines() {
            let line = l.unwrap();

            if line.is_empty() && lines.last().filter(|&s| s.is_empty()).is_some() {
                break;
            }

            lines.push(line);
        }
    }

    println!(
        "Calculating {} steps in '{}' mode...",
        steps,
        if perform_sum { "sum" } else { "product" }
    );

    let now = Instant::now();

    let res = d19::run_program(lines, steps, perform_sum);

    let elapsed_time = now.elapsed();

    println!("Time: {} seconds.", elapsed_time.as_secs_f32());
    println!("Answer: {}", res);
}

#[derive(ValueEnum, Clone, PartialEq, Eq)]
enum Mode {
    Sum,
    Product,
}

#[derive(Parser)]
struct Cli {
    #[arg(long, short, default_value = "sum")]
    mode: Mode,

    #[arg(long, short, default_value = "24")]
    steps: u8,

    #[arg(long, short)]
    example: Option<u8>,
}
