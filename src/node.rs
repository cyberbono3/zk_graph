use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct Node {
    id: usize,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Node { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug)]
pub struct EqualityConstraint {
    left: usize,
    right: usize,
}

impl EqualityConstraint {
    pub fn new(left: usize, right: usize) -> Self {
        Self { left, right }
    }

    pub fn left(&self) -> usize {
        self.left
    }

    pub fn right(&self) -> usize {
        self.right
    }
}

pub trait HintFn: Send {
    fn compute(&self, values: &HashMap<usize, u32>) -> Option<u32>;
    fn box_clone(&self) -> Box<dyn HintFn>;
}

impl<F> HintFn for F
where
    F: Fn(&HashMap<usize, u32>) -> Option<u32> + Clone + Send + 'static,
{
    fn compute(&self, values: &HashMap<usize, u32>) -> Option<u32> {
        self(values)
    }

    fn box_clone(&self) -> Box<dyn HintFn> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn HintFn> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
