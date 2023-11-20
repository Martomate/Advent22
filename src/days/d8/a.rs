use std::io::{self, BufRead};

pub fn main() {
    println!("Hello, world!");

    let mut grid: Vec<Vec<i8>> = Vec::new();

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line.is_empty() {
            break;
        }

        grid.push(line.chars().map(|c| c as i8 - '0' as i8).collect());
    }

    let h = grid.len();
    let w = grid[0].len();

    let mut visible: Vec<bool> = vec![false; w * h];

    for y in 0..h {
        let mut top: i8 = -1;
        for x in 0..w {
            let here = grid[y][x];
            if here > top {
                top = here;
                visible[y * w + x] = true;
            }
        }

        top = -1;
        for x in (0..w).rev() {
            let here = grid[y][x];
            if here > top {
                top = here;
                visible[y * w + x] = true;
            }
        }
    }

    for x in 0..w {
        let mut top: i8 = -1;
        for y in 0..h {
            let here = grid[y][x];
            if here > top {
                top = here;
                visible[y * w + x] = true;
            }
        }

        top = -1;
        for y in (0..h).rev() {
            let here = grid[y][x];
            if here > top {
                top = here;
                visible[y * w + x] = true;
            }
        }
    }

    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            if visible[y * w + x] {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
