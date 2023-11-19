use instruction::{parse_instructions, Instruction};
use player::*;

mod instruction;
mod player;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpaceContent {
    Empty,
    Ground,
    Stone,
}

struct Board {
    rows: Vec<Vec<SpaceContent>>,
    layout: [[bool; 4]; 4],
    size: u32,
    is_cube: bool,
}

fn calc_square_size(rows: &[Vec<SpaceContent>]) -> u32 {
    let h = rows.len();
    let w = rows.iter().map(|r| r.len()).max().unwrap();

    if w % 3 == 0 {
        (h / 4) as u32
    } else if h % 3 == 0 {
        (w / 4) as u32
    } else {
        panic!("Not a valid layout");
    }
}

fn calc_layout(rows: &[Vec<SpaceContent>], size: u32) -> [[bool; 4]; 4] {
    let mut layout: [[bool; 4]; 4] = Default::default();

    #[allow(clippy::needless_range_loop)]
    for y in 0..4 {
        for x in 0..4 {
            let ry = y * size as usize;
            let rx = x * size as usize;
            if ry < rows.len() && rx < rows[ry].len() && rows[ry][rx] != SpaceContent::Empty {
                layout[y][x] = true;
            }
        }
    }
    layout
}

impl Board {
    fn new(rows: Vec<Vec<SpaceContent>>, is_cube: bool) -> Self {
        let size = calc_square_size(&rows);
        let layout = calc_layout(&rows, size);

        Board {
            rows,
            layout,
            size,
            is_cube,
        }
    }

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

    fn region(&self, x: i32, y: i32) -> (i32, i32, bool) {
        let rx = ((x + self.size as i32) as u32 / self.size) as i32 - 1;
        let ry = ((y + self.size as i32) as u32 / self.size) as i32 - 1;

        let is_within_layout = (0..4).contains(&ry) && (0..4).contains(&rx);
        let is_non_empty = is_within_layout && self.layout[ry as usize][rx as usize];

        (rx, ry, is_non_empty)
    }

    fn move_one_step(&self, here: Player) -> Player {
        let next = here.after_move();
        let region_next = self.region(next.x, next.y);
        let needs_to_wrap = !region_next.2;

        let next = if needs_to_wrap { self.wrap(here) } else { next };

        match self.lookup(next.x, next.y) {
            SpaceContent::Ground => next,
            SpaceContent::Stone => here,
            SpaceContent::Empty => {
                panic!("The wrap did'nt work! hx: {}, hy: {}, nx: {}, ny: {}", here.x, here.y, next.x, next.y);
            }
        }
    }

    fn wrap(&self, here: Player) -> Player {
        if self.is_cube {
            self.wrap_cube(here)
        } else {
            self.wrap_basic(here)
        }
    }

    fn wrap_cube(&self, here: Player) -> Player {
        use Direction::*;
        let s = self.size;

        let (rx, ry, _) = self.region(here.x, here.y);
        let p = here.translate(s, -rx, -ry);

        let dir = here.dir;
        let (cw_rotations, dest_rx, dest_ry) = if s == 4 { // small example
            match (rx, ry, dir) {
                (2, 0, Left) => (3, 1, 0),
                (2, 0, Up) => (2, 0, 0),
                (2, 0, Right) => (2, 4, 2),

                (2, 1, Right) => (1, 3, 1),
                
                (3, 2, Up) => (3, 3, 1),
                (3, 2, Right) => (2, 3, 0),
                (3, 2, Down) => (3, -1, 1),
                
                (2, 2, Down) => (2, 0, 2),
                (2, 2, Left) => (1, 1, 2),
                
                (1, 1, Down) => (3, 1, 2),
                (1, 1, Up) => (1, 1, 0),
                
                (0, 1, Down) => (2, 2, 3),
                (0, 1, Left) => (1, 3, 3),
                (0, 1, Up) => (2, 2, -1),

                _ => (0, rx, ry),
            }
        } else { // big example
            match (rx, ry, dir) {
                (1, 0, Left) => (2, -1, 2),
                (1, 0, Up) => (1, -1, 3),
                
                (2, 0, Up) => (0, 0, 4),
                (2, 0, Right) => (2, 2, 2),
                (2, 0, Down) => (1, 2, 1),
                
                (1, 1, Left) => (3, 0, 1),
                (1, 1, Right) => (3, 2, 1),
                
                (1, 2, Right) => (2, 3, 0),
                (1, 2, Down) => (1, 1, 3),
                
                (0, 2, Left) => (2, 0, 0),
                (0, 2, Up) => (1, 0, 1),
                
                (0, 3, Left) => (3, 1, -1),
                (0, 3, Down) => (0, 2, -1),
                (0, 3, Right) => (3, 1, 3),

                _ => (0, rx, ry),
            }
        };
        let mut p = p;
        for _ in 0..cw_rotations {
            p = p.rotate_cw(s);
        }
        p = p.translate(s, dest_rx, dest_ry);
        p.after_move()
    }

    fn wrap_basic(&self, here: Player) -> Player {
        let mut p = here.after_u_turn();
        loop {
            let n = p.after_move();
            if self.lookup(n.x, n.y) == SpaceContent::Empty {
                break;
            }
            p = n;
        }
        Player { dir: here.dir, ..p }
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

impl Board {
    fn parse(lines: Vec<&str>, cube: bool) -> Self {
        let rows = lines
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
            .collect();
        Board::new(rows, cube)
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

fn parse_input(input: &[&str], cube: bool) -> (Board, Vec<Instruction>) {
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

    let board = Board::parse(lines, cube);
    let instructions = parse_instructions(steps.unwrap());

    (board, instructions)
}

fn run_program(input: Vec<&str>, cube: bool) -> u32 {
    let start_x = input[0].find(|c| c == '.').unwrap();
    let (board, instructions) = parse_input(&input, cube);

    let start_pos = Player::new(start_x as i32, 0, Direction::Right);
    let end_pos = board.walk(start_pos, instructions);

    score(end_pos)
}

fn main() {
    let input: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap()).collect();
    let input = input.iter().map(|s| s.as_str()).collect();

    let result = run_program(input, false);

    println!("Score: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn calc_square_size_is_correct_ex1() {
        let input: Vec<_> = include_str!("ex1.txt").split('\n').collect();
        let (board, _) = parse_input(&input, false);
        assert_eq!(board.size, 4);
        assert_eq!(
            board.layout,
            [
                [false, false, true, false],
                [true, true, true, false],
                [false, false, true, true],
                [false, false, false, false]
            ]
        );
    }

    #[test]
    fn calc_square_size_is_correct_ex2() {
        let input: Vec<_> = include_str!("ex2.txt").split('\n').collect();
        let (board, _) = parse_input(&input, false);
        assert_eq!(board.size, 50);
        assert_eq!(
            board.layout,
            [
                [false, true, true, false],
                [false, true, false, false],
                [true, true, false, false],
                [true, false, false, false]
            ]
        );
    }

    #[test]
    fn small_example_should_work_part_1() {
        assert_eq!(
            run_program(include_str!("ex1.txt").split('\n').collect(), false),
            6032
        );
    }

    #[test]
    fn big_example_should_work_part_1() {
        assert_eq!(
            run_program(include_str!("ex2.txt").split('\n').collect(), false),
            117102
        );
    }

    #[test]
    fn small_example_should_work_part_2() {
        assert_eq!(
            run_program(include_str!("ex1.txt").split('\n').collect(), true),
            5031
        );
    }

    #[test]
    fn big_example_should_work_part_2() {
        assert_eq!(
            run_program(include_str!("ex2.txt").split('\n').collect(), true),
            135297
        );
    }
}
