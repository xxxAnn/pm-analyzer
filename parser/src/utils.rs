use super::{node::Node, Tree};


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

pub fn merge_trees(trees: &[Tree]) -> Tree {
    if trees.len() == 0 {
        return Tree::new(Node::new("empty".to_owned(), 0), 1);
    }
    let mut new_tree = trees[0].clone();
    for tree in trees.iter().skip(1) {
        new_tree = new_tree.merge(tree);
    }
    new_tree
}