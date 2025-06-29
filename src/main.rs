

use std::ptr;
use std::alloc::{alloc, dealloc};


struct Node {
    data: f32,
    height: usize,
    left: Option<*mut Node>,
    right: Option<*mut Node>,
}
impl Node {
    fn new(data: f32) -> Node {
        Node {
            data: data,
            height: 0,
            left: None,
            right: None
        }
    }
}

struct Tree {
    root: Node,
}
impl Tree {
    fn new(root: Node) ->Self {
        Self {
            root: root,
        }
    }
    fn getHeight(root: &Node) -> usize {
        root.height
    }
    fn getBalance(root: &Node)  {
       
    }
}



fn main() {
    println!("HELLO WORLD")
}