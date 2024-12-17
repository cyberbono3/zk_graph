use crate::node::HintFn;


#[derive(Clone)]
pub enum Operation {
    Input,
    Constant(u32),
    Add(usize, usize),
    Multiply(usize, usize),
    Hint(Box<dyn HintFn>),
}


impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Input => write!(f, "Input"),
            Operation::Constant(v) => write!(f, "Constant({})", v),
            Operation::Add(a, b) => write!(f, "Add({}, {})", a, b),
            Operation::Multiply(a, b) => write!(f, "Multiply({}, {})", a, b),
            Operation::Hint(_) => write!(f, "Hint(<function>)"),
        }
    }
}