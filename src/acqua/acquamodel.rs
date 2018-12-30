

// logical structure of decision trees

pub trait Bitset {
    fn get(&self, bit: i32) -> bool;
}

pub trait Column {
    fn get_column_no(&self) -> i32;
}

enum Comparison {
    None(),
    IsLessThan(f32),
    BitsetContains(Box<Bitset>),
}

struct Condition {
    comparison: Comparison,
    is_na: bool,
    invert: bool,
}

pub struct DecisionNode {
    column: Box<Column>,
    condition: Condition,
    do_then: Box<Node>,
    do_else: Box<Node>,
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

struct Col {
    column_no: i32
}

impl Col {
    fn new(column_no: i32) -> Self {
        Col {
            column_no
        }
    }
}

impl Column for Col {
    fn get_column_no(&self) -> i32 {
        self.column_no
    }
}


fn main() {
    let col1 = Box::new(Col::new(1234));
    let cond = Condition {
        comparison: Comparison::IsLessThan(3.14),
        is_na: false,
        invert: false
    };
    let tree = Node::DecisionNode(DecisionNode::new(col1, cond, Node::ValueNode(5.3), Node::ValueNode(1.2)));

    match tree {
        Node::DecisionNode(d) => println!("Hi {}", d.column.get_column_no()),
        Node::ValueNode(n) => println!("Number = {}", n)
    }
}
