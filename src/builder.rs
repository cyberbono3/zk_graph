use crate::operation::Operation;
use std::collections::HashMap;

use crate::node::{EqualityConstraint, Node};

/// A builder that will be used to create a computational graph.
#[derive(Default)]
pub struct Builder {
    nodes: Vec<Operation>,
    constraints: Vec<EqualityConstraint>,
    values: HashMap<usize, u32>,
}

impl Builder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    fn add_node(&mut self, op: Operation) -> Node {
        let id = self.nodes.len();
        self.nodes.push(op);
        Node::new(id)
    }

    /// Initializes a node in the graph.
    pub fn init(&mut self) -> Node {
        self.add_node(Operation::Input)
    }

    /// Initializes a node in a graph, set to a constant value.
    pub fn constant(&mut self, value: u32) -> Node {
        self.add_node(Operation::Constant(value))
    }

    /// Adds 2 nodes in the graph, returning a new node.
    pub fn add(&mut self, a: Node, b: Node) -> Node {
        self.add_node(Operation::Add(a.id(), b.id()))
    }

    /// Multiplies 2 nodes in the graph, returning a new node.
    pub fn mul(&mut self, a: Node, b: Node) -> Node {
        self.add_node(Operation::Multiply(a.id(), b.id()))
    }

    /// Asserts that 2 nodes are equal.
    pub fn assert_equal(&mut self, a: Node, b: Node) {
        let ec = EqualityConstraint::new(a.id(), b.id());
        self.constraints.push(ec);
    }

    fn evaluate_node(&self, node_id: usize) -> Option<u32> {
        match &self.nodes[node_id] {
            Operation::Input => self.values.get(&node_id).copied(),
            Operation::Constant(value) => Some(*value),
            Operation::Add(a, b) => {
                let a_val = self.values.get(a)?;
                let b_val = self.values.get(b)?;
                Some(a_val.wrapping_add(*b_val))
            }
            Operation::Multiply(a, b) => {
                let a_val = self.values.get(a)?;
                let b_val = self.values.get(b)?;
                Some(a_val.wrapping_mul(*b_val))
            }
            Operation::Hint(f) => f.compute(&self.values),
        }
    }

    /// Fills in all the nodes of the graph based on some inputs.
    pub fn fill_nodes(&mut self, inputs: &[(Node, u32)]) {
        self.values.clear();

        for (node, value) in inputs {
            self.values.insert(node.id(), *value);
        }

        for (id, op) in self.nodes.iter().enumerate() {
            if let Operation::Constant(value) = op {
                self.values.insert(id, *value);
            }
        }

        let mut changed = true;
        while changed {
            changed = false;
            for node_id in 0..self.nodes.len() {
                #[allow(clippy::map_entry)]
                if !self.values.contains_key(&node_id) {
                    if let Some(value) = self.evaluate_node(node_id) {
                        self.values.insert(node_id, value);
                        changed = true;
                    }
                }
            }
        }
    }
   
    /// Given a graph that has `fill_nodes` already called on it
    /// checks that all the constraints hold.
    pub fn check_constraints(&self) -> bool {
        self.constraints.iter().all(|constraint| {
            match (
                self.values.get(&constraint.left()),
                self.values.get(&constraint.right()),
            ) {
                (Some(left), Some(right)) => left == right,
                _ => false,
            }
        })
    }

    pub fn get_value(&self, node: &Node) -> Option<u32> {
        self.values.get(&node.id()).copied()
    }

    /// An API for hinting values that allows you to perform operations
    /// like division or computing square roots.
    pub fn hint<F>(&mut self, f: F) -> Node
    where
        F: Fn(&HashMap<usize, u32>) -> Option<u32> + Clone + Send + 'static,
    {
        self.add_node(Operation::Hint(Box::new(f)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // Example 1: f(x) = x^2 + x + 5
        let mut builder = Builder::new();
        let x = builder.init();
        let x_squared = builder.mul(x, x);
        let five = builder.constant(5);
        let x_squared_plus_5 = builder.add(x_squared, five);
        let result = builder.add(x_squared_plus_5, x);

        builder.fill_nodes(&[(x, 3)]);
        assert_eq!(builder.get_value(&result), Some(17)); // Should print 17 (3^2 + 3 + 5)
    }
}