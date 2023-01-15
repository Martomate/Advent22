use std::{
    collections::HashMap,
    io::{self, BufRead},
};

struct ParseResult {
    valve_name: String,
    flow_rate: u32,
    edges: Vec<String>,
}

fn parse_line(line: &str) -> Option<ParseResult> {
    let name = &line[6..8];

    let rest1 = &line[23..];
    let semi_idx = rest1.find(';')?;
    let flow = rest1[..semi_idx].parse::<u32>().ok()?;

    let edges_idx = match rest1.find("valves") {
        Some(idx) => idx + 7,
        None => match rest1.find("valve") {
            Some(idx) => idx + 6,
            None => return None,
        },
    };
    let rest2 = &rest1[edges_idx..];
    let edges: Vec<String> = rest2.split(", ").map(|s| s.to_string()).collect();

    Some(ParseResult {
        valve_name: name.to_string(),
        flow_rate: flow,
        edges,
    })
}

struct Node {
    flow_rate: u32,
    edges: Vec<usize>,
}

struct NodeState {
    is_open: bool,
}

fn find_max_volume(
    graph: &Vec<Node>,
    graph_state: &mut Vec<NodeState>,
    node_idx: usize,
    prev_idx: usize,
    time_left: u32,
) -> u32 {
    if time_left == 0 {
        return 0;
    }

    let mut max_volume = 0u32;

    if !graph_state[node_idx].is_open && graph[node_idx].flow_rate != 0 {
        // try to perform an "open valve" action

        graph_state[node_idx].is_open = true;

        let volume_added_here = graph[node_idx].flow_rate * (time_left - 1);
        let volume_added_later =
            find_max_volume(graph, graph_state, node_idx, node_idx, time_left - 1);
        max_volume = volume_added_here + volume_added_later;

        graph_state[node_idx].is_open = false;
    }

    for i in 0..graph[node_idx].edges.len() {
        let next_idx = graph[node_idx].edges[i];
        // try to move to another valve, but don't move back to where you came from

        if next_idx != prev_idx {
            let volume = find_max_volume(graph, graph_state, next_idx, node_idx, time_left - 1);
            if volume > max_volume {
                max_volume = volume;
            }
        }
    }

    max_volume
}

fn main() {
    println!("Hello, world!");

    let mut parsed_lines: Vec<ParseResult> = Vec::new();
    let mut name_table: HashMap<String, usize> = HashMap::new();

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line.len() == 0 {
            break;
        }

        let res = parse_line(line.as_str()).unwrap();
        name_table.insert(res.valve_name.clone(), parsed_lines.len());
        parsed_lines.push(res);
    }

    let mut graph: Vec<Node> = Vec::new();
    let mut graph_state: Vec<NodeState> = Vec::new();

    for r in parsed_lines {
        graph.push(Node {
            flow_rate: r.flow_rate,
            edges: r.edges.iter().map(|name| name_table[name]).collect(),
        });
        graph_state.push(NodeState { is_open: false })
    }

    let start_node_idx = name_table["AA"];

    let result = find_max_volume(&graph, &mut graph_state, start_node_idx, start_node_idx, 30);

    println!("{}", result);
}
