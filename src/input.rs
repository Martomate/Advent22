use std::env;

pub fn run_a_or_b(a: fn() -> (), b: fn() -> ()) {
    let part = env::args().nth(1).expect("Please specify one argument: a or b");
    match part.as_str() {
        "a" => a(),
        "b" => b(),
        _ => panic!("Please specify a or b, not {}", part),
    };
}