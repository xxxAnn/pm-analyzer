use std::{cell::RefCell, rc::Rc};

use super::node::Node;


pub fn stringify_tree(node: &Node, depth: usize) -> String {
    let mut result = String::new();
    for _ in 0..depth {
        result.push_str("  ");
    }
    result.push_str(&format!("{}\n", node.name()));
    for child in &node.children() {
        result.push_str(&stringify_tree(child, depth + 1));
    }

    result
}