mod a;
mod b;

pub struct Day;

impl super::Runner for Day {
    type T = i64;

    fn run(input: &str, basic: bool) -> Self::T {
        if basic {
            a::main(input)
        } else {
            b::main(input)
        }
    }
}
