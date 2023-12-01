enum Instr {
    Noop,
    Addx(i32),
}

fn run_program(program: &Vec<Instr>) -> i32 {
    let mut x = 1;

    let mut x_history: Vec<i32> = Vec::new();

    for instr in program {
        match instr {
            Instr::Noop => {
                x_history.push(x);
            }
            Instr::Addx(v) => {
                x_history.push(x);
                x_history.push(x);
                x += v;
            }
        };
    }

    let mut result = 0;

    for cycle in (20..=x_history.len()).step_by(40) {
        result += (cycle as i32) * x_history[cycle - 1];
    }

    result
}

pub fn main(input: &str) -> i32 {
    println!("Hello, world!");

    let mut program: Vec<Instr> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split(' ').collect();

        if parts[0] == "noop" {
            program.push(Instr::Noop);
        } else if parts[0] == "addx" {
            let v = parts[1].parse::<i32>().unwrap();
            program.push(Instr::Addx(v));
        } else {
            panic!("wat?")
        }
    }

    run_program(&program)
}
