use std::{
    fmt::Display,
    io::{self, BufRead},
};

enum Instr {
    Noop,
    Addx(i32),
}

struct CRT {
    pixels: [[bool; 40]; 6],
}

impl Display for CRT {
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

fn make_crt(xs: Vec<i32>) -> CRT {
    let mut crt: CRT = CRT {
        pixels: [[false; 40]; 6],
    };

    for i in 0..xs.len() {
        let on = xs[i].abs_diff(i as i32 % 40) <= 1;
        crt.pixels[i / 40][i % 40] = on;
    }

    crt
}

fn run_program(program: &Vec<Instr>) -> CRT {
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

fn main() {
    println!("Hello, world!");

    let mut program: Vec<Instr> = Vec::new();

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line.len() == 0 {
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

    let result = run_program(&program);

    println!("{}", result);
}
