use std::{
    collections::HashMap,
    fmt::Display,
};

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

type CacheKey = ([usize; 7], usize, usize);

struct BoardCacheValue {
    height: usize,
    shape_count: u64,
}

struct Board {
    rows: Vec<[bool; 7]>, // from the bottom
    wind: RepeatingSequence<WindDirection>,
    col_height: [usize; 7],
    cache: HashMap<CacheKey, BoardCacheValue>,
    shape_count: u64,
    extra_height: u64,
}

impl Board {
    fn new(wind: RepeatingSequence<WindDirection>) -> Board {
        Board {
            rows: Vec::new(),
            wind,
            col_height: Default::default(),
            cache: HashMap::new(),
            shape_count: 0,
            extra_height: 0,
        }
    }

    fn tower_height(&self) -> u64 {
        self.extra_height + self.rows.len() as u64
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
                self.rows.push(Default::default());
            }
            for dx in 0..w {
                if shape.exists_at(dx, dy) {
                    self.rows[y + dy][x + dx] = true;
                    if self.col_height[x + dx] < y + dy {
                        self.col_height[x + dx] = y + dy;
                    }
                }
            }
        }

        self.shape_count += 1;
    }

    /** returns the number of shapes added (usually 1) */
    fn add_shape(&mut self, shape: Shape, shape_idx: usize, shapes_left: u64) -> u64 {
        let mut shapes_added = 0;

        let mut cache_key: Option<CacheKey> = None;
        // make sure we still need caching
        if self.extra_height == 0 {
            let mut col_depths: [usize; 7] = Default::default();
            for (i, col_depth) in col_depths.iter_mut().enumerate() {
                *col_depth = self.rows.len() - self.col_height[i];
            }
            cache_key = Some((col_depths, self.wind.index, shape_idx));
        }

        if let Some(key) = cache_key {
            if let Some(BoardCacheValue {
                height,
                shape_count,
            }) = self.cache.get(&key)
            {
                let current_height = self.rows.len();
                let current_shape_count = self.shape_count;

                let cycle_height = current_height - height;
                let cycle_length = current_shape_count - shape_count;

                let cycles_left = shapes_left / cycle_length;

                self.extra_height = cycles_left * cycle_height as u64;
                shapes_added = cycles_left * cycle_length;
                self.shape_count += shapes_added;

                cache_key = None;
            }
        }

        if let Some(key) = cache_key {
            //println!("{:?}, {}, {}", key, self.rows.len(), self.shape_count);
            // it has not modded yet
            self.cache.insert(
                key,
                BoardCacheValue {
                    height: self.rows.len(),
                    shape_count: self.shape_count,
                },
            );
        }

        let start_y = self.rows.len() + 3;

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
        shapes_added += 1;

        shapes_added
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

pub fn read_input(input: &str) -> String {
    input.lines().next().unwrap().to_string()
}

pub fn find_tower_height(input: String, iterations: u64) -> u64 {
    let wind = RepeatingSequence::new(parse_wind_list(input).unwrap());

    let mut shapes = RepeatingSequence::new(vec![
        Shape::Dash,
        Shape::Plus,
        Shape::Bracket,
        Shape::Bar,
        Shape::Box,
    ]);

    let mut board = Board::new(wind);

    let mut iteration = 0u64;
    loop {
        if iteration >= iterations {
            break;
        }
        let shape = shapes.next().unwrap();
        let shapes_added = board.add_shape(shape, shapes.index, iterations - iteration);
        if shapes_added < 1 {
            panic!("less than one shape added!");
        }
        iteration += shapes_added;
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

        board.add_shape(Shape::Dash, 0, 1);

        assert_eq!(board.tower_height(), 1);
    }

    #[test]
    fn a_board_with_a_plus_has_height_3() {
        let mut board = Board::new(make_left_wind());

        board.add_shape(Shape::Plus, 0, 1);
        assert_eq!(board.tower_height(), 3);
    }

    #[test]
    fn a_board_with_a_bracket_has_height_3() {
        let mut board = Board::new(make_left_wind());

        board.add_shape(Shape::Bracket, 0, 1);
        assert_eq!(board.tower_height(), 3);
    }

    #[test]
    fn a_board_with_a_bar_has_height_4() {
        let mut board = Board::new(make_left_wind());

        board.add_shape(Shape::Bar, 0, 1);
        assert_eq!(board.tower_height(), 4);
    }

    #[test]
    fn a_board_with_a_box_has_height_2() {
        let mut board = Board::new(make_left_wind());

        board.add_shape(Shape::Box, 0, 1);
        assert_eq!(board.tower_height(), 2);
    }

    #[test]
    fn a_board_with_two_bars_has_height_8() {
        let mut board = Board::new(make_left_wind());

        board.add_shape(Shape::Bar, 0, 1);
        board.add_shape(Shape::Bar, 0, 1);
        assert_eq!(board.tower_height(), 8);
    }

    #[test]
    fn example_works() {
        let input = include_str!("ex1.txt").to_string();
        assert_eq!(find_tower_height(input, 1000000000000), 1514285714288);
    }

    #[test]
    fn big_example_works() {
        let input = include_str!("ex2.txt").to_string();
        assert_eq!(find_tower_height(input, 1000000000000), 1542941176480);
    }
}
