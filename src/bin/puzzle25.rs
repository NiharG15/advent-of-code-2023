use std::collections::HashMap;

use itertools::Itertools;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::{Graph, Undirected};
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    name: &'static str,
}

fn main() {
    // let input = include_str!("../../inputs/puzzle25_sample.txt");
    let input = include_str!("../../inputs/puzzle25.txt");

    let lines = input.lines().collect_vec();

    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();

    let mut edges: Vec<(NodeIndex, NodeIndex)> = vec![];

    let mut graph: Graph<&str, (), Undirected> = UnGraph::new_undirected();

    for line in &lines {
        let (source, dests) = line.split(':').collect_tuple().unwrap();
        let destinations = dests.split_ascii_whitespace().map(str::trim).collect_vec();
        let source_node = get_or_insert_node(&mut nodes, &mut graph, source);
        for dest_node in destinations {
            let destination = get_or_insert_node(&mut nodes, &mut graph, dest_node);
            edges.push((source_node, destination));
        }
    }

    graph.extend_with_edges(&edges);

    let result = stoer_wagner_min_cut(&graph, |_| Ok::<i32, i32>(1));


    let (min_cut, partition) = result.unwrap().unwrap();

    println!("-- Part 1 Ans: {}", (graph.node_count() - partition.len()) * partition.len());
}

fn get_or_insert_node(nodes: &mut HashMap<&str, NodeIndex>, graph: &mut Graph<&str, (), Undirected>, source: &'static str) -> NodeIndex {
    let source_node = if nodes.contains_key(source) {
        *nodes.get(source).unwrap()
    } else {
        let node_index = graph.add_node(source);
        nodes.insert(source, node_index);
        node_index
    };
    source_node
}
