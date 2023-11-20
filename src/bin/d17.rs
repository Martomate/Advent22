use advent22::{days, input};

fn main() {
    input::run_a_or_b(
        || days::d17::run_program(false),
        || days::d17::run_program(true),
    );
}
