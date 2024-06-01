mod node;
mod tree;
pub mod utils;

use node::Node;
pub use tree::Tree;

pub struct Parser {
    root: Node,
    unique_id: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            root: Node::new("root".to_string(), 0),
            unique_id: 0
        }
    }

    fn new_node(&mut self, name: String) -> Node {
        self.unique_id += 1;
        Node::new(name, self.unique_id)
    }
    /// A horror beyond human comprehension
    /// Lol It's so long I'm sorry
    /// Surely there's a recursive way to do this
    /// But you'd need to pass around a ton of references
    /// And it's just not pretty
    /// I'm not sure if this is pretty either to be fair
    pub fn parse(mut self, text: String) -> Tree {
        let mut current = self.root.clone();
        let mut leaf_list = Vec::new();
        let mut equal_seen = false;
        let mut line_skip = false;
        let mut last_token = String::new();

        for line in text.lines() {
            // Add a space before the comment
            // This fixes an issue where a comment is not recognized
            // if it's part of a token
            let fixed_line = line.replace("#", " #"); 
            for token in fixed_line.split_whitespace() {
                if line_skip {
                    break;
                }
                match token {
                    "=" => {
                        equal_seen = true;
                        leaf_list = Vec::new();
                    }
                    "{" => {

                        leaf_list = Vec::new();
                        if equal_seen {
                            let key = last_token.clone();
                            let new_node = self.new_node(key);
                            current.add_child(new_node.clone());
                            current = new_node;
                        }
                        equal_seen = false;
                    }
                    "}" => {
                        // Go back to the parent node
                        if leaf_list.len() > 0 {
                            for leaf in leaf_list {
                                let leaf_node = self.new_node(leaf);
                                current.add_child(leaf_node);
                            }
                            leaf_list = Vec::new();
                        }
                        let parent_node = self.find_parent(&self.root, &current).unwrap();
                        current = parent_node;
                    }
                    "#" => {
                        // This is a comment
                        // We can ignore this line
                        line_skip = true;
                    }
                    _ => {
                        if equal_seen {
                            let key = last_token.clone();
                            let value = token.to_string();
                            let mut new_node = self.new_node(key);
                            let leaf = self.new_node(value);   
                            new_node.add_child(leaf);
                            current.add_child(new_node.clone());
                        } else {
                            leaf_list.push(token.to_string());
                        }
                        equal_seen = false;
                        last_token = token.to_string();
                    }
                }
            }
            line_skip = false;
        }

        Tree::new(self.root, self.unique_id+1)
    }

    fn find_parent(&self, root: &Node, node: &Node) -> Option<Node> {
        if root.children().contains(node) {
            return Some(root.clone());
        }
        for child in &root.children() {
            if let Some(parent) = self.find_parent(child, node) {
                return Some(parent);
            }
        }
        None
    }
}