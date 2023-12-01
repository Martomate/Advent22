use std::collections::HashSet;

pub fn main(input: &str) -> usize {
    println!("Hello, world!");

    let line = input.lines().next().unwrap();

    let mut result = 0;

    for i in 3..(line.len() - 1) {
        let mut s: HashSet<char> = HashSet::new();
        for c in line[(i - 3)..=i].chars() {
            s.insert(c);
        }
        if s.len() == 4 {
            result = i + 1;
            break;
        }
    }

    result
}
