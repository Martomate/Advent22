use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum ParsedOp {
    Human,
    Const(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl ParsedOp {
    fn create_two_op(l: &str, r: &str, ops_map: &HashMap<&str, ParsedOp>) -> Option<(Op, Op)> {
        if let (Some(l), Some(r)) = (
            ParsedOp::create_op(l, ops_map),
            ParsedOp::create_op(r, ops_map),
        ) {
            Some((l, r))
        } else {
            None
        }
    }

    fn create_op(name: &str, ops_map: &HashMap<&str, ParsedOp>) -> Option<Op> {
        let op = ops_map.get(name)?;

        match op {
            ParsedOp::Human => Some(Op::Human),
            ParsedOp::Const(c) => Some(Op::Const(*c)),
            ParsedOp::Add(l, r) => ParsedOp::create_two_op(l, r, ops_map)
                .map(|(l, r)| Op::Add(Box::new(l), Box::new(r))),
            ParsedOp::Sub(l, r) => ParsedOp::create_two_op(l, r, ops_map)
                .map(|(l, r)| Op::Sub(Box::new(l), Box::new(r))),
            ParsedOp::Mul(l, r) => ParsedOp::create_two_op(l, r, ops_map)
                .map(|(l, r)| Op::Mul(Box::new(l), Box::new(r))),
            ParsedOp::Div(l, r) => ParsedOp::create_two_op(l, r, ops_map)
                .map(|(l, r)| Op::Div(Box::new(l), Box::new(r))),
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Human,
    Const(i64),
    Add(Box<Op>, Box<Op>),
    Sub(Box<Op>, Box<Op>),
    Mul(Box<Op>, Box<Op>),
    Div(Box<Op>, Box<Op>),
}

impl Op {
    fn evaluate(&self, human: i64) -> i64 {
        use Op::*;

        match self {
            Human => human,
            Const(c) => *c,
            Add(l, r) => l.evaluate(human) + r.evaluate(human),
            Sub(l, r) => l.evaluate(human) - r.evaluate(human),
            Mul(l, r) => l.evaluate(human) * r.evaluate(human),
            Div(l, r) => l.evaluate(human) / r.evaluate(human),
        }
    }

    fn simplify(&self) -> Op {
        use Op::*;

        match self {
            Human => Human,
            Const(c) => Const(*c),
            Add(l, r) => 
                match (l.simplify(), r.simplify()) {
                    (Const(l), Const(r)) => Const(l + r),
                    (l, r) => Add(Box::new(l), Box::new(r)),
                },
            Sub(l, r) => 
                match (l.simplify(), r.simplify()) {
                    (Const(l), Const(r)) => Const(l - r),
                    (l, r) => Sub(Box::new(l), Box::new(r)),
                },
            Mul(l, r) => 
                match (l.simplify(), r.simplify()) {
                    (Const(l), Const(r)) => Const(l * r),
                    (l, r) => Mul(Box::new(l), Box::new(r)),
                },
            Div(l, r) => 
                match (l.simplify(), r.simplify()) {
                    (Const(l), Const(r)) => Const(l / r),
                    (l, r) => Div(Box::new(l), Box::new(r)),
                },
        }
    }

    fn extract_human_if_eq(&self, value: Op) -> Option<Op> {
        use Op::*;

        let op = match self {
            Human => Some(value),
            Const(_) => None,
            Add(l, r) => match (l.as_ref(), r.as_ref()) {
                (Const(c), r) => r.extract_human_if_eq(Sub(Box::new(value), Box::new(Const(*c)))),
                (l, Const(c)) => l.extract_human_if_eq(Sub(Box::new(value), Box::new(Const(*c)))),
                _ => None,
            }
            Sub(l, r) => match (l.as_ref(), r.as_ref()) {
                (Const(c), r) => r.extract_human_if_eq(Sub(Box::new(Const(*c)), Box::new(value))),
                (l, Const(c)) => l.extract_human_if_eq(Add(Box::new(Const(*c)), Box::new(value))),
                _ => None,
            }
            Mul(l, r) => match (l.as_ref(), r.as_ref()) {
                (Const(c), r) => r.extract_human_if_eq(Div(Box::new(value), Box::new(Const(*c)))),
                (l, Const(c)) => l.extract_human_if_eq(Div(Box::new(value), Box::new(Const(*c)))),
                _ => None,
            }
            Div(l, r) => match (l.as_ref(), r.as_ref()) {
                (Const(c), r) => r.extract_human_if_eq(Div(Box::new(Const(*c)), Box::new(value))),
                (l, Const(c)) => l.extract_human_if_eq(Mul(Box::new(Const(*c)), Box::new(value))),
                _ => None,
            }
        };

        op
    }

    fn find_human_value_if_zero(&self) -> Option<i64> {
        let op = self.simplify().extract_human_if_eq(Op::Const(0));

        op.map(|h| h.evaluate(0))
    }
}

fn parse_input(input: &str) -> HashMap<&str, ParsedOp> {
    let lines: Vec<_> = input
        .split('\n')
        .map(|line| line.split_once(": ").unwrap())
        .collect();
    let mut named_ops: HashMap<&str, ParsedOp> = HashMap::new();
    for &(name, v) in lines.iter() {
        let parsed_op = match v.parse::<i64>() {
            Ok(c) => ParsedOp::Const(c),
            _ => {
                let parts: Vec<_> = v.split(' ').collect();
                match parts[1] {
                    "+" => ParsedOp::Add(parts[0].to_string(), parts[2].to_string()),
                    "-" => ParsedOp::Sub(parts[0].to_string(), parts[2].to_string()),
                    "*" => ParsedOp::Mul(parts[0].to_string(), parts[2].to_string()),
                    "/" => ParsedOp::Div(parts[0].to_string(), parts[2].to_string()),
                    _ => panic!("unsupported operator: {}", parts[1]),
                }
            }
        };
        named_ops.insert(name, parsed_op);
    }

    named_ops
}

fn run_program(part2: bool, big: bool) -> i64 {
    let input = load_example(big);
    let mut ops_map = parse_input(input);
    if part2 {
        ops_map.insert("humn", ParsedOp::Human);
        ops_map.insert("root", match ops_map.get("root").unwrap() {
            ParsedOp::Add(l, r) => ParsedOp::Sub(l.clone(), r.clone()),
            ParsedOp::Sub(l, r) => ParsedOp::Sub(l.clone(), r.clone()),
            ParsedOp::Mul(l, r) => ParsedOp::Sub(l.clone(), r.clone()),
            ParsedOp::Div(l, r) => ParsedOp::Sub(l.clone(), r.clone()),
            _ => panic!("root does not have a binary operation"),
        });
        let op = ParsedOp::create_op("root", &ops_map).unwrap();
        op.find_human_value_if_zero().unwrap()
    } else {
        let op = ParsedOp::create_op("root", &ops_map).unwrap();
        if let Some(ParsedOp::Const(c)) = ops_map.get("humn") {
            op.evaluate(*c)
        } else {
            panic!("the human was not a constant!");
        }
    }
}

fn load_example(big: bool) -> &'static str {
    if big {
        include_str!("d21_ex_2.txt")
    } else {
        include_str!("d21_ex_1.txt")
    }
}

fn main() {
    println!("Result: {}", run_program(false, false));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_input, ParsedOp, run_program};

    #[test]
    fn parse_input_works_for_single_lines() {
        assert_eq!(parse_input("root: 123"), HashMap::from([("root", ParsedOp::Const(123))]));
        assert_eq!(parse_input("root: abcd + efgh"), HashMap::from([("root", ParsedOp::Add("abcd".to_string(), "efgh".to_string()))]));
        assert_eq!(parse_input("root: abcd - efgh"), HashMap::from([("root", ParsedOp::Sub("abcd".to_string(), "efgh".to_string()))]));
        assert_eq!(parse_input("root: abcd * efgh"), HashMap::from([("root", ParsedOp::Mul("abcd".to_string(), "efgh".to_string()))]));
        assert_eq!(parse_input("root: abcd / efgh"), HashMap::from([("root", ParsedOp::Div("abcd".to_string(), "efgh".to_string()))]));
    }

    #[test]
    fn parse_input_works_for_multiple_lines() {
        assert_eq!(parse_input("a: 123\nb: a + a\nc: a * b"), HashMap::from([
            ("a", ParsedOp::Const(123)),
            ("b", ParsedOp::Add("a".to_string(), "a".to_string())),
            ("c", ParsedOp::Mul("a".to_string(), "b".to_string()))
        ]));
    }

    #[test]
    fn small_example_works_for_part_1() {
        assert_eq!(run_program(false, false), 152);
    }

    #[test]
    fn big_example_works_for_part_1() {
        assert_eq!(run_program(false, true), 194501589693264);
    }

    #[test]
    fn small_example_works_for_part_2() {
        assert_eq!(run_program(true, false), 301);
    }

    #[test]
    fn big_example_works_for_part_2() {
        assert_eq!(run_program(true, true), 3887609741189);
    }
}
