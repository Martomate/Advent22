use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Piece {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

pub fn main() {
    println!("Hello, world!");

    let mut total = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let parts: Vec<&str> = l.split_ascii_whitespace().collect();
        let first = parts[0];
        let second = parts[1];

        let opponent = match first {
            "A" => Piece::Rock,
            "B" => Piece::Paper,
            "C" => Piece::Scissors,
            _ => panic!(),
        };

        let me = match second {
            "X" => Piece::Rock,
            "Y" => Piece::Paper,
            "Z" => Piece::Scissors,
            _ => panic!(),
        };

        let outcome = match (opponent, me) {
            (Piece::Rock, Piece::Rock) => Outcome::Draw,
            (Piece::Rock, Piece::Paper) => Outcome::Win,
            (Piece::Rock, Piece::Scissors) => Outcome::Lose,
            (Piece::Paper, Piece::Rock) => Outcome::Lose,
            (Piece::Paper, Piece::Paper) => Outcome::Draw,
            (Piece::Paper, Piece::Scissors) => Outcome::Win,
            (Piece::Scissors, Piece::Rock) => Outcome::Win,
            (Piece::Scissors, Piece::Paper) => Outcome::Lose,
            (Piece::Scissors, Piece::Scissors) => Outcome::Draw,
        };

        total += match me {
            Piece::Rock => 1,
            Piece::Paper => 2,
            Piece::Scissors => 3,
        };

        total += match outcome {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };
    }

    println!("{}", total);
}
