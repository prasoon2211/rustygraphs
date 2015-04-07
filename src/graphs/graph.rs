use std::collections::HashMap;
use std::vec::Vec;
use super::super::errors::GraphError;
use std::fmt;
use std::fmt::Show;

pub struct Graph {
    nodes: Vec<Node>,
    attr_list: HashMap<uint, HashMap<String, String>>,
    adj_list: HashMap<uint, Vec<uint>>,
    name: String,
}

#[deriving(Eq, PartialEq, Hash, Clone, Show)]
pub enum Node {
    Str(String),
    Int(int),
}

struct Edge(uint, uint);

// Note that changing the nodes Vec physically in any way
// must be accompanied by managing the raw pointers within
// the adj_list of the Graph.
impl Graph {
    pub fn new() -> Graph {
        // Create an empty Graph
        Graph {
            nodes: Vec::new(),
            attr_list: HashMap::new(),
            adj_list: HashMap::new(),
            name: String::new(),
        }
    }

    pub fn name(&self) -> &String {
        // Return name of graph
        return &self.name;
    }

    pub fn add_node(&mut self, node: Node) -> &Node {
        if self.has_node(&node) {
            return self.existing_node(&node);
        }

        self.nodes.push(node);
        let node_ref = self.last_node();

        // internally Vec::len returns self.len (struct field)
        self.adj_list.insert(self.nodes.len(), Vec::new());
        self.attr_list.insert(self.nodes.len(), HashMap::new());

        return node_ref;
    }

    pub fn add_nodes_multiple(&mut self, nodes: Vec<Node>) -> Vec<&Node> {
        // Add several nodes at once.
        let node_refs = Vec::<&Node>::new();
        for node in nodes.into_iter() {
            node_refs.push(self.add_node(node));
        }
        return node_refs;
    }

    pub fn set_node_attr(&mut self, node: &Node,
                     node_attr: HashMap<String, String>) {
        if !self.has_node(node) {
            panic!("Node does not exist in graph.");
        }
        let index = self.get_index(node);
        self.attr_list.insert(index, node_attr);
    }

    pub fn remove_node(&mut self, node: &Node) -> Result<Node, GraphError> {
        // Check for existence and remove the given node.
        // All edges connected to this node are removed, too

        // Manually manage the raw ptr to the removed node
        // We do these three things:
        // 1. Remove edges from the adj_list
        // 2. Remove (swap_remove) the actual Node from nodes
        // 3. Remove attr_dict.
        // 4. Update the pointers

        // We're using raw pointers so we need to be careful as Rust
        // won't save us if we mess up.

        if !self.has_node(node) {
            return Err(GraphError::NodeNotFound);
        }

        let rm_node_index = self.get_index(node);
        let mut index: uint;
        // clone so that double borrow doesn't occur
        let mut conn_nodes = self.adj_list[rm_node_index].clone();
        // type(conn_node) == &Vec<uint>

        for conn_node in conn_nodes.iter() {
            // type(conn_node) == &uint
            let nodes_vec = &mut self.adj_list[*conn_node];
            // Get index of the node to be removed
            index = 0;
            for node_ref_index in nodes_vec.iter() {
                if *node_ref_index == rm_node_index {
                    break;
                }
                index += 1;
            }
            nodes_vec.swap_remove(index);
        }
        // Remove the key to node in adj_list
        self.adj_list.remove(&rm_node_index);

        // Now remove the actual node
        let ret_node: Node;
        match self.nodes.swap_remove(rm_node_index) {
            Some(x) => { ret_node = x; }
            None => { panic!("Shouldn't reach here!"); }
        };

        // Change all of last node's index to rm_node_index
        // (See def of swap_remove)
        let last_node_index = self.nodes.len() + 1; // since one node was removed
        // Wherever `last_node_index` occurs, replace it with `rm_node_index`

        conn_nodes = self.adj_list[last_node_index].clone();

        for conn_node in conn_nodes.iter() {
            let nodes_vec = &mut self.adj_list[*conn_node];
            // Get index of the node to be corrected
            index = 0;
            for node_ref in nodes_vec.iter() {
                if *node_ref == last_node_index {
                    break;
                }
                index += 1;
            }
            nodes_vec[index] = rm_node_index;
        }

        self.adj_list.remove(&last_node_index);
        self.adj_list.insert(rm_node_index, conn_nodes);

        // Remove the node from attr_list
        self.attr_list.remove(&rm_node_index);
        // ...and, all done! Now, we return the removed node.
        return Ok(ret_node);
    }

    pub fn add_edge(&mut self, node1: &Node, node2: &Node) {
        // Add a single edge between two nodes
        // Nodes may or may not be already added.

        // Check if edge is already present
        if self.has_edge(node1, node2) {
            return;
        }

        // Check if nodes exist already
        if !self.has_node(node1) {
            let clone_node1 = node1.clone();
            node1 = self.add_node(clone_node1);
        }

        if !self.has_node(node2) {
            let clone_node2 = node2.clone();
            node2 = self.add_node(clone_node2);
        }
        let node1_index = self.get_index(node1);
        let node2_index = self.get_index(node2);

        // Add edges
        // Now we add the edge twice - 1-2 and 2-1
        self.adj_list[node1_index].push(node2_index);
        self.adj_list[node2_index].push(node1_index);
    }

    // Helpers from here on out
    // To be used internally only. No public API.

    fn edges(&self) -> Vec<Edge> {
        // Return all edges of a Graph
        let mut edge_vec = Vec::<Edge>::new();
        let mut visited = Vec::<uint>::new();
        for (node, nbrs) in self.adj_list.iter() {
            for nbr in nbrs.iter() { // methods work on refs, too
                // nbr of type &uint
                if !visited.contains(nbr) { // uint is copyable
                    edge_vec.push(Edge(*node, *nbr));
                }
            }
            visited.push(*node);
        }
        return edge_vec;
    }


    fn get_index(&self, node: &Node) -> uint {
        // All nodes are unique which allows us to assign each node an index
        // Run through the Vec to get the index
        let mut index = 0;
        for node_ref in self.nodes.iter() {
            if *node_ref == *node {
                return index;
            }
            index += 1;
        }
        // No node found. Node doesn't exist
        // Since it is internal function, there should occur no such situation
        // Panic.
        panic!("Node does not exist.");
    }


    fn has_node(&self, node: &Node) -> bool {
        for n in self.nodes.iter() {
            if *n == *node {
                return true;
            }
        }
        return false;
    }

    fn has_edge(&self, node1: &Node, node2: &Node) -> bool {
        let n1_ind = self.get_index(node1);
        let n2_ind = self.get_index(node2);
        if self.adj_list[n1_ind].contains(&n2_ind) {
            return true;
        }
        return false;
    }

    fn extract_node(&self, node: Node) -> String {
        let node_name = match node {
            Node::Str(s) => s,
            Node::Int(v) => v.to_string()
        };
        return node_name;
    }

    fn existing_node(&self, node: &Node) -> &Node {
        // Return ref to existing node
        for n in self.nodes.iter() {
            if *node == *n {
                return n;
            }
        }
        panic!("No such node.");
    }

    fn last_node(&self) -> &Node { &self.nodes[self.nodes.len()] }
}


impl Show for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Shows textual repr of Graph
        try!(write!(f, "{{ Nodes: "));
        for n in self.nodes.iter() {
            try!(write!(f, "{}, ", n));
        }
        try!(writeln!(f, ""));
        try!(write!(f, "Edges: "));
        for edge in self.edges().iter() {
            try!(write!(f, "{}, ", edge));
        }
        write!(f, "}}")
    }
}

impl Show for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Edge(node1, node2) = *self;
        write!(f, "{}--{}", node1, node2)
    }
}
