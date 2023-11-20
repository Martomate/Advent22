use std::io::{self, BufRead};

pub fn main() {
    println!("Hello, world!");

    let mut total = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let first = &l[..(l.len() / 2)];
        let second = &l[(l.len() / 2)..];

        let mut fbits: [bool; 52] = [false; 52];
        let mut sbits: [bool; 52] = [false; 52];

        for c in first.chars() {
            let d = match c {
                'a'..='z' => (c as u32) - ('a' as u32),
                'A'..='Z' => (c as u32) - ('A' as u32) + 26,
                _ => panic!(),
            };
            fbits[d as usize] = true;
        }
        for c in second.chars() {
            let d = match c {
                'a'..='z' => (c as u32) - ('a' as u32),
                'A'..='Z' => (c as u32) - ('A' as u32) + 26,
                _ => panic!(),
            };
            sbits[d as usize] = true;
        }
        let mut dup_idx = 0;
        for i in 0..52 {
            if fbits[i] && sbits[i] {
                dup_idx = i;
            }
        }

        total += dup_idx + 1;
    }

    println!("{}", total);
}
