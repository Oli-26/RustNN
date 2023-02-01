
use crate::node::{Node};

pub struct Layer {
    pub depth : i32,
    pub nodes : Vec<Node>,
}

impl Layer {
    pub fn new(depth: i32, node_count : i32) -> Self {
        let mut nodes : Vec<Node> = vec![];
        for _n in 0..node_count {
            nodes.push(Node::new());
        }
        
        Self{
            depth,
            nodes,
        }
    }
}