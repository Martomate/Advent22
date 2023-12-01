#[derive(PartialEq, Eq, Copy, Clone)]
enum Piece {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

pub fn main(input: &str) -> i32 {
    println!("Hello, world!");

    let mut total = 0;

    for l in input.lines() {
        let parts: Vec<&str> = l.split_ascii_whitespace().collect();
        let first = parts[0];
        let second = parts[1];

        let opponent = match first {
            "A" => Piece::Rock,
            "B" => Piece::Paper,
            "C" => Piece::Scissors,
            _ => panic!(),
        };

        let outcome = match second {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!(),
        };

        let me = match (opponent, outcome) {
            (Piece::Rock, Outcome::Win) => Piece::Paper,
            (Piece::Paper, Outcome::Win) => Piece::Scissors,
            (Piece::Scissors, Outcome::Win) => Piece::Rock,
            (Piece::Rock, Outcome::Lose) => Piece::Scissors,
            (Piece::Paper, Outcome::Lose) => Piece::Rock,
            (Piece::Scissors, Outcome::Lose) => Piece::Paper,
            (Piece::Rock, Outcome::Draw) => Piece::Rock,
            (Piece::Paper, Outcome::Draw) => Piece::Paper,
            (Piece::Scissors, Outcome::Draw) => Piece::Scissors,
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

    total
}
