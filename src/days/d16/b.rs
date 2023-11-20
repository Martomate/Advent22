use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use bit_set::BitSet;

#[derive(Debug, PartialEq)]
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

struct Valve {
    flow_rate: u32,
    edges: Vec<usize>,
}

struct GraphState {
    valves_open: BitSet<u64>,
    flow_left: u32,
    max_volume_so_far: u32,
    memoized_results: HashMap<(BitSet<u64>, usize, usize, u32), u32>,
}

/** My goodness! This is a disgrace! */
fn find_max_volume(
    graph: &[Valve],
    graph_state: &mut GraphState,
    node_idx: (usize, usize),
    prev_idx: (usize, usize),
    volume_so_far: u32,
    time_left: u32,
) -> u32 {
    if time_left == 0 {
        if volume_so_far > graph_state.max_volume_so_far {
            graph_state.max_volume_so_far = volume_so_far;
        }
        return 0;
    }
    if volume_so_far + graph_state.flow_left * (time_left - 1) < graph_state.max_volume_so_far {
        return 0; // this cannot possibly be the best branch, so just give up
    }

    let (node1_idx, node2_idx) = node_idx;
    let (prev1_idx, prev2_idx) = prev_idx;

    let memo_key = (
        graph_state.valves_open.clone(),
        node1_idx.min(node2_idx),
        node1_idx.max(node2_idx),
        time_left,
    );
    if let Some(result) = graph_state.memoized_results.get(&memo_key) {
        return *result;
    }

    let mut max_volume = 0u32;

    // both open
    if node1_idx != node2_idx
        && !graph_state.valves_open.contains(node1_idx)
        && !graph_state.valves_open.contains(node2_idx)
        && graph[node1_idx].flow_rate != 0
        && graph[node2_idx].flow_rate != 0
    {
        // try to perform an "open valve" action

        let flow = graph[node1_idx].flow_rate + graph[node2_idx].flow_rate;

        graph_state.valves_open.insert(node1_idx);
        graph_state.valves_open.insert(node2_idx);
        graph_state.flow_left -= flow;

        let volume_added_here = flow * (time_left - 1);
        let volume = volume_added_here
            + find_max_volume(
                graph,
                graph_state,
                (node1_idx, node2_idx),
                (node1_idx, node2_idx),
                volume_so_far + volume_added_here,
                time_left - 1,
            );
        if volume > max_volume {
            max_volume = volume;
        }

        graph_state.flow_left += flow;
        graph_state.valves_open.remove(node2_idx);
        graph_state.valves_open.remove(node1_idx);
    }

    // 1 opens, 2 walks
    if !graph_state.valves_open.contains(node1_idx) && graph[node1_idx].flow_rate != 0 {
        // try to perform an "open valve" action

        let flow = graph[node1_idx].flow_rate;
        graph_state.valves_open.insert(node1_idx);
        graph_state.flow_left -= flow;

        let volume_added_here = flow * (time_left - 1);

        for i in 0..graph[node2_idx].edges.len() {
            let next_idx = graph[node2_idx].edges[i];
            // try to move to another valve, but don't move back to where you came from

            if next_idx != prev2_idx {
                let volume = volume_added_here
                    + find_max_volume(
                        graph,
                        graph_state,
                        (node1_idx, next_idx),
                        (node1_idx, node2_idx),
                        volume_so_far + volume_added_here,
                        time_left - 1,
                    );
                if volume > max_volume {
                    max_volume = volume;
                }
            }
        }

        graph_state.flow_left += flow;
        graph_state.valves_open.remove(node1_idx);
    }

    // 2 opens, 1 walks
    if !graph_state.valves_open.contains(node2_idx) && graph[node2_idx].flow_rate != 0 {
        // try to perform an "open valve" action

        let flow = graph[node2_idx].flow_rate;
        graph_state.valves_open.insert(node2_idx);
        graph_state.flow_left -= flow;

        let volume_added_here = flow * (time_left - 1);

        for i in 0..graph[node1_idx].edges.len() {
            let next_idx = graph[node1_idx].edges[i];
            // try to move to another valve, but don't move back to where you came from

            if next_idx != prev1_idx {
                let volume = volume_added_here
                    + find_max_volume(
                        graph,
                        graph_state,
                        (next_idx, node2_idx),
                        (node1_idx, node2_idx),
                        volume_so_far + volume_added_here,
                        time_left - 1,
                    );
                if volume > max_volume {
                    max_volume = volume;
                }
            }
        }

        graph_state.flow_left += flow;
        graph_state.valves_open.remove(node2_idx);
    }

    // both walk
    for i in 0..graph[node1_idx].edges.len() {
        for j in 0..graph[node2_idx].edges.len() {
            let next1_idx = graph[node1_idx].edges[i];
            let next2_idx = graph[node2_idx].edges[j];

            // try to move to another valve, but don't move back to where you came from
            if next1_idx != prev1_idx && next2_idx != prev2_idx {
                let volume = find_max_volume(
                    graph,
                    graph_state,
                    (next1_idx, next2_idx),
                    (node1_idx, node2_idx),
                    volume_so_far,
                    time_left - 1,
                );
                if volume > max_volume {
                    max_volume = volume;
                }
            }
        }
    }

    graph_state.memoized_results.insert(memo_key, max_volume);

    max_volume
}

pub fn find_max_volume_for_input(input: Vec<String>) -> u32 {
    let mut parsed_lines: Vec<ParseResult> = Vec::new();
    let mut name_table: HashMap<String, usize> = HashMap::new();

    for line in input {
        let res = parse_line(line.as_str()).unwrap();
        name_table.insert(res.valve_name.clone(), parsed_lines.len());
        parsed_lines.push(res);
    }
    let mut graph: Vec<Valve> = Vec::new();
    let mut graph_state: GraphState = GraphState {
        valves_open: BitSet::default(),
        flow_left: 0,
        max_volume_so_far: 0,
        memoized_results: HashMap::new(),
    };
    for r in parsed_lines {
        graph.push(Valve {
            flow_rate: r.flow_rate,
            edges: r.edges.iter().map(|name| name_table[name]).collect(),
        });
    }
    let start_node_idx = name_table["AA"];
    graph_state.flow_left = graph.iter().map(|v| v.flow_rate).sum();

    find_max_volume(
        &graph,
        &mut graph_state,
        (start_node_idx, start_node_idx),
        (start_node_idx, start_node_idx),
        0,
        26,
    )
}

pub fn read_input() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::{find_max_volume_for_input, parse_line, ParseResult};

    #[test]
    fn parse_line_works_for_single_edge() {
        let result = parse_line("Valve AA has flow rate=42; tunnel leads to valve DD");
        assert_eq!(
            result,
            Some(ParseResult {
                valve_name: "AA".to_string(),
                flow_rate: 42,
                edges: vec!["DD".to_string()]
            })
        );
    }

    #[test]
    fn parse_line_works_for_single_digit() {
        let result = parse_line("Valve AA has flow rate=4; tunnel leads to valve DD");
        assert_eq!(
            result,
            Some(ParseResult {
                valve_name: "AA".to_string(),
                flow_rate: 4,
                edges: vec!["DD".to_string()]
            })
        );
    }

    #[test]
    fn parse_line_works_for_multiple_edges() {
        let result = parse_line("Valve AA has flow rate=42; tunnels lead to valves DD, II, BB");
        assert_eq!(
            result,
            Some(ParseResult {
                valve_name: "AA".to_string(),
                flow_rate: 42,
                edges: vec!["DD".to_string(), "II".to_string(), "BB".to_string()]
            })
        );
    }

    #[test]
    fn example_works() {
        let input: Vec<String> = include_str!("ex1.txt")
            .lines()
            .map(|l| l.to_string())
            .collect();

        let result = find_max_volume_for_input(input);

        assert_eq!(result, 1707);
    }

    #[test]
    fn big_example_works() {
        let input: Vec<String> = include_str!("ex2.txt")
            .lines()
            .map(|l| l.to_string())
            .collect();

        let result = find_max_volume_for_input(input);

        assert_eq!(result, 2602);
    }
}
