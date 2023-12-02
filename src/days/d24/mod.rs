use std::{collections::HashSet, fmt::Display};

use queues::{IsQueue, Queue};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Dir {
    fn dx(self) -> i32 {
        match self {
            Dir::Left => -1,
            Dir::Right => 1,
            _ => 0,
        }
    }

    fn dy(self) -> i32 {
        match self {
            Dir::Up => -1,
            Dir::Down => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Wind {
    x: u32,
    y: u32,
    dir: Dir,
}

impl Wind {
    fn moved_forward(self) -> Wind {
        let Wind { x, y, dir } = self;
        Wind {
            x: (x as i32 + dir.dx()) as u32,
            y: (y as i32 + dir.dy()) as u32,
            dir,
        }
    }

    fn wrapped(self, width: u32, height: u32) -> Wind {
        let mut w = self;
        if w.x == width - 1 {
            w.x = 1;
        } else if w.x == 0 {
            w.x = width - 2;
        }
        if w.y == height - 1 {
            w.y = 1;
        } else if w.y == 0 {
            w.y = height - 2;
        }
        w
    }
}

#[derive(Clone)]
struct Board {
    width: u32,
    height: u32,
    winds: Vec<Wind>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if y == 0 {
                    if x == 1 {
                        '.'
                    } else {
                        '#'
                    }
                } else if y == self.height - 1 {
                    if x == self.width - 2 {
                        '.'
                    } else {
                        '#'
                    }
                } else if x == 0 || x == self.width - 1 {
                    '#'
                } else {
                    let w: Vec<_> = self.winds.iter().filter(|w| w.x == x && w.y == y).collect();
                    let count = w.len();
                    if count == 0 {
                        '.'
                    } else if count == 1 {
                        match w[0].dir {
                            Dir::Up => '^',
                            Dir::Left => '<',
                            Dir::Down => 'v',
                            Dir::Right => '>',
                        }
                    } else if count < 10 {
                        (b'0' + count as u8) as char
                    } else {
                        'X'
                    }
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    fn with_sorted_wind(self) -> Self {
        let mut w = self.winds;
        w.sort();
        Board {
            width: self.width,
            height: self.height,
            winds: w,
        }
    }

    fn move_wind(&self) -> Board {
        Board {
            winds: self
                .winds
                .iter()
                .map(|w| w.moved_forward().wrapped(self.width, self.height))
                .collect(),
            ..*self
        }
    }

    fn is_windy(&self, x: u32, y: u32) -> bool {
        self.winds.iter().any(|w| w.x == x && w.y == y)
    }

    fn can_move(&self, x: u32, y: u32, dir: Option<Dir>) -> bool {
        let (x, y) = if let Some(dir) = dir {
            (x as i32 + dir.dx(), y as i32 + dir.dy())
        } else {
            (x as i32, y as i32)
        };

        let is_inside = x > 0 && x < self.width as i32 - 1 && y > 0 && y < self.height as i32 - 1;
        let is_on_ground = is_inside
            || (x == 1 && y == 0)
            || (x == self.width as i32 - 2 && y == self.height as i32 - 1);
        is_on_ground && !self.is_windy(x as u32, y as u32)
    }
}

fn parse_input(lines: &[&str]) -> Board {
    let height = lines.len();
    let width = lines[0].len();
    let mut winds = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let dir = match c {
                '<' => Some(Dir::Left),
                '>' => Some(Dir::Right),
                '^' => Some(Dir::Up),
                'v' => Some(Dir::Down),
                _ => None,
            };

            if let Some(dir) = dir {
                winds.push(Wind {
                    x: x as u32,
                    y: y as u32,
                    dir,
                });
            }
        }
    }

    Board {
        width: width as u32,
        height: height as u32,
        winds,
    }
}

#[derive(Clone)]
struct BfsState {
    board: Board,
    x: u32,
    y: u32,
    steps: u32,
}

#[derive(PartialEq, Eq, Hash)]
struct BfsKey {
    winds: Vec<Wind>,
    x: u32,
    y: u32,
}

impl BfsState {
    fn new(board: Board, x: u32, y: u32, steps: u32) -> Self {
        BfsState { board, x, y, steps }
    }
}

fn calc(board: Board, start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> (Board, u32) {
    let mut seen = HashSet::<BfsKey>::new();

    let mut q: Queue<BfsState> = Queue::new();
    q.add(BfsState::new(board, start_x, start_y, 0)).unwrap();
    loop {
        let s = q.remove().unwrap();

        if s.y == end_y && s.x == end_x {
            return (s.board.clone(), s.steps);
        }

        let key = BfsKey {
            winds: s.board.winds.clone(),
            x: s.x,
            y: s.y,
        };
        if !seen.insert(key) {
            continue;
        }

        let b = s.board.move_wind().with_sorted_wind(); // sort to make 'seen' more efficient
        if b.can_move(s.x, s.y, Some(Dir::Up)) {
            q.add(BfsState::new(b.clone(), s.x, s.y - 1, s.steps + 1))
                .unwrap();
        }
        if b.can_move(s.x, s.y, Some(Dir::Down)) {
            q.add(BfsState::new(b.clone(), s.x, s.y + 1, s.steps + 1))
                .unwrap();
        }
        if b.can_move(s.x, s.y, Some(Dir::Left)) {
            q.add(BfsState::new(b.clone(), s.x - 1, s.y, s.steps + 1))
                .unwrap();
        }
        if b.can_move(s.x, s.y, Some(Dir::Right)) {
            q.add(BfsState::new(b.clone(), s.x + 1, s.y, s.steps + 1))
                .unwrap();
        }
        if b.can_move(s.x, s.y, None) {
            q.add(BfsState::new(b.clone(), s.x, s.y, s.steps + 1))
                .unwrap();
        }
    }
}

fn run_program(lines: &[&str], part1: bool) -> u32 {
    let board = parse_input(lines);

    let start_x = 1;
    let start_y = 0;
    let end_x = lines[board.height as usize - 1].find('.').unwrap() as u32;
    let end_y = board.height - 1;

    let mut total = 0;

    let (board, steps) = calc(board, start_x, start_y, end_x, end_y);
    total += steps;

    if !part1 {
        let (board, steps) = calc(board, end_x, end_y, start_x, start_y);
        total += steps;

        let (_, steps) = calc(board, start_x, start_y, end_x, end_y);
        total += steps;
    }

    total
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
    fn part_1_small() {
        let lines: Vec<_> = SMALL_EXAMPLE.split('\n').collect();
        assert_eq!(run_program(&lines, true), 18);
    }

    #[test]
    fn part_1_big() {
        let lines: Vec<_> = BIG_EXAMPLE.split('\n').collect();
        assert_eq!(run_program(&lines, true), 225);
    }

    #[test]
    fn part_2_small() {
        let lines: Vec<_> = SMALL_EXAMPLE.split('\n').collect();
        assert_eq!(run_program(&lines, false), 54);
    }

    #[test]
    fn part_2_big() {
        let lines: Vec<_> = BIG_EXAMPLE.split('\n').collect();
        assert_eq!(run_program(&lines, false), 711);
    }
}
