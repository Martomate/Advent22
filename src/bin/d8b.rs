use std::io::{self, BufRead};

fn visible_trees(grid: &Vec<Vec<i8>>, tx: usize, ty: usize, dx: i32, dy: i32) -> u32 {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let here = grid[ty][tx];

    let mut x = tx as i32;
    let mut y = ty as i32;
    loop {
        x += dx;
        y += dy;

        if x == -1 || y == -1 || x == w || y == h {
            x -= dx;
            y -= dy;
            break;
        }

        if grid[y as usize][x as usize] >= here {
            break;
        }
    }
    // at this point (x, y) is the last tree in sight
    return (tx as i32).abs_diff(x) + (ty as i32).abs_diff(y);
}

fn scenic_score(grid: &Vec<Vec<i8>>, tx: usize, ty: usize) -> u32 {
    visible_trees(&grid, tx, ty, 1, 0)
        * visible_trees(&grid, tx, ty, -1, 0)
        * visible_trees(&grid, tx, ty, 0, 1)
        * visible_trees(&grid, tx, ty, 0, -1)
}

fn main() {
    println!("Hello, world!");

    let mut grid: Vec<Vec<i8>> = Vec::new();

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line.len() == 0 {
            break;
        }

        grid.push(line.chars().map(|c| c as i8 - '0' as i8).collect());
    }

    let h = grid.len();
    let w = grid[0].len();

    let mut max_score: u32 = 0;

    for ty in 0..h {
        for tx in 0..w {
            let score = scenic_score(&grid, tx, ty);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("{}", max_score);
}
