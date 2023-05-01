#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Move(u32),
    TurnLeft,
    TurnRight,
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut steps: u32 = 0;

    for c in s.chars() {
        if c == 'L' {
            instructions.push(Instruction::Move(steps));
            instructions.push(Instruction::TurnLeft);
            steps = 0;
        } else if c == 'R' {
            instructions.push(Instruction::Move(steps));
            instructions.push(Instruction::TurnRight);
            steps = 0;
        } else {
            steps = steps * 10 + ((c as u8 - b'0') as u32);
        }
    }

    instructions.push(Instruction::Move(steps));

    instructions
        .into_iter()
        .filter(|i| match *i {
            Instruction::Move(n) => n != 0,
            _ => true,
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Up,
    Left,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }

    fn turn_cw(self) -> Direction {
        match self {
            Direction::Down => Direction::Left,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_ccw(self) -> Direction {
        match self {
            Direction::Down => Direction::Right,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Position {
    fn new(x: i32, y: i32, dir: Direction) -> Position {
        Position { x, y, dir }
    }

    fn after_move(&self) -> Position {
        match self.dir {
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
                dir: self.dir,
            },
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
                dir: self.dir,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
                dir: self.dir,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
                dir: self.dir,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpaceContent {
    Empty,
    Ground,
    Stone,
}

struct Board {
    rows: Vec<Vec<SpaceContent>>,
}

impl Board {
    fn lookup(&self, x: i32, y: i32) -> SpaceContent {
        let is_off_the_board = y < 0
            || x < 0
            || y >= self.rows.len() as i32
            || x >= self.rows[y as usize].len() as i32;
        if is_off_the_board {
            SpaceContent::Empty
        } else {
            self.rows[y as usize][x as usize]
        }
    }

    fn walk(&self, start: Position, steps: Vec<Instruction>) -> Position {
        let mut here = start;
        println!("{:?}", here);
        for step in steps {
            here = self.perform(here, step);
            println!("{:?}", here);
        }
        here
    }

    fn perform(&self, start: Position, step: Instruction) -> Position {
        let mut here = start;
        match step {
            Instruction::Move(d) => {
                for _ in 0..d {
                    let next = here.after_move();
                    let content = self.lookup(next.x, next.y);
                    if content == SpaceContent::Ground {
                        here = next;
                    } else if content == SpaceContent::Empty {
                        let mut p = Position {
                            dir: here.dir.opposite(),
                            ..here
                        };
                        loop {
                            let n = p.after_move();
                            if self.lookup(n.x, n.y) == SpaceContent::Empty {
                                break;
                            }
                            p = n;
                        }
                        if self.rows[p.y as usize][p.x as usize] == SpaceContent::Ground {
                            here = Position { dir: here.dir, ..p };
                        }
                    }
                }
            }
            Instruction::TurnLeft => {
                here = Position {
                    dir: here.dir.turn_ccw(),
                    ..here
                }
            }
            Instruction::TurnRight => {
                here = Position {
                    dir: here.dir.turn_cw(),
                    ..here
                }
            }
        };
        here
    }
}

impl From<Vec<&str>> for Board {
    fn from(lines: Vec<&str>) -> Self {
        Board {
            rows: lines
                .into_iter()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '.' => SpaceContent::Ground,
                            '#' => SpaceContent::Stone,
                            _ => SpaceContent::Empty,
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

fn score(pos: Position) -> u32 {
    let dir_num = match pos.dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };
    (pos.y + 1) as u32 * 1000 + (pos.x + 1) as u32 * 4 + dir_num
}

fn parse_input(input: Vec<&str>) -> (Board, Vec<Instruction>) {
    let mut lines: Vec<&str> = Vec::new();
    let mut board_done = false;
    let mut steps: Option<&str> = None;

    for line in input.iter() {
        if line.is_empty() {
            board_done = true;
        } else if !board_done {
            lines.push(line);
        } else {
            steps = Some(line);
        }
    }

    let board = Board::from(lines);
    let instructions = parse_instructions(steps.unwrap());

    (board, instructions)
}

fn run_program(input: Vec<&str>) -> u32 {
    let start_x = input[0].find(|c| c == '.').unwrap();
    println!("{}", input[0]);
    let (board, instructions) = parse_input(input);

    let start_pos = Position::new(start_x as i32, 0, Direction::Right);
    let end_pos = board.walk(start_pos, instructions);

    score(end_pos)
}

fn main() {
    let input: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap()).collect();
    let input = input.iter().map(|s| s.as_str()).collect();

    let result = run_program(input);

    println!("Score: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instructions_handles_one_number() {
        assert_eq!(parse_instructions("123"), vec![Instruction::Move(123)]);
    }

    #[test]
    fn parse_instructions_handles_one_turn() {
        assert_eq!(parse_instructions("L"), vec![Instruction::TurnLeft]);
        assert_eq!(parse_instructions("R"), vec![Instruction::TurnRight]);
    }

    #[test]
    fn parse_instructions_handles_one_number_and_one_turn() {
        assert_eq!(
            parse_instructions("123L"),
            vec![Instruction::Move(123), Instruction::TurnLeft]
        );
        assert_eq!(
            parse_instructions("L123"),
            vec![Instruction::TurnLeft, Instruction::Move(123)]
        );
    }

    #[test]
    fn parse_instructions_handles_simple_example() {
        use Instruction::*;

        assert_eq!(
            parse_instructions("10R5L5R10L4R5L5"),
            vec![
                Move(10),
                TurnRight,
                Move(5),
                TurnLeft,
                Move(5),
                TurnRight,
                Move(10),
                TurnLeft,
                Move(4),
                TurnRight,
                Move(5),
                TurnLeft,
                Move(5)
            ]
        );
    }

    #[test]
    fn parse_board_handles_small_rectangle() {
        use SpaceContent::*;

        let board = Board::from(vec!["..", "#."]);
        assert_eq!(board.rows, vec![vec![Ground, Ground], vec![Stone, Ground]])
    }

    #[test]
    fn parse_board_handles_small_rectangle_with_space() {
        use SpaceContent::*;

        let board = Board::from(vec![" ..", " #."]);
        assert_eq!(
            board.rows,
            vec![vec![Empty, Ground, Ground], vec![Empty, Stone, Ground]]
        )
    }

    #[test]
    fn board_walk_handles_a_single_move_on_ground() {
        let board = Board::from(vec!["...", "...", "..."]);
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Right), Instruction::Move(1)),
            Position::new(2, 1, Direction::Right)
        );
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Left), Instruction::Move(1)),
            Position::new(0, 1, Direction::Left)
        );
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Down), Instruction::Move(1)),
            Position::new(1, 2, Direction::Down)
        );
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Up), Instruction::Move(1)),
            Position::new(1, 0, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_multiple_moves_on_ground() {
        let board = Board::from(vec![".....", ".....", ".....", ".....", "....."]);
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Right), Instruction::Move(2)),
            Position::new(4, 2, Direction::Right)
        );
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Left), Instruction::Move(2)),
            Position::new(0, 2, Direction::Left)
        );
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Down), Instruction::Move(2)),
            Position::new(2, 4, Direction::Down)
        );
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Up), Instruction::Move(2)),
            Position::new(2, 0, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_a_single_move_into_stone() {
        let board = Board::from(vec![".#.", "#.#", ".#."]);
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Right), Instruction::Move(1)),
            Position::new(1, 1, Direction::Right)
        );
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Left), Instruction::Move(1)),
            Position::new(1, 1, Direction::Left)
        );
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Down), Instruction::Move(1)),
            Position::new(1, 1, Direction::Down)
        );
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Up), Instruction::Move(1)),
            Position::new(1, 1, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_a_single_move_to_the_other_side_with_empty_space_around() {
        let board = Board::from(vec!["", " .. ", " .. ", ""]);
        assert_eq!(
            board.perform(Position::new(2, 1, Direction::Right), Instruction::Move(1)),
            Position::new(1, 1, Direction::Right)
        );
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Left), Instruction::Move(1)),
            Position::new(2, 1, Direction::Left)
        );
        assert_eq!(
            board.perform(Position::new(1, 2, Direction::Down), Instruction::Move(1)),
            Position::new(1, 1, Direction::Down)
        );
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Up), Instruction::Move(1)),
            Position::new(1, 2, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_a_single_move_to_the_other_side_without_empty_space_around() {
        let board = Board::from(vec!["..", ".."]);
        assert_eq!(
            board.perform(Position::new(1, 0, Direction::Right), Instruction::Move(1)),
            Position::new(0, 0, Direction::Right)
        );
        assert_eq!(
            board.perform(Position::new(0, 0, Direction::Left), Instruction::Move(1)),
            Position::new(1, 0, Direction::Left)
        );
        assert_eq!(
            board.perform(Position::new(0, 1, Direction::Down), Instruction::Move(1)),
            Position::new(0, 0, Direction::Down)
        );
        assert_eq!(
            board.perform(Position::new(0, 0, Direction::Up), Instruction::Move(1)),
            Position::new(0, 1, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_a_single_move_to_the_other_side_into_stone() {
        let board = Board::from(vec!["", " .# ", " #. ", ""]);
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Right), Instruction::Move(1)),
            Position::new(2, 2, Direction::Right)
        );
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Left), Instruction::Move(1)),
            Position::new(1, 1, Direction::Left)
        );
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Down), Instruction::Move(1)),
            Position::new(2, 2, Direction::Down)
        );
        assert_eq!(
            board.perform(Position::new(1, 1, Direction::Up), Instruction::Move(1)),
            Position::new(1, 1, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_left_turn() {
        let board = Board::from(vec![".....", ".....", ".....", ".....", "....."]);
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Right), Instruction::TurnLeft),
            Position::new(2, 2, Direction::Up)
        );
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Left), Instruction::TurnLeft),
            Position::new(2, 2, Direction::Down)
        );
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Down), Instruction::TurnLeft),
            Position::new(2, 2, Direction::Right)
        );
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Up), Instruction::TurnLeft),
            Position::new(2, 2, Direction::Left)
        );
    }

    #[test]
    fn score_uses_1_indexing() {
        assert_eq!(score(Position::new(0, 0, Direction::Right)), 1004);
        assert_eq!(score(Position::new(0, 0, Direction::Down)), 1005);
        assert_eq!(score(Position::new(0, 0, Direction::Left)), 1006);
        assert_eq!(score(Position::new(0, 0, Direction::Up)), 1007);
        
        assert_eq!(score(Position::new(1, 0, Direction::Right)), 1008);
        assert_eq!(score(Position::new(0, 1, Direction::Right)), 2004);
    }

    #[test]
    fn board_walk_handles_right_turn() {
        let board = Board::from(vec![".....", ".....", ".....", ".....", "....."]);
        assert_eq!(
            board.perform(
                Position::new(2, 2, Direction::Right),
                Instruction::TurnRight
            ),
            Position::new(2, 2, Direction::Down)
        );
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Left), Instruction::TurnRight),
            Position::new(2, 2, Direction::Up)
        );
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Down), Instruction::TurnRight),
            Position::new(2, 2, Direction::Left)
        );
        assert_eq!(
            board.perform(Position::new(2, 2, Direction::Up), Instruction::TurnRight),
            Position::new(2, 2, Direction::Right)
        );
    }

    #[test]
    fn small_example_should_work() {
        assert_eq!(run_program(include_str!("d22_ex_1.txt").split('\n').collect()), 6032);
    }

    #[test]
    fn big_example_should_work() {
        assert_eq!(run_program(include_str!("d22_ex_2.txt").split('\n').collect()), 117102);
    }
}
