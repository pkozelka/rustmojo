

// logical structure of decision trees

pub trait Bitset {
    fn get(&self, bit: i32) -> bool;
}

pub trait Column {
    fn get_column_no(&self) -> usize;
}

pub enum Comparison {
    None,
    Numeric(f64),
    Bitset(Box<dyn Bitset>),
}

pub enum NoNumberHandling {
    None,
    AsTrue,
    AsFalse
}

pub struct Condition {
    pub nan: NoNumberHandling,
    pub comparison: Comparison,
    pub invert: bool,
}

pub struct DecisionNode {
    pub column: Box<dyn Column>,
    pub condition: Condition,
    pub do_then: Box<Node>,
    pub do_else: Box<Node>,
}

pub enum Node {
    ValueNode(f64),
    DecisionNode(DecisionNode)
}

impl DecisionNode {
    #[allow(dead_code)]
    fn new(column: Box<dyn Column>, condition: Condition, do_then: Node, do_else: Node) -> DecisionNode {
        DecisionNode {
            column,
            condition,
            do_then: Box::new(do_then),
            do_else: Box::new(do_else)
        }
    }
}

pub struct Col {
    column_no: usize
}

impl Col {
    pub fn new(column_no: usize) -> Self {
        Col {
            column_no
        }
    }
}

impl Column for Col {
    fn get_column_no(&self) -> usize {
        self.column_no
    }
}
