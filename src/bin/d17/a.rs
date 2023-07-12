use std::{
    fmt::Display,
    io::{self, BufRead},
};

pub fn main() {
    println!("Hello, world!");

    let lines = read_input();
    let result = find_tower_height(lines, 2022);

    println!("{}", result);
}

struct RepeatingSequence<T> {
    items: Vec<T>,
    index: usize,
}

impl<T> RepeatingSequence<T>
where
    T: Copy,
{
    fn new(items: Vec<T>) -> RepeatingSequence<T> {
        RepeatingSequence { items, index: 0 }
    }
}

impl<T: Copy> Iterator for RepeatingSequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items[self.index];
        self.index += 1;
        if self.index == self.items.len() {
            self.index = 0;
        }
        Some(item)
    }
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    Dash,
    Plus,
    Bracket,
    Bar,
    Box,
}

impl Shape {
    fn width(&self) -> usize {
        use Shape::*;
        match self {
            Dash => 4,
            Plus => 3,
            Bracket => 3,
            Bar => 1,
            Box => 2,
        }
    }

    fn height(&self) -> usize {
        use Shape::*;
        match self {
            Dash => 1,
            Plus => 3,
            Bracket => 3,
            Bar => 4,
            Box => 2,
        }
    }

    /** This function assumes that 0 <= x < width and 0 <= y < height */
    fn exists_at(&self, x: usize, y: usize) -> bool {
        use Shape::*;
        match self {
            Dash => true,
            Plus => x == 1 || y == 1,
            Bracket => x == 2 || y == 0,
            Bar => true,
            Box => true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WindDirection {
    Left,
    Right,
}

fn parse_wind_list(line: String) -> Result<Vec<WindDirection>, String> {
    line.chars()
        .map(|c| match c {
            '<' => Ok(WindDirection::Left),
            '>' => Ok(WindDirection::Right),
            _ => Err(format!("invalid wind direction: {}", c)),
        })
        .collect::<Result<Vec<_>, _>>()
}

struct Board {
    rows: Vec<[bool; 7]>, // from the bottom
    wind: RepeatingSequence<WindDirection>,
}

impl Board {
    fn new(wind: RepeatingSequence<WindDirection>) -> Board {
        Board {
            rows: Vec::new(),
            wind,
        }
    }

    fn tower_height(&self) -> usize {
        self.rows.len()
    }

    fn overlaps_with_shape(&self, shape: Shape, x: usize, y: usize) -> bool {
        let w = shape.width();
        let h = shape.height();

        for dy in 0..h {
            if y + dy >= self.rows.len() {
                break;
            }
            for dx in 0..w {
                if self.rows[y + dy][x + dx] && shape.exists_at(dx, dy) {
                    return true;
                }
            }
        }
        false
    }

    fn add_shape_at(&mut self, shape: Shape, x: usize, y: usize) {
        let w = shape.width();
        let h = shape.height();

        for dy in 0..h {
            if self.rows.len() <= y + dy {
                self.rows.push(Default::default())
            }
            for dx in 0..w {
                if shape.exists_at(dx, dy) {
                    self.rows[y + dy][x + dx] = true;
                }
            }
        }
    }

    fn add_shape(&mut self, shape: Shape) {
        let start_y = self.tower_height() + 3;

        let mut x: usize = 2;
        let mut y: usize = start_y;

        loop {
            match self.wind.next().unwrap() {
                WindDirection::Left => {
                    if x > 0 && !self.overlaps_with_shape(shape, x - 1, y) {
                        x -= 1
                    }
                }
                WindDirection::Right => {
                    if x + shape.width() < 7 && !self.overlaps_with_shape(shape, x + 1, y) {
                        x += 1
                    }
                }
            };

            if y == 0 || self.overlaps_with_shape(shape, x, y - 1) {
                break;
            } else {
                y -= 1;
            }
        }

        self.add_shape_at(shape, x, y);
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rows
                .iter()
                .rev()
                .map(|row| {
                    row.iter()
                        .map(|is_on| if *is_on { '#' } else { '.' })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

fn read_input() -> String {
    let stdin = io::stdin();
    stdin.lock().lines().next().unwrap().unwrap()
}

fn find_tower_height(input: String, iterations: u32) -> usize {
    let wind = RepeatingSequence::new(parse_wind_list(input).unwrap());

    let mut shapes = RepeatingSequence::new(vec![
        Shape::Dash,
        Shape::Plus,
        Shape::Bracket,
        Shape::Bar,
        Shape::Box,
    ]);

    let mut board = Board::new(wind);

    for _ in 0..iterations {
        let shape = shapes.next().unwrap();
        board.add_shape(shape);
    }

    board.tower_height()
}

#[cfg(test)]
mod tests {
    use super::{
        find_tower_height, parse_wind_list, Board, RepeatingSequence, Shape, WindDirection,
    };

    fn make_left_wind() -> RepeatingSequence<WindDirection> {
        RepeatingSequence::new(vec![WindDirection::Left])
    }

    #[test]
    fn repeating_sequence_returns_values_in_order() {
        let mut seq = RepeatingSequence::new(vec![1, 2, 3]);

        assert_eq!(seq.next(), Some(1));
        assert_eq!(seq.next(), Some(2));
        assert_eq!(seq.next(), Some(3));
    }

    #[test]
    fn repeating_sequence_repeats() {
        let mut seq = RepeatingSequence::new(vec![1, 2, 3]);

        seq.next();
        seq.next();
        seq.next();

        assert_eq!(seq.next(), Some(1));
        assert_eq!(seq.next(), Some(2));
        assert_eq!(seq.next(), Some(3));
        assert_eq!(seq.next(), Some(1));
    }

    #[test]
    fn parse_wind_list_works() {
        use super::WindDirection::*;
        assert_eq!(
            parse_wind_list("<<<>><".to_string()),
            Ok(vec![Left, Left, Left, Right, Right, Left])
        )
    }

    #[test]
    fn an_empty_board_has_height_0() {
        let board = Board::new(make_left_wind());

        assert_eq!(board.tower_height(), 0);
    }

    #[test]
    fn a_board_with_a_dash_has_height_1() {
        let mut board = Board::new(make_left_wind());

        board.add_shape(Shape::Dash);

        assert_eq!(board.tower_height(), 1);
    }

    #[test]
    fn a_board_with_a_plus_has_height_3() {
        let mut board = Board::new(make_left_wind());

        board.add_shape(Shape::Plus);
        assert_eq!(board.tower_height(), 3);
    }

    #[test]
    fn a_board_with_a_bracket_has_height_3() {
        let mut board = Board::new(make_left_wind());

        board.add_shape(Shape::Bracket);
        assert_eq!(board.tower_height(), 3);
    }

    #[test]
    fn a_board_with_a_bar_has_height_4() {
        let mut board = Board::new(make_left_wind());

        board.add_shape(Shape::Bar);
        assert_eq!(board.tower_height(), 4);
    }

    #[test]
    fn a_board_with_a_box_has_height_2() {
        let mut board = Board::new(make_left_wind());

        board.add_shape(Shape::Box);
        assert_eq!(board.tower_height(), 2);
    }

    #[test]
    fn a_board_with_two_bars_has_height_8() {
        let mut board = Board::new(make_left_wind());

        board.add_shape(Shape::Bar);
        board.add_shape(Shape::Bar);
        assert_eq!(board.tower_height(), 8);
    }

    #[test]
    fn example_works() {
        let input = include_str!("ex1.txt").to_string();
        assert_eq!(find_tower_height(input, 2022), 3068);
    }

    #[test]
    fn big_example_works() {
        let input = include_str!("ex2.txt").to_string();
        assert_eq!(find_tower_height(input, 2022), 3127);
    }
}
