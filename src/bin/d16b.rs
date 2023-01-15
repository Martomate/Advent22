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

struct Valve {
    flow_rate: u32,
    edges: Vec<usize>,
}

struct ValveState {
    is_open: bool,
}

struct GraphState {
    valve_states: Vec<ValveState>,
    flow_left: u32,
    max_volume_so_far: u32,
}

/** My goodness! This is a disgrace! */
fn find_max_volume(
    graph: &Vec<Valve>,
    graph_state: &mut GraphState,
    node1_idx: usize,
    prev1_idx: usize,
    node2_idx: usize,
    prev2_idx: usize,
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

    let mut max_volume = 0u32;

    // both open
    if node1_idx != node2_idx
        && !graph_state.valve_states[node1_idx].is_open
        && !graph_state.valve_states[node2_idx].is_open
        && graph[node1_idx].flow_rate != 0
        && graph[node2_idx].flow_rate != 0
    {
        // try to perform an "open valve" action

        let flow = graph[node1_idx].flow_rate + graph[node2_idx].flow_rate;

        graph_state.valve_states[node1_idx].is_open = true;
        graph_state.valve_states[node2_idx].is_open = true;
        graph_state.flow_left -= flow;

        let volume_added_here = flow * (time_left - 1);
        let volume = volume_added_here
            + find_max_volume(
                graph,
                graph_state,
                node1_idx,
                node1_idx,
                node2_idx,
                node2_idx,
                volume_so_far + volume_added_here,
                time_left - 1,
            );
        if volume > max_volume {
            max_volume = volume;
        }

        graph_state.flow_left += flow;
        graph_state.valve_states[node2_idx].is_open = false;
        graph_state.valve_states[node1_idx].is_open = false;
    }

    // 1 opens, 2 walks
    if !graph_state.valve_states[node1_idx].is_open && graph[node1_idx].flow_rate != 0 {
        // try to perform an "open valve" action

        let flow = graph[node1_idx].flow_rate;
        graph_state.valve_states[node1_idx].is_open = true;
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
                        node1_idx,
                        node1_idx,
                        next_idx,
                        node2_idx,
                        volume_so_far + volume_added_here,
                        time_left - 1,
                    );
                if volume > max_volume {
                    max_volume = volume;
                }
            }
        }

        graph_state.flow_left += flow;
        graph_state.valve_states[node1_idx].is_open = false;
    }

    // 2 opens, 1 walks
    if !graph_state.valve_states[node2_idx].is_open && graph[node2_idx].flow_rate != 0 {
        // try to perform an "open valve" action

        let flow = graph[node2_idx].flow_rate;
        graph_state.valve_states[node2_idx].is_open = true;
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
                        next_idx,
                        node1_idx,
                        node2_idx,
                        node2_idx,
                        volume_so_far + volume_added_here,
                        time_left - 1,
                    );
                if volume > max_volume {
                    max_volume = volume;
                }
            }
        }

        graph_state.flow_left += flow;
        graph_state.valve_states[node2_idx].is_open = false;
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
                    next1_idx,
                    node1_idx,
                    next2_idx,
                    node2_idx,
                    volume_so_far,
                    time_left - 1,
                );
                if volume > max_volume {
                    max_volume = volume;
                }
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

    let mut graph: Vec<Valve> = Vec::new();
    let mut graph_state: GraphState = GraphState {
        valve_states: Vec::new(),
        flow_left: 0,
        max_volume_so_far: 0,
    };

    for r in parsed_lines {
        graph.push(Valve {
            flow_rate: r.flow_rate,
            edges: r.edges.iter().map(|name| name_table[name]).collect(),
        });
        graph_state.valve_states.push(ValveState { is_open: false })
    }

    let start_node_idx = name_table["AA"];

    graph_state.flow_left = graph.iter().map(|v| v.flow_rate).sum();

    let result = find_max_volume(
        &graph,
        &mut graph_state,
        start_node_idx,
        start_node_idx,
        start_node_idx,
        start_node_idx,
        0,
        26,
    );

    println!("{}", result);
}
