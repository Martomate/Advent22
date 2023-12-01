use std::collections::VecDeque;

struct Ship {
    stacks: Vec<VecDeque<char>>,
}

struct MoveInstr {
    from: u32,
    to: u32,
    count: u32,
}

impl Ship {
    fn move_crates(&mut self, instr: MoveInstr) {
        for _ in 0..instr.count {
            let c = self.stacks[instr.from as usize].pop_back().unwrap();
            self.stacks[instr.to as usize].push_back(c);
        }
    }
}

fn parse_ship(lines: &[&str]) -> Ship {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for line in lines.iter().rev() {
        if !line.starts_with(" 1") {
            let row: Vec<char> = line.chars().skip(1).step_by(4).collect();

            for i in 0..(row.len()) {
                if i == stacks.len() {
                    stacks.push(VecDeque::new());
                }
                if row[i] != ' ' {
                    stacks[i].push_back(row[i]);
                }
            }
        }
    }
    Ship { stacks }
}

fn parse_move_instr(line: &str) -> Option<MoveInstr> {
    let parts: Vec<_> = line.split(' ').collect();
    match (
        parts[1].parse::<u32>().ok(),
        parts[3].parse::<u32>().ok(),
        parts[5].parse::<u32>().ok(),
    ) {
        (Some(count), Some(from), Some(to)) => Some(MoveInstr {
            from: from - 1,
            to: to - 1,
            count,
        }),
        _ => None,
    }
}

pub fn main(input: &str) -> String {
    println!("Hello, world!");

    let initial_ship_lines: Vec<_> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .collect();

    let mut ship = parse_ship(&initial_ship_lines);

    for line in input
        .lines()
        .skip(initial_ship_lines.len())
        .take_while(|l| !l.is_empty())
    {
        let instr = parse_move_instr(line).unwrap();
        ship.move_crates(instr);
    }

    let result = ship
        .stacks
        .iter()
        .map(|s| s.back().unwrap())
        .collect::<String>();

    result
}
