

// logical structure of decision trees

pub trait Bitset {
    fn get(&self, bit: i32) -> bool;
}

pub trait Column {
    fn get_column_no(&self) -> i32;
}

enum Condition {
    IsNotAssigned(),
    IsLessThan(f32),
    IsLessThanOrNA(f32),
    BitsetContains(Box<Bitset>)
}

struct DecisionNode {
    column: Box<Column>,
    condition: Condition,
    invert: bool,
    do_then: Box<Node>,
    do_else: Box<Node>,
}

enum Node {
    ValueNode(f32),
    DecisionNode(DecisionNode)
}

impl DecisionNode {
    fn new(column: Box<Column>, condition: Condition, invert: bool, do_then: Node, do_else: Node) -> DecisionNode {
        DecisionNode {
            column,
            condition,
            invert,
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
    let col1 = Box::new(Col::new(1));
    let cond = Condition::IsNotAssigned();
    let tree = Node::DecisionNode(DecisionNode::new(col1, cond, false, Node::ValueNode(5.3), Node::ValueNode(1.2)));

    match tree {
        Node::DecisionNode(d) => println!("Hi {}", d.invert),
        Node::ValueNode(n) => println!("Number = {}", n)
    }
}
