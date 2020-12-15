
use crate::utils::string_from_file;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::{algo, prelude::*};

pub fn run() {
    let input = string_from_file("src/10input");

    println!("{}", solve(input));
}

// Keeping this solution as it works but it's inefficient because it calculates a Vec of all
// paths, which grows to over 24 trillion. Walking the graph is more efficient.
#[allow(dead_code)]
pub fn solve_inefficiently(input: String) -> usize {
    let mut adapters = input
        .lines()
        .map(|l| l.trim())
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    adapters.sort();

    let mut graph: Graph<usize, usize> = DiGraph::new();

    let target = adapters.iter().max().unwrap() + 3;
    let target_node = graph.add_node(target);

    let origin: NodeIndex = graph.add_node(0);
    let mut nodes = vec![origin];

    adapters.iter().for_each(|x| nodes.push(graph.add_node(*x)));

    nodes.push(target_node);

    for cursor in 0..nodes.len() {
        let current_node_index = nodes[cursor];
        let current_node_value = *graph.node_weight(current_node_index).unwrap();
        for c in cursor + 1..(nodes.len().min(cursor + 4)) {
            let attempt = *graph.node_weight(nodes[c]).unwrap();
            let max = current_node_value + 3;
            // println!("attempt: {} max: {}", attempt, max);
            if attempt <= max {
                // println!("ADDING EDGE {}->{}", current_node_value, attempt);
                graph.add_edge(current_node_index, nodes[c], 1);
            } else {
                break;
            }
        }
    }

    // let mut dot_file = File::create("graph.dot").expect("Unable to create file");
    // dot_file
    //     .write_all(format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])).as_bytes())
    //     .expect("Unable to write dot file");

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let paths = algo::all_simple_paths::<Vec<NodeIndex>, _>(
        &graph,
        origin,
        target_node,
        0,
        None, // No maximum
    );
    paths.count()
}


pub fn solve(input: String) -> usize {
    let mut adapters = input
        .lines()
        .map(|l| l.trim())
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    adapters.sort();

    let mut graph: Graph<usize, usize> = DiGraph::new();

    let target = adapters.iter().max().unwrap() + 3;
    let target_node = graph.add_node(target);

    let origin: NodeIndex = graph.add_node(0);
    let mut nodes = vec![origin];

    adapters.iter().for_each(|x| nodes.push(graph.add_node(*x)));

    nodes.push(target_node);

    for cursor in 0..nodes.len() {
        let current_node_index = nodes[cursor];
        let current_node_value = *graph.node_weight(current_node_index).unwrap();
        for c in cursor + 1..(nodes.len().min(cursor + 4)) {
            let attempt = *graph.node_weight(nodes[c]).unwrap();
            let max = current_node_value + 3;
            // println!("attempt: {} max: {}", attempt, max);
            if attempt <= max {
                // println!("ADDING EDGE {}->{}", current_node_value, attempt);
                graph.add_edge(current_node_index, nodes[c], 1);
            } else {
                break;
            }
        }
    }

    // let mut dot_file = File::create("graph.dot").expect("Unable to create file");
    // dot_file
    //     .write_all(format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])).as_bytes())
    //     .expect("Unable to write dot file");

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let mut paths: Vec<usize> = Vec::new();

    let mut cursor = 0;
    let mut node: NodeIndex;
    while cursor < nodes.len() {
        node = nodes[cursor];

        if graph.neighbors_directed(node, Outgoing).count() > 1 {
            let end: NodeIndex;

            loop {
                cursor += 1;

                if graph.neighbors_directed(nodes[cursor], Outgoing).count() != 1 {
                    continue;
                }

                if graph
                    .neighbors_directed(nodes[cursor + 1], Outgoing)
                    .count()
                    == 1
                {
                    continue;
                }

                end = nodes[cursor];

                break;
            }

            paths.push(
                algo::all_simple_paths::<Vec<NodeIndex>, _>(
                    &graph, node, end, 0, None, // No maximum
                )
                .count(),
            );
        } else {
            cursor += 1;
        }
    }

    paths.iter().product()
}

#[test]
fn example_1() {
    let input = "16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4";

    assert_eq!(solve(input.to_string()), 8);
}

#[test]
fn example_2() {
    let input = "28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3";

    assert_eq!(solve(input.to_string()), 19208);
}

#[test]
fn example_3() {
    let input = "16
    10
    15
    5
    1
    11
    7
    8
    19
    6
    12
    4";

    assert_eq!(solve(input.to_string()), 29);
}
