use std::{
    collections::HashSet,
    fmt::Display,
    io::{self, BufRead},
};

struct Cave {
    stones: HashSet<(i32, i32)>,
    sand: HashSet<(i32, i32)>,
}

impl Cave {
    fn new() -> Self {
        Cave {
            stones: HashSet::new(),
            sand: HashSet::new(),
        }
    }

    fn x_lo(&self) -> Option<i32> {
        self.stones.iter().min_by_key(|p| p.0).map(|r| r.0)
    }

    fn x_hi(&self) -> Option<i32> {
        self.stones.iter().max_by_key(|p| p.0).map(|r| r.0)
    }

    fn y_lo(&self) -> Option<i32> {
        self.stones.iter().min_by_key(|p| p.1).map(|r| r.1)
    }

    fn y_hi(&self) -> Option<i32> {
        self.stones.iter().max_by_key(|p| p.1).map(|r| r.1)
    }

    /// Simulates a piece of sand falling from (start_x, start_y).
    /// Returns Some((x, y)) if the sand stopped at (x, y), and None if the sand fell into the void.
    fn simulate_one_step(&self, start_x: i32, start_y: i32, floor: i32) -> Option<(i32, i32)> {
        let mut x = start_x;
        let mut y = start_y;

        while y < floor {
            if !self.stones.contains(&(x, y + 1)) && !self.sand.contains(&(x, y + 1)) {
                y += 1;
            } else if !self.stones.contains(&(x - 1, y + 1)) && !self.sand.contains(&(x - 1, y + 1))
            {
                x -= 1;
                y += 1;
            } else if !self.stones.contains(&(x + 1, y + 1)) && !self.sand.contains(&(x + 1, y + 1))
            {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        if y < floor {
            Some((x, y))
        } else {
            None
        }
    }

    fn simulate(&mut self, start_x: i32, start_y: i32) {
        let floor = self.y_hi().unwrap();
        while let Some((x, y)) = self.simulate_one_step(start_x, start_y, floor) {
            self.sand.insert((x, y));
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.stones.is_empty() {
            return std::fmt::Result::Ok(());
        }

        let x_lo = self.x_lo().unwrap();
        let x_hi = self.x_hi().unwrap();
        let y_lo = self.y_lo().unwrap();
        let y_hi = self.y_hi().unwrap();

        let mut lines: Vec<String> = Vec::new();
        for y in y_lo..=y_hi {
            let s = (x_lo..=x_hi)
                .map(|x| {
                    if self.stones.contains(&(x, y)) {
                        '#'
                    } else if self.sand.contains(&(x, y)) {
                        'o'
                    } else {
                        '.'
                    }
                })
                .collect::<String>();
            lines.push(s);
        }
        write!(f, "{}", lines.join("\n"))
    }
}

pub fn main() {
    let mut cave = Cave::new();

    for l in io::stdin().lock().lines() {
        let line = l.unwrap();

        if line.is_empty() {
            break;
        }

        let pts: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|part| {
                let (ls, rs) = part.split_once(',').unwrap();
                (ls.parse::<i32>().unwrap(), rs.parse::<i32>().unwrap())
            })
            .collect();

        for i in 1..pts.len() {
            let prev = pts[i - 1];
            let here = pts[i];

            let dx = (here.0 - prev.0).signum();
            let dy = (here.1 - prev.1).signum();

            let mut x = prev.0;
            let mut y = prev.1;

            loop {
                cave.stones.insert((x, y));

                if x == here.0 && y == here.1 {
                    break;
                }

                x += dx;
                y += dy;
            }
        }
    }

    println!("{}", cave);
    println!();

    cave.simulate(500, 0);
    let sand_count = cave.sand.len();

    println!();
    println!("{}", cave);

    println!();
    println!("Sand: {}", sand_count);
}
