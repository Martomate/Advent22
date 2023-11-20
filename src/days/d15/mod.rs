mod a;
mod b;

pub fn run(part1: bool) {
    if part1 {
        a::main();
    } else {
        b::main();
    }
}