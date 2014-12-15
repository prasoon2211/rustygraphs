use std::collections::HashMap;
use std::vec::Vec;

struct Graph<'a> {
    nodes: Vec<Node>,
    adj_list: HashMap<*mut Node, Vec<*mut Node>>,
    name: String,
}

#[deriving(Eq, PartialEq, Hash)]
enum Node {
    Str(String),
    Int(int),
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a> {
        // Create an empty Graph
        Graph {
            nodes: Vec::new(),
            adj_list: HashMap::new(),
            name: String::new(),
        }
    }

    fn name(&self) -> &String {
        // Return name of graph
        return &self.name;
    }

    fn add_node(&mut self, mut node: Node) {
        // Add a single node to graph
        // Check for repeated nodes. If exists, do nothing.
        if self.has_node(&node) {
            return;
        }
        // Add node
        self.adj_list.insert(&mut node, Vec::new());
        self.nodes.push(node);
    }

    fn add_edge(&mut self, mut node1: Node, mut node2: Node) {
        // Add a single edge between two nodes
        // Nodes may or may not be already added.

        // Check if edge is already present
        if self.has_edge(&mut node1, &mut node2) {
            return;
        }

        // Create raw ptrs
        let node1_ptr: *mut Node = &mut node1;
        let node2_ptr: *mut Node = &mut node2;

        // Check if nodes exist already
        if !self.has_node(&node1) {
            self.add_node(node1);
        }
        if !self.has_node(&node2) {
            self.add_node(node2);
        }

        // Add edges
        // Now we add the edge twice - 1-2 and 2-1
        self.adj_list[node1_ptr].push(node2_ptr);
        self.adj_list[node2_ptr].push(node1_ptr);
    }

    // Helpers from here on out
    fn has_node(&self, node: &Node) -> bool {
        if self.nodes.contains(node) {
            return true;
        }
        return false;
    }

    fn has_edge(&self, node1: *mut Node, node2: *mut Node) -> bool {
        if self.adj_list[node1].contains(&node2) {
            return true;
        }
        return false;
    }

    fn extract_node(&self, node: Node) -> String {
        let node_name = match node {
            Node::Str(s) => s,
            Node::Int(v) => v.to_string(),
        };
        return node_name;
    }
}
