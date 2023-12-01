#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(&self, other: Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

struct SensorReading {
    sensor: Point,
    beacon: Point,
}

impl SensorReading {
    fn range(&self) -> u32 {
        self.sensor.dist(self.beacon)
    }
}

fn parse_point(s: &str) -> Option<Point> {
    s.split_once(", ").map(|(x_str, y_str)| Point {
        x: x_str[2..].parse::<i32>().unwrap(),
        y: y_str[2..].parse::<i32>().unwrap(),
    })
}

fn parse_line(line: &str) -> SensorReading {
    match line[10..].split_once(": closest beacon is at ") {
        Some((sensor_str, beacon_str)) => SensorReading {
            sensor: parse_point(sensor_str).unwrap(),
            beacon: parse_point(beacon_str).unwrap(),
        },
        None => panic!("wrong input format!"),
    }
}

fn find_hole(readings: &[SensorReading], search_width: i32) -> Option<Point> {
    for y in 0..=4000000 {
        let mut xs = readings
            .iter()
            .map(|r| {
                let dx = r.range() as i32 - r.sensor.y.abs_diff(y) as i32;
                let x_min = r.sensor.x - dx;
                let x_max = r.sensor.x + dx;
                (x_min, x_max)
            })
            .collect::<Vec<_>>();
        xs.sort_by_key(|xm| xm.0);

        let mut max = 0;

        for (x_min, x_max) in xs {
            if x_min > max + 1 {
                return Some(Point { x: max + 1, y });
            }
            max = max.max(x_max);

            if max >= search_width {
                break;
            }
        }
    }
    None
}

pub fn main(input: &str) -> i64 {
    let mut readings: Vec<SensorReading> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        readings.push(parse_line(line));
    }

    let hole = find_hole(&readings, if readings.len() > 14 { 4000000 } else { 20 }).unwrap();

    hole.x as i64 * 4000000 + hole.y as i64
}
