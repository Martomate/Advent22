use num::integer::lcm;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum RightOperand {
    Num(i32),
    Old,
}

#[derive(Clone, Copy)]
enum Operation {
    Add(RightOperand),
    Mul(RightOperand),
}

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    divisibility: i32,
    yes_dest: usize,
    no_dest: usize,
    inspections: u32,
}

fn parse_monkey(lines: &[String]) -> Monkey {
    let items: Vec<i32> = lines[1][(lines[1].find(':').unwrap() + 2)..]
        .split(", ")
        .map(|a| a.parse::<i32>().unwrap())
        .collect();

    let operation_str_parts: Vec<&str> = lines[2][(lines[2].find(':').unwrap() + 2)..]
        .split(' ')
        .collect();
    let right_operand = match operation_str_parts[4] {
        "old" => RightOperand::Old,
        s => RightOperand::Num(s.parse::<i32>().unwrap()),
    };
    let operation: Operation = match operation_str_parts[3] {
        "+" => Operation::Add(right_operand),
        "*" => Operation::Mul(right_operand),
        _ => panic!("unsupported operation"),
    };

    let divisibility = lines[3][(lines[3].find("by").unwrap() + 3)..]
        .parse::<i32>()
        .unwrap();

    let yes_dest = lines[4][(lines[4].find("monkey").unwrap() + 7)..]
        .parse::<usize>()
        .unwrap();

    let no_dest = lines[5][(lines[5].find("monkey").unwrap() + 7)..]
        .parse::<usize>()
        .unwrap();

    Monkey {
        items,
        operation,
        divisibility,
        yes_dest,
        no_dest,
        inspections: 0,
    }
}

fn evaluate_operation(old: i32, operation: Operation) -> i64 {
    match operation {
        Operation::Add(right) => match right {
            RightOperand::Num(num) => (old + num) as i64,
            RightOperand::Old => (old + old) as i64,
        },
        Operation::Mul(right) => match right {
            RightOperand::Num(num) => old as i64 * num as i64,
            RightOperand::Old => old as i64 * old as i64,
        },
    }
}

fn simulate_monkey_round(monkeys: &mut Vec<Monkey>, total_mod: i64) {
    for i in 0..monkeys.len() {
        let q = monkeys[i].items.clone();
        monkeys[i].items.clear();

        monkeys[i].inspections += q.len() as u32;

        for item in q {
            let new_item = (evaluate_operation(item, monkeys[i].operation) % total_mod) as i32;
            if new_item % monkeys[i].divisibility == 0 {
                let dest = monkeys[i].yes_dest;
                monkeys[dest].items.push(new_item);
            } else {
                let dest = monkeys[i].no_dest;
                monkeys[dest].items.push(new_item);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut last_monkey_lines: Vec<String> = Vec::new();

    let mut monkeys: Vec<Monkey> = Vec::new();

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line.is_empty() {
            if last_monkey_lines.is_empty() {
                break;
            } else {
                monkeys.push(parse_monkey(&last_monkey_lines));
                last_monkey_lines.clear();
            }
        } else {
            last_monkey_lines.push(line);
        }
    }

    let mut total_mod: i32 = 1;
    for m in monkeys.iter() {
        total_mod = lcm(total_mod, m.divisibility);
    }

    println!("mod: {}", total_mod);

    for _ in 0..10000 {
        simulate_monkey_round(&mut monkeys, total_mod as i64);
    }

    monkeys.sort_by(|l, r| l.inspections.cmp(&r.inspections));

    let next_busiest = monkeys[monkeys.len() - 2].inspections;
    let busiest = monkeys[monkeys.len() - 1].inspections;

    let result = next_busiest as u64 * busiest as u64;

    println!("{}", result);
}
