
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    id: usize,
}

impl Node {
    fn new(id: usize) -> Self {
        Node { id }
    }
}

#[derive(Debug)]
pub struct EqualityConstraint {
    left: usize,
    right: usize,
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


