use std::{
    collections::HashSet,
    io::{self, BufRead},
};

#[derive(Clone, Copy)]
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

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

fn move_head(head: &mut Pos, dir: Dir) {
    match dir {
        Dir::Up => head.y -= 1,
        Dir::Down => head.y += 1,
        Dir::Left => head.x -= 1,
        Dir::Right => head.x += 1,
    };
}

fn new_tail_pos(head: &Pos, tail: Pos) -> Pos {
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

        Pos {
            x: tail.x + dx,
            y: tail.y + dy,
        }
    } else {
        tail
    }
}

pub fn main() {
    println!("Hello, world!");

    let mut motions: Vec<Motion> = Vec::new();

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
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

    let mut knots: [Pos; 10] = [Pos { x: 0, y: 0 }; 10];

    let mut tail_history: Vec<Pos> = Vec::new();
    tail_history.push(knots[knots.len() - 1]);

    for motion in motions {
        for _ in 0..motion.steps {
            move_head(&mut knots[0], motion.dir);

            for i in 1..knots.len() {
                knots[i] = new_tail_pos(&knots[i - 1], knots[i]);
            }

            tail_history.push(knots[knots.len() - 1]);
        }
    }

    let mut unique_tail_positions: HashSet<Pos> = HashSet::new();

    for pos in tail_history {
        unique_tail_positions.insert(pos);
    }

    let result = unique_tail_positions.len();

    println!("{}", result);
}
