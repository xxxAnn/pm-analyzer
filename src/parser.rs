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
struct Node {
    name: String,
    unique_id: usize,
    children: Vec<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
pub struct Tree {
    root: Rc<RefCell<Node>>,
}

impl Tree {
    fn new(root: Rc<RefCell<Node>>) -> Tree {
        Tree {
            root,
        }
    }

    pub fn get_name(&self) -> String {
        self.root.borrow().name.clone()
    }

    pub fn get_children_names(&self) -> Vec<String> {
        self.root.borrow().children.iter().map(|child| child.borrow().name.clone()).collect()
    }

    pub fn get_children_names_filtered(&self, filter: impl Fn(&String) -> bool) -> Vec<String> {
        self.root.borrow().children.iter().map(|child| child.borrow().name.clone()).filter(|name| filter(name)).collect()
    }

    pub fn value(&self) -> Result<String, String> {
        if self.root.borrow().children.len() == 1 {
            Ok(self.root.borrow().children.first().unwrap().borrow().name.clone())
        } else {
            Err("Node is not a leaf".to_string())
        }
    }

    pub fn get(&self, name: impl Into<String>) -> Result<Tree, String> {
        self.get_scope_internal(&[name.into()])
    }

    fn get_scope_internal(&self, name: &[String]) -> Result<Tree, String> {
        let mut v = vec!["root".to_string()];
        v.extend(name.iter().cloned());
        match self.find_scope(&v, None) {
            Some(scope) => Ok(Tree::new(scope)),
            None => Err("Scope not found".to_string())
        }
    }

    fn find_scope(&self, name: &[String], root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut current = root.unwrap_or(self.root.clone());
        let first = name.first()?;
        for child in &self.root.borrow().children {
            if child.borrow().name == *first {
                current = child.clone();
            }
        }
        if name.len() == 1 {
            return Some(current);
        }
        self.find_scope(&name[1..], Some(current))
    }

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

    pub fn parse(mut self, text: String) -> Tree {
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
                        leaf_list = Vec::new();
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
                        if leaf_list.len() > 0 {
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

        Tree::new(self.root)
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

impl ToString for Tree {
    fn to_string(&self) -> String {
        stringify_tree(&self.root, 0)
    }
}

fn stringify_tree(node: &Rc<RefCell<Node>>, depth: usize) -> String {
    let mut result = String::new();
    for _ in 0..depth {
        result.push_str("  ");
    }
    result.push_str(&format!("{}\n", node.borrow().name));
    for child in &node.borrow().children {
        result.push_str(&stringify_tree(child, depth + 1));
    }

    result
}