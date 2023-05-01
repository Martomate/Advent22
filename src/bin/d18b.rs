use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use queues::{IsQueue, Queue};

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
    exterior: HashSet<Point>,
}

impl Model {
    fn from_pixels(pixels: &[Point]) -> Model {
        Model {
            pixels: HashSet::from_iter(pixels.iter().copied()),
            exterior: Model::calculate_exterior(pixels),
        }
    }

    fn neighbor_count(&self, p: Point) -> u32 {
        p.neighbors()
            .iter()
            .filter(|n| self.exterior.contains(n))
            .count() as u32
    }

    fn calculate_exterior(pixels: &[Point]) -> HashSet<Point> {
        let xlo = pixels.iter().map(|p| p.x).min().unwrap() - 1;
        let ylo = pixels.iter().map(|p| p.y).min().unwrap() - 1;
        let zlo = pixels.iter().map(|p| p.z).min().unwrap() - 1;
        let xhi = pixels.iter().map(|p| p.x).max().unwrap() + 1;
        let yhi = pixels.iter().map(|p| p.y).max().unwrap() + 1;
        let zhi = pixels.iter().map(|p| p.z).max().unwrap() + 1;

        let mut exterior = HashSet::<Point>::new();
        let mut q: Queue<Point> = Queue::new();
        for (x, y, z) in [
            (xlo, ylo, zlo),
            (xhi, ylo, zlo),
            (xlo, yhi, zlo),
            (xlo, ylo, zhi),
            (xhi, yhi, zlo),
            (xhi, ylo, zhi),
            (xlo, yhi, zhi),
            (xhi, yhi, zhi)] {
            let p = Point { x, y, z };
            exterior.insert(p);
            q.add(p).unwrap();
        }

        while let Ok(p) = q.remove() {
            for n in p.neighbors() {
                if n.x < xlo || n.x > xhi || n.y < ylo || n.y > yhi || n.z < zlo || n.z > zhi {
                    continue;
                }
                if !pixels.contains(&n) && !exterior.contains(&n) {
                    exterior.insert(n);
                    q.add(n).unwrap();
                }
            }
        }

        exterior
    }

    fn calculate_surface_area(&self) -> u32 {
        self.pixels.iter().map(|&p| self.neighbor_count(p)).sum()
    }
}

fn main() {
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

    use crate::{parse_input, run_program, Point};

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
        let lines = vec![
            "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
            "1,2,5", "3,2,5", "2,1,5", "2,3,5",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(run_program(lines), 58);
    }

    #[test]
    fn big_example_works() {
        let lines = "4,14,13
12,7,15
7,14,15
7,16,12
7,9,18
13,8,15
17,10,6
14,2,11
12,10,2
2,8,6
4,4,6
7,14,4
7,3,10
8,9,16
5,15,7
14,4,7
15,11,6
14,12,5
4,11,15
2,10,6
16,12,9
2,6,10
9,14,5
18,10,8
13,14,13
10,17,12
6,15,7
13,12,15
6,8,17
12,7,16
17,10,10
3,8,5
4,13,6
14,10,3
3,5,10
11,17,13
9,13,14
4,3,12
12,8,2
4,3,13
5,4,9
3,8,12
6,2,12
15,9,3
2,6,7
10,5,4
7,16,7
10,14,2
9,10,15
9,12,17
13,15,6
6,4,9
16,12,13
6,15,10
16,7,14
3,10,7
11,6,3
10,6,15
1,9,12
1,11,9
17,8,5
13,7,3
10,2,11
17,7,11
4,10,16
8,3,14
16,9,6
11,3,11
9,6,17
14,17,7
6,8,3
9,16,6
8,18,9
12,5,17
15,10,12
8,14,3
3,9,15
4,7,15
16,13,8
17,10,12
12,5,16
11,18,11
14,13,16
7,9,16
5,6,16
4,6,4
4,12,6
8,6,2
2,9,8
12,3,6
7,2,7
2,10,14
11,5,17
8,5,16
16,9,7
11,17,7
8,5,2
13,11,4
7,7,3
10,6,4
8,5,3
2,8,8
7,18,8
14,9,3
13,12,17
12,6,3
17,8,8
10,9,2
11,3,6
11,12,17
9,14,16
10,3,8
10,3,6
13,3,13
3,13,6
8,18,10
13,3,9
7,1,10
7,16,5
6,2,9
14,6,15
4,9,13
5,4,11
15,5,5
2,11,8
7,3,8
2,12,7
11,9,17
5,15,9
10,12,1
13,4,5
3,14,11
6,11,16
13,16,14
11,13,16
9,7,4
14,5,4
3,8,13
7,6,17
17,12,10
16,11,10
15,5,13
7,12,4
5,5,15
11,15,4
8,17,8
16,5,7
4,8,13
12,12,16
12,12,2
6,7,2
4,3,9
4,13,13
9,13,2
16,14,10
14,4,11
6,4,12
12,2,11
13,16,9
16,12,14
5,12,3
5,8,3
10,6,2
11,8,1
7,7,2
17,4,9
6,15,6
10,16,13
12,13,4
4,10,3
5,13,16
5,10,15
15,7,4
17,8,10
14,17,12
11,2,9
14,15,12
9,1,9
11,8,17
7,4,4
1,7,8
3,8,8
11,6,17
8,12,17
4,14,14
8,3,5
7,18,9
7,16,13
11,15,14
8,17,5
9,11,2
12,13,15
3,8,6
7,16,14
15,4,8
14,4,14
14,15,6
5,6,13
13,17,12
13,17,8
9,7,3
11,13,17
14,3,6
15,13,15
2,10,13
7,6,3
13,2,9
5,6,5
6,16,8
17,12,8
2,12,8
4,15,7
4,12,12
6,13,5
13,17,10
16,5,9
17,14,9
9,11,16
5,4,15
10,7,2
15,11,5
8,11,0
3,11,12
4,6,7
14,12,2
5,6,17
11,16,15
9,5,2
5,14,9
4,9,12
2,12,5
12,3,13
6,3,9
14,14,6
7,16,6
14,5,15
3,13,7
12,2,10
13,12,2
11,8,18
18,11,9
2,13,5
12,15,6
14,3,11
12,2,7
6,5,6
9,10,2
18,10,10
9,1,10
10,5,14
14,3,9
3,12,5
9,12,3
17,8,9
14,16,11
15,9,4
3,6,6
11,9,2
9,17,11
14,7,15
9,10,1
15,11,3
8,3,8
12,7,3
15,13,12
3,6,8
5,14,14
3,14,12
3,7,7
10,4,5
14,17,9
16,12,7
6,7,17
16,4,6
4,14,7
13,13,15
10,17,6
1,10,9
13,16,15
8,13,4
7,5,4
15,5,6
14,7,4
8,15,15
3,16,9
11,18,9
9,17,9
6,14,3
10,10,1
15,15,9
4,3,10
8,4,13
17,9,9
7,18,10
10,8,18
7,7,17
2,11,9
5,17,8
4,13,14
14,16,7
11,14,2
3,10,14
13,13,3
13,16,6
4,12,13
16,7,10
3,11,13
11,7,17
5,16,7
10,2,9
2,11,12
13,5,4
14,12,11
3,6,13
3,9,13
6,13,3
16,16,13
17,9,13
13,6,4
12,15,8
6,11,17
14,7,7
7,17,10
9,9,2
4,15,9
4,6,5
11,3,8
14,2,10
15,11,4
7,5,15
7,2,11
4,9,16
13,3,11
15,4,10
4,5,5
11,16,7
16,9,15
13,9,2
14,14,15
10,9,16
2,6,9
7,13,16
16,6,11
10,6,1
10,12,17
14,9,17
15,6,4
4,13,10
16,10,14
6,12,16
13,6,15
12,16,6
12,15,13
15,10,15
12,3,11
9,6,4
12,9,17
17,6,7
11,17,12
5,4,4
12,7,17
14,10,17
3,10,5
15,15,10
6,16,10
3,9,11
12,3,14
10,13,16
13,4,14
17,11,10
7,14,6
12,7,2
13,17,7
4,5,6
9,9,18
8,4,8
12,16,9
2,6,13
2,9,10
7,3,9
12,1,9
14,8,3
4,6,10
7,4,7
7,13,15
4,9,15
10,11,16
18,6,9
7,9,3
2,7,9
14,10,15
2,10,10
15,5,10
4,4,5
11,17,6
15,9,14
14,4,12
8,14,15
3,13,14
6,12,3
17,9,12
9,15,4
10,15,15
11,12,2
4,15,12
9,16,15
9,7,2
4,4,11
17,13,7
16,15,10
8,13,3
4,12,14
4,7,4
11,4,5
17,7,9
11,10,1
9,5,15
10,15,13
2,6,12
5,3,8
14,14,14
15,5,7
9,3,4
9,9,17
15,8,15
6,17,12
13,16,10
17,12,14
4,11,16
15,10,5
7,3,6
11,5,2
3,5,9
3,14,8
7,13,4
6,2,7
4,8,4
9,7,1
3,5,8
4,4,10
8,18,7
8,4,15
3,12,6
16,10,9
16,7,7
15,13,11
3,10,6
3,11,6
12,13,16
16,12,5
5,5,5
4,6,6
4,14,9
11,5,15
6,8,2
15,11,9
16,11,15
4,7,12
5,7,15
16,7,11
3,11,8
13,14,12
9,17,10
9,5,6
15,11,15
3,4,12
11,11,3
15,8,14
13,3,8
6,11,3
11,13,1
5,13,14
9,6,2
10,13,3
5,10,3
6,6,15
15,7,15
15,3,13
6,9,2
15,2,8
10,15,3
3,12,9
12,5,4
4,3,11
7,6,16
16,5,10
6,1,12
15,13,14
5,3,12
9,12,2
7,15,14
8,3,13
15,11,10
15,7,13
3,5,11
16,3,12
14,4,13
2,8,11
3,9,5
4,8,6
17,10,8
9,10,3
16,8,13
8,16,9
14,13,5
6,4,13
5,2,10
14,13,15
17,13,11
15,15,14
7,17,7
6,3,11
6,6,3
13,5,5
6,13,15
8,9,1
15,12,5
4,6,14
11,15,15
15,8,5
4,13,4
12,8,17
4,11,14
13,5,14
12,2,9
3,6,11
13,10,3
6,5,8
11,3,5
6,16,12
8,4,5
12,12,17
17,13,8
16,9,5
7,1,9
3,6,9
14,6,4
2,5,13
14,15,8
10,15,16
6,17,8
15,4,6
14,16,13
2,13,12
17,10,4
3,9,14
16,13,6
8,15,3
1,6,9
8,1,7
2,7,12
10,15,4
13,3,14
9,6,3
13,11,3
10,7,16
15,4,11
5,7,7
13,14,4
7,5,13
5,10,16
15,3,11
8,11,1
4,3,8
10,5,3
11,9,3
5,13,12
17,5,10
13,7,16
14,15,4
13,3,7
16,5,6
2,9,7
3,14,9
13,15,10
12,6,15
4,8,15
1,10,8
3,9,7
13,9,17
7,17,8
10,9,19
14,5,3
12,15,14
4,9,4
16,12,12
8,15,6
9,13,3
12,3,10
13,16,11
14,14,13
15,16,12
13,16,7
3,7,11
3,14,10
9,17,12
2,9,13
7,5,17
4,11,13
13,2,7
7,2,8
17,9,11
10,17,9
13,4,4
3,13,5
6,15,5
4,7,8
16,14,12
10,2,8
16,3,10
7,12,14
17,12,9
7,17,12
13,8,2
10,17,8
10,9,17
18,7,11
4,4,12
5,11,5
9,10,18
12,2,5
11,16,11
18,10,7
12,2,6
12,9,2
16,6,9
3,8,14
18,8,8
14,16,6
9,3,5
11,2,7
7,5,5
12,5,15
10,8,16
14,16,8
11,3,7
14,13,13
10,2,5
12,11,2
14,12,4
14,4,5
1,11,7
11,10,16
13,4,8
2,11,10
18,10,11
5,11,6
9,15,16
11,8,3
5,16,13
5,6,4
16,15,13
6,16,15
8,11,2
10,8,17
12,2,13
6,6,14
10,6,17
13,9,4
13,4,12
17,6,11
9,6,16
11,13,3
2,9,9
10,17,13
14,5,5
13,3,6
12,15,7
4,5,13
9,1,8
8,10,15
4,8,14
5,7,4
14,16,10
4,15,8
4,7,14
5,16,6
8,16,12
11,18,13
7,12,2
9,11,18
5,3,5
9,18,10
4,15,11
7,16,8
14,6,14
7,11,3
13,15,13
6,12,15
16,16,9
9,12,4
11,16,8
17,11,8
12,5,5
10,10,18
13,16,13
6,13,16
15,9,15
6,2,8
15,12,16
5,5,4
16,15,11
4,5,9
14,14,12
4,15,6
7,16,4
8,7,3
5,10,1
8,17,12
13,6,13
17,8,12
17,8,13
13,2,10
6,7,15
5,5,8
10,4,14
7,12,17
6,5,15
13,14,7
7,9,17
16,5,4
14,12,12
10,11,17
8,16,13
9,3,13
16,5,13
7,2,9
13,4,6
11,2,5
6,14,12
16,9,14
14,11,15
14,13,14
5,5,10
15,4,7
10,14,16
1,9,7
5,17,7
8,17,13
6,15,14
2,9,12
8,12,3
2,9,5
1,10,7
3,12,11
3,7,12
14,8,16
17,8,6
7,16,11
17,7,13
9,7,18
1,6,13
18,9,10
13,14,15
4,5,11
3,9,4
4,13,12
16,16,12
17,5,11
15,3,7
3,12,10
2,10,7
3,8,15
3,6,7
8,15,16
4,13,5
9,1,11
3,6,10
2,13,11
13,3,5
5,9,2
14,3,5
16,4,7
15,5,11
7,10,2
8,3,12
11,10,2
1,11,13
17,11,7
11,5,3
10,16,12
4,14,4
3,8,9
11,16,5
18,7,8
15,9,6
14,15,10
2,6,11
6,12,18
6,10,2
8,6,17
3,4,11
6,3,13
11,0,10
6,9,3
14,13,10
17,8,11
16,14,13
14,17,11
5,11,4
13,2,12
17,7,8
17,9,8
6,15,15
9,4,6
12,4,16
16,15,8
3,12,7
13,7,4
9,16,12
8,2,15
9,18,11
11,6,16
16,14,7
14,7,13
13,11,15
3,7,13
4,14,8
8,2,4
10,4,3
10,3,4
16,7,13
11,2,8
11,8,16
7,3,13
15,7,11
9,16,7
13,2,8
16,13,10
12,5,13
8,1,9
4,9,5
14,16,12
3,15,10
16,13,11
15,6,5
15,9,13
11,14,15
11,4,4
9,14,15
15,8,6
4,13,7
6,16,13
2,5,7
2,8,13
1,8,12
15,15,8
2,9,6
10,8,1
12,16,11
2,7,8
8,17,11
6,17,13
7,4,12
5,15,11
14,8,13
6,16,9
8,3,7
8,2,8
12,4,15
14,10,7
18,11,10
13,17,6
9,4,15
13,3,12
9,3,9
14,7,16
10,2,14
16,11,8
11,5,11
10,6,16
16,8,14
4,10,5
8,14,16
15,7,7
11,3,13
1,6,10
10,13,15
6,17,11
5,15,6
8,8,17
8,10,2
2,5,9
2,5,8
15,7,6
16,9,9
11,11,17
16,14,6
3,14,4
17,11,6
14,16,9
14,13,12
5,11,15
14,10,16
10,1,10
11,14,14
12,3,9
11,13,4
3,6,12
12,17,6
9,8,18
10,3,10
4,7,5
13,15,14
2,7,6
7,4,16
8,7,2
5,7,6
11,16,4
15,14,9
9,16,4
17,12,12
15,4,9
14,5,13
15,7,8
17,9,6
6,12,4
8,6,16
7,17,14
15,13,10
2,13,9
10,16,11
8,4,6
9,2,5
3,9,16
9,16,9
15,15,6
2,14,9
10,7,18
6,7,16
10,12,2
14,8,17
8,2,6
11,5,4
12,4,14
3,11,15
4,10,13
15,6,13
11,12,3
14,13,4
14,15,13
4,14,11
7,2,12
9,18,9
10,2,12
5,7,16
2,7,13
7,10,18
13,8,5
10,11,1
14,11,3
2,11,11
18,10,9
17,11,14
16,6,14
3,7,6
6,5,14
13,11,16
8,16,4
11,2,12
7,7,16
10,9,18
11,11,1
16,14,8
17,14,11
18,8,7
6,8,15
12,4,4
9,5,5
15,5,9
11,5,16
14,14,7
2,7,7
5,14,5
13,8,4
2,4,8
7,11,17
5,7,3
12,13,5
13,7,5
12,3,8
4,11,12
7,17,5
16,13,7
7,3,12
15,9,16
13,7,15
7,12,3
8,3,9
16,13,9
11,10,17
6,4,4
16,7,15
15,11,14
15,15,11
10,3,12
18,8,10
13,9,3
6,17,6
9,2,9
12,17,9
7,15,5
12,3,12
10,15,12
15,8,16
8,15,8
15,14,12
16,13,12
5,9,16
1,8,9
9,8,1
11,9,1
9,11,3
14,12,13
11,14,17
7,8,18
3,7,10
9,2,7
13,15,3
4,12,4
11,16,13
13,11,1
10,4,2
5,10,13
15,6,14
5,13,3
9,6,15
16,7,4
11,15,10
8,12,16
13,14,5
12,14,16
10,1,8
8,13,16
13,4,15
6,17,9
10,14,15
4,14,12
12,14,15
11,3,15
8,8,16
14,8,4
5,9,3
7,4,13
7,15,6
3,10,10
9,14,1
14,8,5
12,11,15
11,1,10
15,6,7
15,3,9
13,6,5
8,9,17
5,7,2
14,4,8
17,11,11
6,16,11
16,10,6
11,11,16
10,7,5
7,5,18
6,4,11
10,2,6
14,12,14
10,1,7
6,8,16
2,12,12
13,17,9
13,6,2
7,2,13
1,7,6
3,13,13
14,11,17
12,3,7
15,14,15
7,15,4
10,10,17
2,7,11
8,4,16
12,10,17
8,7,4
12,4,8
2,4,10
16,11,9
15,16,9
17,11,13
9,0,8
13,15,5
4,12,7
3,11,7
10,14,13
4,8,5
6,7,13
9,16,14
5,3,7
7,8,2
11,7,1
9,17,13
11,7,2
8,2,7
10,3,3
12,15,11
15,13,13
10,10,3
7,10,15
12,16,7
3,8,10
4,4,9
11,17,8
3,3,7
8,8,18
16,4,8
15,5,8
10,2,4
3,9,12
12,8,3
13,4,13
7,17,6
5,4,14
7,2,6
6,7,4
11,8,4
13,10,2
9,17,14
5,15,13
15,8,4
7,14,16
6,4,5
16,13,14
8,5,17
10,7,17
9,2,6
5,12,16
13,8,17
8,6,5
5,14,4
14,14,4
15,14,4
15,3,6
18,8,12
13,16,12
11,3,14
4,6,11
16,5,8
11,4,15
5,16,10
10,5,16
7,10,3
4,15,10
3,11,9
12,14,4
17,8,7
2,12,13
11,14,4
11,4,16
10,2,7
10,17,5
15,12,14
10,8,2
12,15,4
1,11,8
4,10,14
17,14,13
17,6,10
15,8,3
3,12,15
6,6,18
15,4,12
11,14,16
16,4,10
6,8,4
15,7,5
8,7,1
6,10,3
4,5,8
6,3,5
17,10,13
4,11,4
7,15,15
4,16,7
13,18,8
7,7,15
5,13,13
2,8,9
3,5,12
16,8,4
2,11,14
16,8,5
14,6,10
16,12,11
13,11,5
12,18,10
15,12,8
8,2,9
14,3,10
5,4,8
5,13,5
13,5,15
6,11,5
14,14,5
7,1,11
11,1,12
8,2,5
10,16,8
5,4,5
5,5,14
14,15,5
2,9,11
17,10,11
5,16,15
16,13,13
5,8,16
7,14,17
4,12,15
6,12,17
9,2,11
9,18,12
5,11,3
16,11,5
13,6,6
12,10,3
5,17,10
3,6,4
13,9,16
7,15,12
6,14,5
5,6,12
5,13,15
7,16,15
17,12,11
8,19,12
8,8,3
8,17,7
3,11,4
8,16,5
12,16,8
9,4,5
11,4,3
15,12,12
5,5,13
7,4,14
7,2,10
6,17,7
16,6,7
10,16,14
11,16,14
17,10,7
9,17,7
12,15,15
10,16,5
7,4,17
13,5,16
13,11,17
3,12,4
12,4,6
8,12,1
11,4,14
14,6,16
3,13,12
2,7,10
11,15,5
16,8,8
17,10,5
8,3,15
16,11,4
3,7,5
9,5,17
8,14,5
11,3,12
6,11,4
5,16,8
2,6,6
3,10,13
6,3,6
16,10,12
16,8,15
13,13,4
10,2,10
9,7,17
9,2,12
3,13,11
13,18,12
11,12,18
10,18,7
3,12,12
16,12,8
8,17,10
14,15,3
8,2,12
4,7,13
15,14,16
5,8,2
15,6,12
12,12,18
6,13,14
7,2,14
12,17,13
4,12,5
12,7,4
5,12,17
14,3,8
7,13,3
12,3,4
6,13,4
11,12,16
13,16,8
8,3,6
12,2,12
15,5,14
7,11,16
6,9,15
6,9,17
5,7,5
13,8,16
12,16,15
5,3,11
5,2,6
14,12,16
5,13,4
8,7,17
14,15,11
6,5,3
16,5,14
15,13,4
4,14,5
3,5,6
14,14,16
8,17,6
16,10,13
3,10,12
2,14,8
13,11,14
17,13,5
15,12,15
9,4,16
15,10,4
10,16,4
12,1,11
13,13,2
17,5,9
9,5,14
18,8,9
9,11,1
7,12,16
5,5,12
7,8,16
15,14,8
4,11,9
2,13,10
8,2,10
3,9,10
6,1,11
17,14,12
5,6,7
14,10,5
10,15,6
13,3,10
6,2,10
3,5,7
7,4,8
9,12,1
16,7,8
3,4,9
6,15,4
4,6,15
12,4,5
13,1,7
16,7,12
12,4,10
5,4,12
12,17,7
3,10,9
14,9,4
15,14,11
12,6,4
6,14,15
10,10,16
13,5,2
18,9,9
8,6,3
6,4,7
16,14,9
3,10,15
5,13,6
11,12,1
8,11,17
10,15,5
2,8,12
2,7,5
16,6,13
7,4,5
6,10,15
2,8,5
6,15,12
8,2,11
5,4,7
9,15,3
16,10,11
4,14,10
4,16,9
9,7,16
7,10,17
2,12,9
6,10,1
2,8,10
15,13,7
16,5,11
16,8,6
4,7,6
16,6,5
4,11,5
1,12,9
11,11,2
6,15,13
17,7,10
12,16,10
17,14,10
12,14,9
3,15,9
9,10,16
17,11,9
4,5,14
12,16,13
11,3,16
12,11,17
9,14,2
15,11,16
5,15,8
9,8,17
8,14,2
13,6,3
4,10,15
16,6,6
3,8,3
10,3,14
10,18,9
10,17,10
9,9,1
9,9,12
2,10,9
4,13,9
7,10,16
16,10,15
16,12,6
12,14,12
3,11,10
8,10,1
5,3,13
12,11,18
7,12,15
12,8,16
12,16,5
16,8,10
15,15,13
11,16,12
6,16,6
17,12,6
10,16,16
4,14,15
8,4,14
15,7,9
11,17,10
15,15,12
12,14,3
16,7,6
12,11,3
11,2,10
2,12,11
16,13,4
2,5,11
6,11,2
13,15,4
10,13,17
17,12,7
6,12,2
10,6,3
8,10,17
17,4,7
2,6,8
9,8,2
16,14,11
5,14,7
11,7,3
0,11,12
1,12,8
5,16,12
12,17,10
5,6,3
16,9,12
10,14,3
10,15,8
1,11,10
7,18,7
8,15,4
6,3,15
8,16,10
15,14,13
3,8,4
8,16,8
16,9,4
15,13,3
7,15,11
13,4,7
7,5,14
14,12,15
9,11,17
12,10,18
15,8,13
8,9,18
14,13,7
4,6,13
3,14,7
10,10,2
12,6,14
15,9,12
15,2,9
14,4,6
9,16,5
10,16,7
18,9,8
12,17,8
11,6,2
8,16,15
3,13,9
6,16,5
14,2,6
7,5,10
13,4,11
9,2,10
5,15,14
6,9,16
4,14,6
15,16,10
3,11,14
2,15,10
11,18,10
4,13,16
11,8,2
9,1,12
6,18,12
10,3,11
2,10,12
11,11,4
15,5,12
9,7,14
7,14,14
12,4,11
8,10,4
12,4,3
15,13,5
9,3,7
6,4,15
13,17,5
4,16,10
10,16,3
2,7,15
1,13,12
17,13,10
11,1,8
14,9,6
14,11,4
11,6,4
16,4,11
6,4,16
9,5,3
12,8,5
7,8,17
14,11,16
14,13,6
13,6,16
5,16,9
12,8,1
11,4,6
7,11,2
4,11,3
8,4,4
11,1,11
5,7,17
7,14,2
9,5,16
17,13,13
15,13,9
8,16,7
13,12,4
13,12,16
14,10,2
15,13,6
13,14,14
7,5,3
9,18,6
15,10,16
15,10,10
10,4,6
8,4,3
8,18,11
5,8,5
7,16,9
4,12,9
15,15,7
3,12,8
16,9,8
14,14,9
13,12,3
8,13,2
12,7,1
9,13,16
12,1,10
18,7,9
6,13,7
5,10,6
5,5,9
16,5,12
8,4,12
5,8,8
14,12,3
3,12,13
12,7,18
3,7,15
15,4,4
3,10,8
5,14,6
13,2,11
10,5,17
10,14,17
16,15,12
4,3,7
2,12,10
8,3,11
14,5,7
13,12,14
10,4,16
8,5,4
9,5,4
15,12,6
3,15,8
5,14,15
9,12,18
8,3,4
9,3,3
7,17,9
17,5,8
7,15,16
17,11,15
1,10,11
18,11,13
4,8,3
8,11,16
9,16,13
11,2,13
9,17,8
5,15,12
9,4,3
6,10,16
12,18,9
7,6,2
8,5,14
5,11,16
6,9,4
10,3,13
5,3,10
13,18,9
1,11,11
12,15,5
6,14,4
12,11,1
1,8,8
4,9,14
14,10,14
12,17,11
2,11,7
15,10,14
5,2,8
3,4,8
13,8,3
10,4,4
7,5,16
6,3,12
10,17,7
15,4,5
16,7,5
13,2,6
1,8,7
12,18,8
5,15,5
7,17,4
15,16,8
7,3,14
11,5,1
7,6,4
2,11,5
3,11,11
7,17,11
5,15,10
6,7,3
12,2,8
6,3,7
12,6,16
6,7,5
19,8,9
6,3,4
6,6,16
16,16,11
15,10,3
16,15,7
2,13,8
7,8,1
4,10,6
12,12,3
3,6,5
4,10,4
9,15,15
11,6,1
3,8,11
18,9,6
6,16,7
5,9,5
13,5,3
17,7,6
10,13,4
14,13,2
14,15,7
4,13,11
14,5,6
16,6,8
8,1,11
11,1,9
10,11,2
14,6,7
9,13,17
11,15,13
3,14,13
1,10,10
4,12,10
11,13,18
17,5,12
3,8,16
6,5,5
18,6,11
11,14,3
10,14,4
9,16,10
8,12,2
17,6,8
15,14,10
12,6,1
7,3,5
7,2,5
11,10,19
5,2,7
7,4,6
8,7,15
9,3,15
4,7,11
5,13,8
6,14,14
1,9,13
14,9,15
9,14,3
12,8,4
1,13,11
16,11,14
9,2,8
11,10,18
9,1,7
11,7,18
12,14,2
13,13,10
8,18,12
11,15,16
5,9,15
18,12,8
3,10,4
15,13,16
4,16,8
9,4,4
19,9,10
11,3,4
14,7,18
7,12,5
7,9,15
9,3,6
16,13,5
14,4,4
5,8,15
5,9,17
12,16,12
13,10,14
11,16,9
10,4,15
6,5,4
9,9,16
17,6,9
2,8,15
6,14,13
5,16,11
8,5,15
8,2,13
14,5,10
10,7,15
2,4,9
12,4,13
9,3,14
17,9,5
6,3,14
3,15,12
14,11,2
11,17,9
8,11,18
15,13,8
10,14,5
4,4,8
4,12,3
10,18,10
18,12,11
11,5,14
5,17,13
9,4,14
11,18,8
8,16,6
17,9,7
17,11,12
14,8,2
3,15,13
7,13,5
1,12,7
12,11,4
6,16,14
14,8,14
14,6,5
8,16,3
5,8,4
8,13,17
9,10,17
11,15,3
7,14,7
12,6,2
3,16,11
16,11,16
16,9,13
11,3,10
17,7,5
10,5,2
18,9,7
9,18,8
15,16,11
1,7,11
15,14,6
15,8,17
6,15,8
14,2,13
9,15,13
14,9,5
5,11,2
16,10,3
12,17,4
10,13,2
15,16,6
5,12,6
13,18,10
5,4,6
8,7,16
16,11,7
16,3,8
3,11,5
6,14,16
15,15,5
10,17,14
3,13,10
18,10,15
7,9,2
5,6,14
8,16,14
5,2,11
3,7,9
13,15,9
7,3,7
11,4,13
2,10,8
15,12,4
5,14,12
18,10,6
3,16,8
10,3,7
2,10,5
11,2,6
4,8,16
18,7,13
11,5,13
3,4,10
9,17,6
1,13,10
10,4,9
15,7,14
7,15,13
7,3,16
10,8,4
3,5,14
1,12,11
18,8,11
5,3,9
10,2,13
12,10,16
3,13,15
16,6,10
7,4,15
10,1,11
10,3,5
10,12,3
9,8,16
5,11,13
16,9,3
6,15,11
7,6,14
12,13,17
13,1,8
8,17,9
14,7,12
3,6,14
7,7,1
16,11,13
8,1,12
18,10,13
2,8,7
14,5,14
14,2,12
6,8,1
14,7,3
3,7,14
10,13,18
11,6,15
12,12,4
8,14,4
13,15,7
14,13,3
3,10,11
8,10,18
17,13,9
13,5,6
13,13,14
1,8,10
6,18,11
15,14,5
1,7,10
13,13,16
15,3,8
6,12,5
6,3,10
14,10,1
10,12,15
4,7,16
9,3,12
6,13,6
5,14,16
13,15,11
4,5,7
19,11,8
3,9,6
16,4,9
13,10,16
15,6,3
11,18,12
7,17,13
9,4,8
9,15,2
5,13,7
12,8,15
9,13,15
12,11,16
1,7,9
16,10,10
10,16,15
14,5,8
5,17,11
17,9,10
4,9,3
17,9,14
12,12,1
14,6,3
9,15,14
1,9,6
1,11,12
12,6,17
18,9,13
15,7,16
3,13,8
11,17,14
3,5,5
17,7,12
6,3,8
8,8,2
2,11,6
11,15,6
9,6,14
14,13,17
9,14,17
17,7,7
4,9,7
7,14,3
5,3,14
0,12,9
5,8,17
4,4,14
5,14,13
10,11,18
15,16,7
12,10,15
4,8,2
7,3,11
14,11,5
2,11,13
1,11,6
9,10,4
15,10,7
11,13,2
10,1,12
10,4,7
13,5,13
6,6,4
6,2,11
17,4,8
9,15,5
14,7,5
8,9,3
14,11,14"
            .lines()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(run_program(lines), 2106);
    }
}
