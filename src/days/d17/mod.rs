mod a;
mod b;

pub struct Day;

impl super::Runner for Day {
    type T = u64;

    fn run(input: &str, basic: bool) -> Self::T {
        if basic {
            a::find_tower_height(a::read_input(input), 2022) as u64
        } else {
            b::find_tower_height(b::read_input(input), 1000000000000)
        }
    }
}
