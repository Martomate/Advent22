mod a;
mod b;

pub struct Day;

impl super::Runner for Day {
    type T = u32;

    fn run(input: &str, basic: bool) -> Self::T {
        if basic {
            a::find_max_volume_for_input(a::read_input(input))
        } else {
            b::find_max_volume_for_input(b::read_input(input))
        }
    }
}
