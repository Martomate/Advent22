use std::{
    fmt::Display,
    iter::zip,
    str::Chars,
};

enum Node {
    Int(u32),
    List(Vec<Node>),
}

fn compare(left: &Node, right: &Node) -> i32 {
    match (left, right) {
        (Node::Int(l), Node::Int(r)) => *l as i32 - *r as i32,
        (Node::Int(num), _) => compare(&Node::List(vec![Node::Int(*num)]), right),
        (_, Node::Int(num)) => compare(left, &Node::List(vec![Node::Int(*num)])),
        (Node::List(ls), Node::List(rs)) => {
            let llen = ls.len();
            let rlen = rs.len();

            match zip(ls, rs).find(|(l, r)| compare(l, r) != 0) {
                Some((l, r)) => compare(l, r),
                None => llen as i32 - rlen as i32,
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

pub fn main(input: &str) -> usize {
    println!("Hello, world!");

    let mut pairs: Vec<(Node, Node)> = Vec::new();

    let mut nodes: Vec<String> = Vec::new();

    for l in input.lines() {
        if l.is_empty() {
            if nodes.is_empty() {
                break;
            } else {
                let ln = parse_node(&mut nodes[0].chars()).0.unwrap();
                let rn = parse_node(&mut nodes[1].chars()).0.unwrap();
                pairs.push((ln, rn));
                nodes.clear();
            }
        } else {
            nodes.push(l.to_string());
        }
    }

    if nodes.len() > 1 {
        let ln = parse_node(&mut nodes[0].chars()).0.unwrap();
        let rn = parse_node(&mut nodes[1].chars()).0.unwrap();
        pairs.push((ln, rn));
    }

    let mut total = 0;

    for (i, (l, r)) in pairs.iter().enumerate() {
        if compare(l, r) < 0 {
            total += i + 1;
        }
    }

    total
}
