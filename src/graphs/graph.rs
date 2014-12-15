use std::collections::HashMap;
use std::vec::Vec;

struct Graph<'a> {
    nodes: Vec<Node<'a>,
    adj_list: HashMap<&'a Node, Vec<&'a Node>>,
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
            nodes: Vec::<Node>::new(),
            adj_list: HashMap::<&'a Node, Vec<&'a Node>>::new(),
            name: String::new(),
        }
    }

    fn name(&self) -> &'a String {
        // Return name of graph
        &self.name;
    }

    fn add_node(&mut self, node: Node) -> Graph<'a> {
        // Add a single node to graph
        // Check for repeated nodes. If exists, do nothing.
        if self.has_node(&node) {
            return *self;
        }
        // Add node
        self.adj_list.insert(&node, Vec::new());
        self.nodes.push(node);

        return *self;
    }

    fn add_edge(&mut self, node1: Node, node2: Node) -> Graph<'a> {
        // Add a single edge between two nodes
        // Nodes may or may not be already added.
        // Check if nodes exist already
        if !self.has_node(&node1) {
            self.add_node(node1);
        }
        if !self.has_node(&node2) {
            self.add_node(node2);
        }

        // Add edges
        // Check if edge is already present
        if self.has_edge(&node1, &node2) {
            return *self;
        }
        // Now we add the edge twice - 1-2 and 2-1
        self.adj_list[&node1].push(&node2);
        self.adj_list[&node2].push(&node1);

        return *self;
    }

    // Helpers from here on out
    fn has_node(&self, node: &Node) -> bool {
        if self.nodes.contains(node) {
            return true;
        }
        return false;
    }

    fn has_edge(&self, node1: &'a Node, node2: &'a Node) -> bool {
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
