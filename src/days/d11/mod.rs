mod a;
mod b;

pub struct Day;

impl super::Runner for Day {
    type T = u64;

    fn run(input: &str, basic: bool) -> Self::T {
        if basic {
            a::main(input)
        } else {
            b::main(input)
        }
    }
}
