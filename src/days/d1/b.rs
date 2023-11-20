use std::io::{self, BufRead};

pub fn main() {
    println!("Hello, world!");

    let mut sums: Vec<i32> = Vec::new();
    let mut latest: i32 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if !l.is_empty() {
            let c = l.parse::<i32>().unwrap();
            latest += c;
        } else {
            sums.push(latest);
            latest = 0;
        }
    }
    sums.push(latest);

    sums.sort();

    let max3 = sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3];

    println!("{}", max3);
}
