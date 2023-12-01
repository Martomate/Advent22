use std::collections::HashSet;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Motion {
    dir: Dir,
    steps: u32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

pub fn main(input: &str) -> usize {
    println!("Hello, world!");

    let mut motions: Vec<Motion> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        if let Some((d, s)) = line.split_once(' ') {
            let dir = match d {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => panic!("Unknown direction"),
            };
            motions.push(Motion {
                dir,
                steps: s.parse::<u32>().unwrap(),
            });
        }
    }

    let mut tail_history: Vec<Pos> = Vec::new();

    let mut head: Pos = Pos { x: 0, y: 0 };
    let mut tail: Pos = Pos { x: 0, y: 0 };

    tail_history.push(tail.clone());

    for motion in motions {
        for _ in 0..motion.steps {
            match motion.dir {
                Dir::Up => head.y -= 1,
                Dir::Down => head.y += 1,
                Dir::Left => head.x -= 1,
                Dir::Right => head.x += 1,
            };

            let mut dx = head.x - tail.x;
            let mut dy = head.y - tail.y;

            if dx.abs() > 1 || dy.abs() > 1 {
                if dx < 0 {
                    dx = -1;
                } else if dx >= 1 {
                    dx = 1;
                } else {
                    dx = 0;
                }

                if dy <= -1 {
                    dy = -1;
                } else if dy >= 1 {
                    dy = 1;
                } else {
                    dy = 0;
                }

                tail.x += dx;
                tail.y += dy;

                tail_history.push(tail.clone());
            }
        }
    }

    let mut unique_tail_positions: HashSet<Pos> = HashSet::new();

    for pos in tail_history {
        unique_tail_positions.insert(pos);
    }

    unique_tail_positions.len()
}
