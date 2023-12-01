pub fn main(input: &str) -> i32 {
    println!("Hello, world!");

    let mut latest: i32 = 0;
    let mut max: i32 = 0;

    for l in input.lines() {
        if !l.is_empty() {
            let c = l.parse::<i32>().unwrap();
            latest += c;
        } else {
            if latest > max {
                max = latest;
            }
            latest = 0;
        }
    }
    if latest > max {
        max = latest;
    }

    max
}
