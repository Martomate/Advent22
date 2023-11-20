mod a;
mod b;

pub fn run_program(part2: bool) {
    let result = if part2 {
        b::find_tower_height(b::read_input(), 1000000000000)
    } else {
        a::find_tower_height(a::read_input(), 2022) as u64
    };
    println!("{}", result);
}
