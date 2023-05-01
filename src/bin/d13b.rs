use std::{
    cmp::Ordering,
    fmt::Display,
    io::{self, BufRead},
    iter::zip,
    str::Chars,
};

#[derive(PartialEq, Eq)]
enum Node {
    Int(u32),
    List(Vec<Node>),
}

fn compare(left: &Node, right: &Node) -> Ordering {
    match (left, right) {
        (Node::Int(l), Node::Int(r)) => l.cmp(r),
        (Node::Int(num), _) => compare(&Node::List(vec![Node::Int(*num)]), right),
        (_, Node::Int(num)) => compare(left, &Node::List(vec![Node::Int(*num)])),
        (Node::List(ls), Node::List(rs)) => {
            let llen = ls.len();
            let rlen = rs.len();

            match zip(ls, rs).find(|(l, r)| compare(l, r).is_ne()) {
                Some((l, r)) => compare(l, r),
                None => llen.cmp(&rlen),
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Int(num) => write!(f, "{}", num),
            Node::List(elems) => write!(
                f,
                "[{}]",
                elems
                    .iter()
                    .map(|e| (*e).to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

fn parse_list(line: &mut Chars) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    while let (Some(node), last_char) = parse_node(line) {
        nodes.push(node);

        if let Some(last) = last_char {
            match last {
                ',' => continue,
                ']' => break,
                _ => panic!("What??"),
            }
        }
    }
    nodes
}

fn is_digit(c: char) -> bool {
    c as u8 >= b'0' && c as u8 <= b'9'
}

fn parse_node(line: &mut Chars) -> (Option<Node>, Option<char>) {
    if let Some(first_char) = line.next() {
        if first_char == '[' {
            return (Some(Node::List(parse_list(line))), line.next());
        } else if is_digit(first_char) {
            let mut number_so_far: u32 = 0;
            let mut ch: Option<char> = Some(first_char);
            while let Some(c) = ch {
                if is_digit(c) {
                    number_so_far *= 10;
                    number_so_far += (c as u32) - ('0' as u32);
                } else if c == ',' || c == ']' {
                    return (Some(Node::Int(number_so_far)), ch);
                }
                ch = line.next();
            }
            return (None, None);
        }
    }

    (None, None)
}

fn main() {
    println!("Hello, world!");

    let mut nodes: Vec<Node> = Vec::new();

    let mut last_line_empty = true;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();

        if l.is_empty() {
            if last_line_empty {
                break;
            }
            last_line_empty = true;
        } else {
            last_line_empty = false;
            nodes.push(parse_node(&mut l.chars()).0.unwrap());
        }
    }

    nodes.push(Node::List(vec![Node::List(vec![Node::Int(2)])]));
    nodes.push(Node::List(vec![Node::List(vec![Node::Int(6)])]));

    nodes.sort_by(compare);

    let mut idx2 = 0;
    let mut idx6 = 0;

    for (i, node) in nodes.iter().enumerate() {
        if *node == Node::List(vec![Node::List(vec![Node::Int(2)])]) {
            idx2 = i;
        }

        if *node == Node::List(vec![Node::List(vec![Node::Int(6)])]) {
            idx6 = i;
        }
    }

    println!("2: {}", idx2);
    println!("6: {}", idx6);
    println!("res: {}", (idx2 + 1) * (idx6 + 1));
}
