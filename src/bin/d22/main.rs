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
        use Direction::*;

        match self {
            Down => Up,
            Up => Down,
            Right => Left,
            Left => Right,
        }
    }

    fn turn_cw(self) -> Direction {
        use Direction::*;

        match self {
            Down => Left,
            Up => Right,
            Right => Down,
            Left => Up,
        }
    }

    fn turn_ccw(self) -> Direction {
        use Direction::*;

        match self {
            Down => Right,
            Up => Left,
            Right => Up,
            Left => Down,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Player {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Player {
    fn new(x: i32, y: i32, dir: Direction) -> Player {
        Player { x, y, dir }
    }

    fn after_move(&self) -> Player {
        match self.dir {
            Direction::Down => Player {
                y: self.y + 1,
                ..*self
            },
            Direction::Up => Player {
                y: self.y - 1,
                ..*self
            },
            Direction::Right => Player {
                x: self.x + 1,
                ..*self
            },
            Direction::Left => Player {
                x: self.x - 1,
                ..*self
            },
        }
    }

    fn after_cw_turn(&self) -> Self {
        Player {
            dir: self.dir.turn_cw(),
            ..*self
        }
    }

    fn after_ccw_turn(&self) -> Self {
        Player {
            dir: self.dir.turn_ccw(),
            ..*self
        }
    }

    fn after_u_turn(&self) -> Self {
        Player {
            dir: self.dir.opposite(),
            ..*self
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

    fn walk(&self, start: Player, steps: Vec<Instruction>) -> Player {
        let mut here = start;
        for step in steps {
            here = self.perform(here, step);
        }
        here
    }

    fn move_one_step(&self, here: Player) -> Player {
        let next = here.after_move();
        match self.lookup(next.x, next.y) {
            SpaceContent::Ground => next,
            SpaceContent::Stone => here,
            SpaceContent::Empty => {
                let mut p = here.after_u_turn();
                loop {
                    let n = p.after_move();
                    if self.lookup(n.x, n.y) == SpaceContent::Empty {
                        break;
                    }
                    p = n;
                }
                if self.rows[p.y as usize][p.x as usize] == SpaceContent::Ground {
                    Player { dir: here.dir, ..p }
                } else {
                    here
                }
            }
        }
    }

    fn perform(&self, start: Player, step: Instruction) -> Player {
        match step {
            Instruction::Move(d) => {
                let mut here = start;
                for _ in 0..d {
                    here = self.move_one_step(here);
                }
                here
            }
            Instruction::TurnLeft => start.after_ccw_turn(),
            Instruction::TurnRight => start.after_cw_turn(),
        }
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

fn score(pos: Player) -> u32 {
    let dir_num = match pos.dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };
    (pos.y + 1) as u32 * 1000 + (pos.x + 1) as u32 * 4 + dir_num
}

fn parse_input(input: &[&str]) -> (Board, Vec<Instruction>) {
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
    let (board, instructions) = parse_input(&input);

    let start_pos = Player::new(start_x as i32, 0, Direction::Right);
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
            board.perform(Player::new(1, 1, Direction::Right), Instruction::Move(1)),
            Player::new(2, 1, Direction::Right)
        );
        assert_eq!(
            board.perform(Player::new(1, 1, Direction::Left), Instruction::Move(1)),
            Player::new(0, 1, Direction::Left)
        );
        assert_eq!(
            board.perform(Player::new(1, 1, Direction::Down), Instruction::Move(1)),
            Player::new(1, 2, Direction::Down)
        );
        assert_eq!(
            board.perform(Player::new(1, 1, Direction::Up), Instruction::Move(1)),
            Player::new(1, 0, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_multiple_moves_on_ground() {
        let board = Board::from(vec![".....", ".....", ".....", ".....", "....."]);
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Right), Instruction::Move(2)),
            Player::new(4, 2, Direction::Right)
        );
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Left), Instruction::Move(2)),
            Player::new(0, 2, Direction::Left)
        );
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Down), Instruction::Move(2)),
            Player::new(2, 4, Direction::Down)
        );
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Up), Instruction::Move(2)),
            Player::new(2, 0, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_a_single_move_into_stone() {
        let board = Board::from(vec![".#.", "#.#", ".#."]);
        assert_eq!(
            board.perform(Player::new(1, 1, Direction::Right), Instruction::Move(1)),
            Player::new(1, 1, Direction::Right)
        );
        assert_eq!(
            board.perform(Player::new(1, 1, Direction::Left), Instruction::Move(1)),
            Player::new(1, 1, Direction::Left)
        );
        assert_eq!(
            board.perform(Player::new(1, 1, Direction::Down), Instruction::Move(1)),
            Player::new(1, 1, Direction::Down)
        );
        assert_eq!(
            board.perform(Player::new(1, 1, Direction::Up), Instruction::Move(1)),
            Player::new(1, 1, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_a_single_move_to_the_other_side_with_empty_space_around() {
        let board = Board::from(vec!["", " .. ", " .. ", ""]);
        assert_eq!(
            board.perform(Player::new(2, 1, Direction::Right), Instruction::Move(1)),
            Player::new(1, 1, Direction::Right)
        );
        assert_eq!(
            board.perform(Player::new(1, 1, Direction::Left), Instruction::Move(1)),
            Player::new(2, 1, Direction::Left)
        );
        assert_eq!(
            board.perform(Player::new(1, 2, Direction::Down), Instruction::Move(1)),
            Player::new(1, 1, Direction::Down)
        );
        assert_eq!(
            board.perform(Player::new(1, 1, Direction::Up), Instruction::Move(1)),
            Player::new(1, 2, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_a_single_move_to_the_other_side_without_empty_space_around() {
        let board = Board::from(vec!["..", ".."]);
        assert_eq!(
            board.perform(Player::new(1, 0, Direction::Right), Instruction::Move(1)),
            Player::new(0, 0, Direction::Right)
        );
        assert_eq!(
            board.perform(Player::new(0, 0, Direction::Left), Instruction::Move(1)),
            Player::new(1, 0, Direction::Left)
        );
        assert_eq!(
            board.perform(Player::new(0, 1, Direction::Down), Instruction::Move(1)),
            Player::new(0, 0, Direction::Down)
        );
        assert_eq!(
            board.perform(Player::new(0, 0, Direction::Up), Instruction::Move(1)),
            Player::new(0, 1, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_a_single_move_to_the_other_side_into_stone() {
        let board = Board::from(vec!["", " .# ", " #. ", ""]);
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Right), Instruction::Move(1)),
            Player::new(2, 2, Direction::Right)
        );
        assert_eq!(
            board.perform(Player::new(1, 1, Direction::Left), Instruction::Move(1)),
            Player::new(1, 1, Direction::Left)
        );
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Down), Instruction::Move(1)),
            Player::new(2, 2, Direction::Down)
        );
        assert_eq!(
            board.perform(Player::new(1, 1, Direction::Up), Instruction::Move(1)),
            Player::new(1, 1, Direction::Up)
        );
    }

    #[test]
    fn board_walk_handles_left_turn() {
        let board = Board::from(vec![".....", ".....", ".....", ".....", "....."]);
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Right), Instruction::TurnLeft),
            Player::new(2, 2, Direction::Up)
        );
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Left), Instruction::TurnLeft),
            Player::new(2, 2, Direction::Down)
        );
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Down), Instruction::TurnLeft),
            Player::new(2, 2, Direction::Right)
        );
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Up), Instruction::TurnLeft),
            Player::new(2, 2, Direction::Left)
        );
    }

    #[test]
    fn score_uses_1_indexing() {
        assert_eq!(score(Player::new(0, 0, Direction::Right)), 1004);
        assert_eq!(score(Player::new(0, 0, Direction::Down)), 1005);
        assert_eq!(score(Player::new(0, 0, Direction::Left)), 1006);
        assert_eq!(score(Player::new(0, 0, Direction::Up)), 1007);

        assert_eq!(score(Player::new(1, 0, Direction::Right)), 1008);
        assert_eq!(score(Player::new(0, 1, Direction::Right)), 2004);
    }

    #[test]
    fn board_walk_handles_right_turn() {
        let board = Board::from(vec![".....", ".....", ".....", ".....", "....."]);
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Right), Instruction::TurnRight),
            Player::new(2, 2, Direction::Down)
        );
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Left), Instruction::TurnRight),
            Player::new(2, 2, Direction::Up)
        );
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Down), Instruction::TurnRight),
            Player::new(2, 2, Direction::Left)
        );
        assert_eq!(
            board.perform(Player::new(2, 2, Direction::Up), Instruction::TurnRight),
            Player::new(2, 2, Direction::Right)
        );
    }

    #[test]
    fn small_example_should_work() {
        assert_eq!(
            run_program(include_str!("ex1.txt").split('\n').collect()),
            6032
        );
    }

    #[test]
    fn big_example_should_work() {
        assert_eq!(
            run_program(include_str!("ex2.txt").split('\n').collect()),
            117102
        );
    }
}
