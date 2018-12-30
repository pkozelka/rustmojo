

// logical structure of decision trees

pub trait Bitset {
    fn get(&self, bit: i32) -> bool;
}

pub trait Column {
    fn get_column_no(&self) -> u16;
}

pub enum Comparison {
    None,
    IsLessThan(f32),
    BitsetContains(Box<Bitset>),
}

pub enum NoNumberHandling {
    None,
    AsTrue,
    AsFalse
}

pub struct Condition {
    pub comparison: Comparison,
    pub invert: bool,
    pub nan: NoNumberHandling,
}

pub struct DecisionNode {
    pub column: Box<Column>,
    pub condition: Condition,
    pub do_then: Box<Node>,
    pub do_else: Box<Node>,
}

pub enum Node {
    ValueNode(f32),
    DecisionNode(DecisionNode)
}

impl DecisionNode {
    fn new(column: Box<Column>, condition: Condition, do_then: Node, do_else: Node) -> DecisionNode {
        DecisionNode {
            column,
            condition,
            do_then: Box::new(do_then),
            do_else: Box::new(do_else)
        }
    }
}

pub struct Col {
    column_no: u16
}

impl Col {
    pub fn new(column_no: u16) -> Self {
        Col {
            column_no
        }
    }
}

impl Column for Col {
    fn get_column_no(&self) -> u16 {
        self.column_no
    }
}
