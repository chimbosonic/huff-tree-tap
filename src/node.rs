/// INTERNAL ONLY: Represents a Node of a Tree
pub struct Node {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub freq: i64,
    pub value: Option<u8>,
}

impl Node {
    pub fn new_leaf(freq: i64, value: Option<u8>) -> Node {
        Node {
            left: None,
            right: None,
            freq,
            value,
        }
    }

    pub fn new_branch(left: Node, right: Node) -> Node {
        let freq = left.freq + right.freq;
        Node {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            freq,
            value: None,
        }
    }
}
