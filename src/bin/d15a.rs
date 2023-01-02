use std::io::{self, BufRead};

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

fn is_covered(readings: &Vec<SensorReading>, point: Point) -> bool {
    for r in readings {
        if r.sensor.dist(point) <= r.range() {
            return true;
        }
    }
    return false;
}

fn parse_point(s: &str) -> Option<Point> {
    s.split_once(", ").map(|(x_str, y_str)| Point {
        x: x_str[2..].parse::<i32>().unwrap(),
        y: y_str[2..].parse::<i32>().unwrap(),
    })
}

fn parse_line(line: String) -> SensorReading {
    match line[10..].split_once(": closest beacon is at ") {
        Some((sensor_str, beacon_str)) => SensorReading {
            sensor: parse_point(sensor_str).unwrap(),
            beacon: parse_point(beacon_str).unwrap(),
        },
        None => panic!("wrong input format!"),
    }
}

fn main() {
    let mut readings: Vec<SensorReading> = Vec::new();

    for l in io::stdin().lock().lines() {
        let line = l.unwrap();

        if line.len() == 0 {
            break;
        }

        readings.push(parse_line(line));
    }

    let x_min = readings
        .iter()
        .map(|r| r.sensor.x - r.range() as i32)
        .min()
        .unwrap();
    let x_max = readings
        .iter()
        .map(|r| r.sensor.x + r.range() as i32)
        .max()
        .unwrap();

    println!("{}, {}", x_min, x_max);

    let y = 2000000;

    let mut count = 0;
    for x in x_min..=x_max {
        let pt = Point { x, y };
        if is_covered(&readings, pt) {
            if readings
                .iter()
                .find(|r| r.sensor.dist(pt) == 0 || r.beacon.dist(pt) == 0)
                .is_none()
            {
                count += 1;
            }
        }
    }

    println!("Count: {}", count)
}
