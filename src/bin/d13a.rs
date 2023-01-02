use std::{
    fmt::Display,
    io::{self, BufRead},
    iter::zip,
    str::Chars,
};

enum Node {
    Int(u32),
    List(Vec<Box<Node>>),
}

fn compare(left: &Node, right: &Node) -> i32 {
    match (left, right) {
        (Node::Int(l), Node::Int(r)) => *l as i32 - *r as i32,
        (Node::Int(num), _) => compare(&Node::List(vec![Box::from(Node::Int(*num))]), right),
        (_, Node::Int(num)) => compare(left, &Node::List(vec![Box::from(Node::Int(*num))])),
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

fn parse_list(line: &mut Chars) -> Vec<Box<Node>> {
    let mut nodes: Vec<Box<Node>> = Vec::new();
    loop {
        match parse_node(line) {
            (Some(node), last_char) => {
                nodes.push(Box::from(node));

                if let Some(last) = last_char {
                    match last {
                        ',' => continue,
                        ']' => break,
                        _ => panic!("What??"),
                    }
                }
            }
            (None, _) => break,
        }
    }
    return nodes;
}

fn is_digit(c: char) -> bool {
    return c as u8 >= '0' as u8 && c as u8 <= '9' as u8;
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

    return (None, None);
}

fn main() {
    println!("Hello, world!");

    let mut pairs: Vec<(Node, Node)> = Vec::new();

    let mut nodes: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();

        if l.len() == 0 {
            if nodes.len() == 0 {
                break;
            } else {
                let ln = parse_node(&mut nodes[0].chars()).0.unwrap();
                let rn = parse_node(&mut nodes[1].chars()).0.unwrap();
                pairs.push((ln, rn));
                nodes.clear();
            }
        } else {
            nodes.push(l);
        }
    }

    let mut total = 0;

    for i in 0..pairs.len() {
        let p = &pairs[i];
        if compare(&p.0, &p.1) < 0 {
            total += i + 1;
        }
    }

    println!("{}", total);
}
