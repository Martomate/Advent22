use std::{
    collections::HashSet,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn neighbors(self) -> Vec<Point> {
        [-1, 1]
            .iter()
            .flat_map(|d| {
                [
                    Point {
                        x: self.x + d,
                        ..self
                    },
                    Point {
                        y: self.y + d,
                        ..self
                    },
                    Point {
                        z: self.z + d,
                        ..self
                    },
                ]
            })
            .collect()
    }
}

struct Model {
    pixels: HashSet<Point>,
}

impl Model {
    fn from_pixels(pixels: &[Point]) -> Model {
        return Model {
            pixels: HashSet::from_iter(pixels.iter().copied()),
        };
    }

    fn neighbor_count(&self, p: Point) -> u32 {
        p.neighbors()
            .iter()
            .filter(|n| !self.pixels.contains(n))
            .count() as u32
    }

    fn calculate_surface_area(&self) -> u32 {
        self.pixels.iter().map(|&p| self.neighbor_count(p)).sum()
    }
}

pub fn main() {
    let stdin = io::stdin();

    let mut lines = Vec::new();

    for l in stdin.lock().lines() {
        let line = l.unwrap();

        if line.is_empty() {
            break;
        }

        lines.push(line);
    }

    let area = run_program(lines);

    println!("{}", area);
}

fn run_program(lines: Vec<String>) -> u32 {
    let pixels = parse_input(lines);

    let model = Model::from_pixels(&pixels);

    model.calculate_surface_area()
}

fn parse_input(lines: Vec<String>) -> Vec<Point> {
    let mut points = Vec::new();

    for line in lines {
        let coords: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        points.push(Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }

    points
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::{parse_input, run_program, Point};

    #[test]
    fn parse_input_should_work_with_one_line() {
        let lines = vec!["1,2,3".to_owned()];
        assert_eq!(parse_input(lines), vec![Point { x: 1, y: 2, z: 3 }]);
    }

    #[test]
    fn parse_input_should_work_with_multiple_lines() {
        let lines = vec![
            "1,2,3".to_owned(),
            "1,200,3".to_owned(),
            "1,2,-3".to_owned(),
        ];
        assert_eq!(
            parse_input(lines),
            vec![
                Point { x: 1, y: 2, z: 3 },
                Point { x: 1, y: 200, z: 3 },
                Point { x: 1, y: 2, z: -3 }
            ]
        );
    }

    #[test]
    fn example_works() {
        let lines = include_str!("ex1.txt")
            .lines()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(run_program(lines), 64);
    }

    #[test]
    fn big_example_works() {
        let lines = include_str!("ex2.txt")
            .lines()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(run_program(lines), 3564);
    }
}
