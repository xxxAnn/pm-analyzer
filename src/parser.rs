//! Parse structures in VIC3's code
//! They typically look something like 
/*
... = {
	... = ...
	... = {
		... = {
			... = ...	
			... = ...
			... = ...			
		}
		... = {
			... = ...
			... = ...
			... = ...
		}
	}
}
*/
//! We want to parse this into a tree structure
//! We can do this by going through the text and saving the last token we saw
//! If we see an = sign followed by a {, we know that the last token was a key
//! We can then create a new node with the key and add it to the current node we are in
//! If we see an # we know this is a comment and we can ignore it
//! If we see a } we know we are done with the current node and we can go back to the parent node
//! We artificially start with a root node that has no name. This is the root of the tree.
//! When we are done parsing, we can return the root node.
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    unique_id: usize,
    children: Vec<Rc<RefCell<Node>>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.unique_id == other.unique_id
    }
}

impl Node {
    fn new(name: String, unique_id: usize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            name,
            unique_id,
            children: Vec::new(),
        }))
    }

    fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child);
    }
}

pub struct Parser {
    root: Rc<RefCell<Node>>,
    unique_id: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            root: Node::new("root".to_string(), 0),
            unique_id: 0
        }
    }

    fn new_node(&mut self, name: String) -> Rc<RefCell<Node>> {
        self.unique_id += 1;
        Rc::new(RefCell::new(Node {
            name,
            unique_id: self.unique_id,
            children: Vec::new(),
        }))
    }

    pub fn parse(mut self, text: String) -> Rc<RefCell<Node>> {
        let mut current = self.root.clone();
        let mut leaf_list = Vec::new();
        let mut equal_seen = false;
        let mut line_skip = false;
        let mut last_token = String::new();

        for line in text.lines() {
            for token in line.split_whitespace() {
                if line_skip {
                    break;
                }
                match token {
                    "=" => {
                        equal_seen = true;
                    }
                    "{" => {

                        leaf_list = Vec::new();
                        if equal_seen {
                            let key = last_token.clone();
                            let new_node = self.new_node(key);
                            current.borrow_mut().add_child(new_node.clone());
                            current = new_node;
                        }
                        equal_seen = false;
                    }
                    "}" => {
                        // Go back to the parent node
                        if (leaf_list.len() > 0) {
                            let key = last_token.clone();
                            for leaf in leaf_list {
                                let leaf_node = self.new_node(leaf);
                                current.borrow_mut().add_child(leaf_node);
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
                            let new_node = self.new_node(key);
                            let leaf = self.new_node(value);   
                            new_node.borrow_mut().add_child(leaf);
                            current.borrow_mut().add_child(new_node.clone());
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

        self.root.clone()
    }

    fn find_parent(&self, root: &Rc<RefCell<Node>>, node: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        if root.borrow().children.contains(node) {
            return Some(root.clone());
        }
        for child in &root.borrow().children {
            if let Some(parent) = self.find_parent(child, node) {
                return Some(parent);
            }
        }
        None
    }
}


pub fn print_tree(node: &Rc<RefCell<Node>>, depth: usize) {
    for _ in 0..depth {
        print!("  ");
    }
    println!("{}", node.borrow().name);
    for child in &node.borrow().children {
        print_tree(child, depth + 1);
    }
}