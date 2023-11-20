use advent22::days;

fn main() {
    let input: Vec<String> = std::io::stdin().lines().map(|l| l.unwrap()).collect();
    let input = input.iter().map(|s| s.as_str()).collect();

    let result = days::d22::run_program(input, false);

    println!("Score: {}", result);
}