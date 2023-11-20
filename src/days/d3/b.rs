use std::io::{self, BufRead};

fn occurances(s: &str) -> [bool; 52] {
    let mut fbits: [bool; 52] = [false; 52];
    for c in s.chars() {
        let d = match c {
            'a'..='z' => (c as u32) - ('a' as u32),
            'A'..='Z' => (c as u32) - ('A' as u32) + 26,
            _ => panic!(),
        };
        fbits[d as usize] = true;
    }
    fbits
}

fn find_same_in_three(a: [bool; 52], b: [bool; 52], c: [bool; 52]) -> usize {
    let mut dup_idx = 0;
    for i in 0..52 {
        if a[i] && b[i] && c[i] {
            dup_idx = i;
            break;
        }
    }
    dup_idx + 1
}

pub fn main() {
    println!("Hello, world!");

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let s: usize = lines
        .chunks(3)
        .map(|ls| (occurances(&ls[0]), occurances(&ls[1]), occurances(&ls[2])))
        .map(|(a, b, c)| find_same_in_three(a, b, c))
        .sum();

    println!("{}", s);
}
