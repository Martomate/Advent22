use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    println!("Hello, world!");

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();

    let mut result = 0;

    for i in 13..(line.len() - 1) {
        let mut s: HashSet<char> = HashSet::new();
        for c in line[(i - 13)..=i].chars() {
            s.insert(c);
        }
        if s.len() == 14 {
            result = i + 1;
            break;
        }
    }

    println!("{}", result);
}
