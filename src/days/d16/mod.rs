mod a;
mod b;

pub fn run_program(part2: bool) {
    let result = if part2 {
        b::find_max_volume_for_input(b::read_input())
    } else {
        a::find_max_volume_for_input(a::read_input())
    };
    println!("{}", result);
}
