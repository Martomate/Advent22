use std::{collections::HashMap, fmt::Display, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
    elfs: Vec<Point>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.bounds();
        for y in b.top..=b.bottom {
            for x in b.left..=b.right {
                if self.elfs.contains(&Point::new(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<Vec<Point>> for Board {
    fn from(elfs: Vec<Point>) -> Self {
        Board { elfs }
    }
}

fn neighbors(p: Point) -> Vec<Point> {
    let mut pts = Vec::with_capacity(8);
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx != 0 || dy != 0 {
                pts.push(Point::new(p.x + dx, p.y + dy));
            }
        }
    }
    pts
}

fn dirs_to_check(time: u32, retry: u32) -> [Point; 3] {
    let i = (time + retry) % 4;
    match i {
        0 => [Point::new(1, -1), Point::new(0, -1), Point::new(-1, -1)],
        1 => [Point::new(-1, 1), Point::new(0, 1), Point::new(1, 1)],
        2 => [Point::new(-1, 1), Point::new(-1, 0), Point::new(-1, -1)],
        3 => [Point::new(1, -1), Point::new(1, 0), Point::new(1, 1)],
        _ => unreachable!(),
    }
}

struct Bounds {
    top: i32,
    left: i32,
    bottom: i32,
    right: i32,
}

impl Board {
    fn simulate_round(&self, time: u32) -> Board {
        let mut proposed_moves: Vec<(Point, Point)> = Vec::new();
        let mut next_try = Vec::new();
        for &elf in self.elfs.iter() {
            if neighbors(elf).iter().any(|n| self.elfs.contains(n)) {
                next_try.push(elf);
            } else {
                proposed_moves.push((elf, elf));
            }
        }
        for retry in 0..4 {
            let dirs = dirs_to_check(time, retry);

            let elfs = next_try;
            next_try = Vec::new();

            for e in elfs {
                let mut taken = false;
                for d in dirs {
                    if self.elfs.contains(&(e + d)) {
                        taken = true;
                        break;
                    }
                }
                if !taken {
                    proposed_moves.push((e, e + dirs[1]));
                } else {
                    next_try.push(e);
                }
            }
        }
        for elf in next_try {
            proposed_moves.push((elf, elf));
        }

        let mut counts = HashMap::<Point, u32>::new();
        for (_, p) in &proposed_moves {
            counts.insert(*p, counts.get(p).unwrap_or(&0) + 1);
        }

        let mut moved_elfs = Vec::new();
        for &(elf, dest) in &proposed_moves {
            if counts[&dest] > 1 {
                moved_elfs.push(elf);
            } else {
                moved_elfs.push(dest);
            }
        }

        Board { elfs: moved_elfs }
    }

    fn bounds(&self) -> Bounds {
        let min_x = self.elfs.iter().map(|e| e.x).min().unwrap();
        let min_y = self.elfs.iter().map(|e| e.y).min().unwrap();
        let max_x = self.elfs.iter().map(|e| e.x).max().unwrap();
        let max_y = self.elfs.iter().map(|e| e.y).max().unwrap();

        Bounds {
            top: min_y,
            left: min_x,
            bottom: max_y,
            right: max_x,
        }
    }

    fn calc_result(&self) -> u32 {
        if self.elfs.is_empty() {
            return 0;
        }

        let b = self.bounds();

        ((b.bottom - b.top + 1) * (b.right - b.left + 1)) as u32 - self.elfs.len() as u32
    }
}

fn parse_input(lines: &[&str]) -> Vec<Point> {
    let mut elfs = Vec::new();
    for (y, &line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elfs.push(Point::new(x as i32, y as i32));
            }
        }
    }
    elfs
}

pub fn run_program(lines: &[&str], part1: bool) -> u32 {
    let elfs = parse_input(lines);
    let mut board = Board::from(elfs);
    if part1 {
        for t in 0..10 {
            board = board.simulate_round(t);
        }
        board.calc_result()
    } else {
        let mut board_before = board.clone();
        let mut t = 0;
        loop {
            board = board_before.simulate_round(t);
            if board == board_before {
                break;
            }
            board_before = board;
            t += 1;
        }
        t+1
    }
}

pub struct Day;

impl super::Runner for Day {
    type T = u32;
    
    fn run(input: &str, basic: bool) -> Self::T {
        let lines: Vec<_> = input.lines().collect();
        run_program(&lines, basic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = include_str!("ex1.txt");
    const BIG_EXAMPLE: &str = include_str!("ex2.txt");

    #[test]
    fn parse_input_works() {
        let elfs = parse_input(&["..#", ".#."]);
        assert_eq!(elfs, vec![Point::new(2, 0), Point::new(1, 1)]);
    }

    #[test]
    fn score_handles_2x2() {
        let elfs = vec![Point::new(3, 4), Point::new(4, 5)];
        assert_eq!(Board::from(elfs).calc_result(), 4 - 2);
    }

    #[test]
    fn score_handles_3x3() {
        let elfs = vec![Point::new(2, 2), Point::new(0, 0)];
        assert_eq!(Board::from(elfs).calc_result(), 9 - 2);
    }

    #[test]
    fn score_handles_negative_coordinates() {
        let elfs = vec![Point::new(-2, -2), Point::new(0, 0)];
        assert_eq!(Board::from(elfs).calc_result(), 9 - 2);
    }

    #[test]
    fn small_example_works_part_1() {
        let lines: Vec<_> = SMALL_EXAMPLE.split('\n').collect();
        assert_eq!(run_program(&lines, true), 110);
    }

    #[test]
    fn big_example_works_part_1() {
        let lines: Vec<_> = BIG_EXAMPLE.split('\n').collect();
        assert_eq!(run_program(&lines, true), 3788);
    }

    #[test]
    fn small_example_works_part_2() {
        let lines: Vec<_> = SMALL_EXAMPLE.split('\n').collect();
        assert_eq!(run_program(&lines, false), 20);
    }

    #[test]
    fn big_example_works_part_2() {
        let lines: Vec<_> = BIG_EXAMPLE.split('\n').collect();
        assert_eq!(run_program(&lines, false), 921);
    }
}
