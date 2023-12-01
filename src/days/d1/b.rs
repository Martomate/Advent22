pub fn main(input: &str) -> i32 {
    println!("Hello, world!");

    let mut sums: Vec<i32> = Vec::new();
    let mut latest: i32 = 0;

    for l in input.lines() {
        if !l.is_empty() {
            let c = l.parse::<i32>().unwrap();
            latest += c;
        } else {
            sums.push(latest);
            latest = 0;
        }
    }
    sums.push(latest);

    sums.sort();

    sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3]
}
