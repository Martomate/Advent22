use std::fmt::Display;

enum Instr {
    Noop,
    Addx(i32),
}

pub struct Crt {
    pixels: [[bool; 40]; 6],
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = String::with_capacity(41 * 6);
        for y in 0..6 {
            for x in 0..40 {
                let c = if self.pixels[y][x] { '#' } else { '.' };
                s.push(c);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

fn make_crt(xs: Vec<i32>) -> Crt {
    let mut crt: Crt = Crt {
        pixels: [[false; 40]; 6],
    };

    for (i, x) in xs.iter().enumerate() {
        let on = x.abs_diff(i as i32 % 40) <= 1;
        crt.pixels[i / 40][i % 40] = on;
    }

    crt
}

fn run_program(program: &Vec<Instr>) -> Crt {
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

    make_crt(x_history)
}

pub fn main(input: &str) -> Crt {
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
