mod a;
mod b;

pub fn run_program(part2: bool) {
    let result = if part2 {
        b::main()
    } else {
        a::main()
    };
    println!("{}", result);
}
