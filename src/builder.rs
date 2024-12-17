
use crate::operation::Operation;
use std::collections::HashMap;

use crate::node::{Node, EqualityConstraint};

/// A builder that will be used to create a computational graph.
struct Builder {
    nodes: Vec<Operation>,
    constraints: Vec<EqualityConstraint>,
    values: HashMap<usize, u32>,
}


impl Builder {
    /// Creates a new builder.
    pub fn new() -> Self {
        todo!()
    }
    
    /// Initializes a node in the graph.
    pub fn init(&mut self) -> Node {
        todo!()
    }
    
    /// Initializes a node in a graph, set to a constant value.
    pub fn constant(&mut self, value: u32) -> Node {
        todo!()
    }
    
    /// Adds 2 nodes in the graph, returning a new node.
    pub fn add(&mut self, a: Node, b: Node) -> Node {
        todo!()
    }
    
    /// Multiplies 2 nodes in the graph, returning a new node.
    pub fn mul(&mut self, a: Node, b: Node) -> Node {
        todo!()
    }
    
    /// Asserts that 2 nodes are equal.
    pub fn assert_equal(&mut self, a: Node, b: Node) {
        todo!()
    }
    
    /// Fills in all the nodes of the graph based on some inputs.
    pub fn fill_nodes(&mut self) {
        todo!()
    }
    
    /// Given a graph that has `fill_nodes` already called on it
    /// checks that all the constraints hold.
    pub fn check_constraints(&mut self) -> bool {
        todo!()
    }
    
    /// An API for hinting values that allows you to perform operations
    /// like division or computing square roots.
    pub fn hint()  {
        todo!()
    }
}
