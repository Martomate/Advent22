use std::io::{self, BufRead};

fn main() {
    println!("Hello, world!");

    let mut latest: i32 = 0;
    let mut max: i32 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if !l.is_empty() {
            let c = l.parse::<i32>().unwrap();
            latest += c;
        } else {
            if latest > max {
                max = latest;
            }
            latest = 0;
        }
    }
    if latest > max {
        max = latest;
    }

    println!("{}", max);
}
