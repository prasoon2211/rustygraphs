extern crate rustygraphs;

use rustygraphs::graphs::graph::{Graph, Node, Edge};
use std::vec::Vec;

// This is the main executable crate - it is in no relation to the library.
// This is just an example usage.

fn main() {
    let mut g = Graph::new();
    let mut nodes = Vec::new();
    nodes.push(Node::Str("Maths".to_string()));
    nodes.push(Node::Str("Physics".to_string()));
    nodes.push(Node::Str("Chemistry".to_string()));
    let mut nodes1 = nodes.clone();
    g.add_nodes_multiple(nodes);

    // g.add_edge(&nodes1[0], &nodes1[1]);
    println!("{}", g);
}
