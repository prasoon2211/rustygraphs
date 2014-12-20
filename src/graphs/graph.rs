use std::collections::HashMap;
use std::vec::Vec;
use super::super::errors::GraphError;
use std::fmt;
use std::fmt::Show;

pub struct Graph<'a> {
    nodes: HashMap<Node, HashMap<String, String>>,
    adj_list: HashMap<*mut Node, Vec<*mut Node>>,
    name: String,
}

#[deriving(Eq, PartialEq, Hash, Clone, Show)]
pub enum Node {
    Str(String),
    Int(int),
}

pub struct Edge(*mut Node, *mut Node);

// Note that changing the nodes Vec physically in any way
// must be accompanied by managing the raw pointers within
// the adj_list of the Graph.
impl<'a> Graph<'a> {
    pub fn new() -> Graph<'a> {
        // Create an empty Graph
        Graph {
            nodes: HashMap::new(),
            adj_list: HashMap::new(),
            name: String::new(),
        }
    }

    pub fn name(&self) -> &String {
        // Return name of graph
        return &self.name;
    }

    pub fn add_node(&mut self, mut node: Node) {
        if self.nodes.contains_key(&node) {
            return;
        }
        self.adj_list.insert(&mut node, Vec::new());
        self.nodes.insert(node, HashMap::new());
    }

    pub fn add_nodes_multiple(&mut self, nodes: Vec<Node>) {
        // Add several nodes at once.
        for node in nodes.into_iter() {
            self.add_node(node);
        }
    }

    pub fn set_node_attr(&mut self, node: &Node,
                     node_attr: HashMap<String, String>) {
        self.nodes.insert(node.clone(), node_attr);
    }

    pub fn remove_node(&mut self, node: &mut Node) -> Result<Node, GraphError> {
        // Check for existence and remove the given node.
        // All edges connected to this node are removed, too

        // Manually manage the raw ptr to the removed node
        // We do these three things:
        // 1. Remove edges from the adj_list
        // 2. Remove (swap_remove) the actual Node from nodes
        // 3. Update the pointers

        // We're using raw pointers so we need to be careful as Rust
        // won't save us if we mess up.

        if !self.nodes.contains_key(node) {
            return Err(GraphError::NodeNotFound);
        }

        let raw_node: *mut Node = node;
        let mut index: uint;
        // clone so that double borrow doesn't occur
        let conn_nodes = self.adj_list[raw_node].clone();
        // type(conn_node) == &Vec<*mut Node>

        for conn_node in conn_nodes.iter() {
            // type(conn_node) == &*mut Node
            // We can do this next step only because of the raw pointer
            let nodes_vec = &mut self.adj_list[*conn_node];
            // Get index of the node to be removed
            index = 0;
            for node_ref in nodes_vec.iter() {
                if *node_ref == raw_node {
                    break;
                }
                index += 1;
            }
            nodes_vec.swap_remove(index);
        }
        // Remove the key to node in adj_list
        self.adj_list.remove(&raw_node);

        // Now remove the actual node
        let ret_node = node.clone();
        self.nodes.remove(node);

        // // Get the raw ptr to the last node since it'll go in the
        // // place of the removed node
        // // Extra var because rust won't let me put the expr directly in []
        // let len_nodes = self.nodes.len();
        // let raw_last_node: *mut Node = &mut self.nodes[len_nodes];
        // let ret_node: Node;
        // match self.nodes.swap_remove(index) {
        //     Some(v) => {
        //         ret_node = v;
        //     }
        //     None => {
        //         return Err(GraphError::CannotRemoveNode);
        //     }
        // };

        // // Node removed from self.nodes. Now manage the raw pointers
        // // Take all raw ptrs to the last node and make them point to
        // // the replaces node. (Vec::swap_remove)
        // conn_nodes = self.adj_list[raw_last_node].clone();
        // let raw_new_node_addr: *mut Node = &mut self.nodes[index];

        // for conn_node in conn_nodes.iter() {
        //     let nodes_vec = &mut self.adj_list[*conn_node];
        //     // Get index of the node to be corrected
        //     index = 0;
        //     for node_ref in nodes_vec.iter() {
        //         if *node_ref == raw_last_node {
        //             break;
        //         }
        //         index += 1;
        //     }
        //     nodes_vec[index] = raw_new_node_addr;
        // }
        // // ...and, all done! Now, we return the removed node.
        return Ok(ret_node);
    }

    pub fn add_edge(&mut self, node1: &mut Node, node2: &mut Node) {
        // Add a single edge between two nodes
        // Nodes may or may not be already added.

        // Check if edge is already present
        if self.has_edge(node1, node2) {
            return;
        }

        // Create raw ptrs
        let mut node1_ptr: *mut Node = node1;
        let mut node2_ptr: *mut Node = node2;

        // Check if nodes exist already
        if !self.nodes.contains_key(node1) {
            let mut clone_node1: Node = node1.clone();
            node1_ptr = &mut clone_node1 as *mut Node;
            self.add_node(clone_node1);
        }
        if !self.nodes.contains_key(node2) {
            let mut clone_node2 = node2.clone();
            node2_ptr = &mut clone_node2 as *mut Node;
            self.add_node(clone_node2);
        }

        // Add edges
        // Now we add the edge twice - 1-2 and 2-1
        self.adj_list[node1_ptr].push(node2_ptr);
        self.adj_list[node2_ptr].push(node1_ptr);
    }

    pub fn edges(&self) -> Vec<Edge> {
        // Return all edges of a Graph
        let mut edge_vec = Vec::<Edge>::new();
        let mut visited = Vec::<*mut Node>::new();
        for (node, nbrs) in self.adj_list.iter() {
            for nbr in nbrs.iter() { // methods work on refs, too
                // nbr is of type &*mut Node
                if !visited.contains(nbr) {
                    edge_vec.push(Edge(*node, *nbr)); // *mut is copyable
                }
            }
            visited.push(*node); // *mut is copyable
        }
        return edge_vec;
    }

    // Helpers from here on out
    // To be used internally only. No public API.

    // fn get_index(&self, node: &Node) -> Result<uint, GraphError> {
    //     // All nodes are unique which allows us to assign each node an index
    //     // Run through the Vec to get the index
    //     let mut index = 0;
    //     for node_ref in self.nodes.iter() {
    //         // node is a ref to an already exisitng node.
    //         // we can thus match just the pointers
    //         if node_ref == node {
    //             return Ok(index);
    //         }
    //         index += 1;
    //     }
    //     return Err(GraphError::NodeNotFound);
    // }


    fn has_edge(&self, node1: *mut Node, node2: *mut Node) -> bool {
        if self.adj_list[node1].contains(&node2) {
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
}


impl<'a> Show for Graph<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Shows textual repr of Graph
        try!(write!(f, "{{ Nodes: "));
        for key in self.nodes.keys() {
            try!(write!(f, "{}, ", key));
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
