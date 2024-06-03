use super::node::Node;

#[derive(Debug)]
pub struct Tree {
    root: Node,
    highest_id: usize,
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        Tree {
            root: self.root.clone(),
            highest_id: self.highest_id,
        }
    }
}

impl Tree {
    pub fn new(root: Node, highest_id: usize) -> Tree {
        Tree {
            root,
            highest_id
        }
    }

    fn children(&self) -> Vec<Node> {
        self.root.children()
    }
    
    pub fn get_highest_id(&self) -> usize {
        self.highest_id
    }

    pub fn get_name(&self) -> String {
        self.root.name().clone().replace("\u{feff}", "")
    }

    pub fn get_children_names(&self) -> Vec<String> {
        self.children().iter().map(|child| child.name().clone()).collect()
    }

    pub fn get_children_names_filtered(&self, filter: impl Fn(&String) -> bool) -> Vec<String> {
        self.get_children_names().into_iter().filter(|name| filter(name)).collect()
    }

    pub fn value(&self) -> Result<String, String> {
        if self.children().len() == 1 {
            Ok(self.children().first().unwrap().name().clone())
        } else {
            Err("Node is not a leaf".to_string())
        }
    }

    pub fn add_child_tree(&mut self, child: Tree) {
        self.root.add_child(child.root.clone());
    }

    pub fn default() -> Tree {
        Tree::new(Node::new("root".to_string(), 0), 1)
    }

    pub fn with_named_root(name: &str) -> Tree {
        Tree::new(Node::new(name.to_string(), 0), 1)
    }

    pub fn get(&self, name: impl Into<String>) -> Result<Tree, String> {
        self.get_scope_internal(&[name.into()])
    }

    fn get_scope_internal(&self, name: &[String]) -> Result<Tree, String> {
        let mut v = vec!["root".to_string()];
        v.extend(name.iter().cloned());
        match self.find_scope(&v, None) {
            Some(scope) => Ok(Tree::new(scope, self.get_highest_id())),
            None => Err("Scope not found".to_string())
        }
    }

    fn find_scope(&self, name: &[String], root: Option<Node>) -> Option<Node> {
        let mut current = root.unwrap_or(self.root.clone());
        let first = name.first()?;
        for child in &self.children() {
            if child.name() == *first {
                current = child.clone();
            }
        }
        if name.len() == 1 {
            return Some(current);
        }
        self.find_scope(&name[1..], Some(current))
    }

}

pub struct TreeIterator {
    stack: Vec<Node>,
    highest_id: usize,
}

impl Iterator for TreeIterator {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }
        let node = self.stack.pop().unwrap();
        Some(Tree::new(node, self.highest_id))
    }
}

impl IntoIterator for Tree {
    type Item = Tree;
    type IntoIter = TreeIterator;

    fn into_iter(self) -> Self::IntoIter {
        TreeIterator {
            stack: self.children().iter().cloned().collect(),
            highest_id: self.get_highest_id(),
        }
    }
}

impl Tree {
    pub fn into_iter_filtered(self, filter: impl Fn(&String) -> bool) -> TreeIterator {
        TreeIterator {
            stack: self.children().iter().cloned().filter(|child| filter(&child.name())).collect(),
            highest_id: self.get_highest_id(),
        }
    }

    pub fn search_child(&self, filter: impl Fn(&String) -> bool) -> Option<Tree> {
        for child in self.children() {
            if filter(&child.name()) {
                return Some(Tree::new(child, self.get_highest_id()));
            }
        }
        None
    }

    pub fn add_child(&mut self, child: Node) {
        self.root.add_child(child);
    }

    pub fn move_up(&self, key: &str) -> Tree {
        // moves all children of the key node to the root
        let mut new_root = Node::new("root".to_string(), self.get_highest_id());
        for child in self.children() {
            if child.name().contains(key) {
                for grandchild in child.children() {
                    new_root.add_child(grandchild.clone());
                }
            }
        }
        Tree::new(new_root, self.get_highest_id())
    }
}

impl ToString for Tree {
    fn to_string(&self) -> String {
        super::utils::stringify_tree(&self.root, 0)
    }
}

    // Merge two trees together
    // This will be useful when we read through
    // multiple files and want to merge them together

impl Tree {

    pub fn merge(&self, other: &Tree) -> Tree {
        let new_tree = self.hard_clone();
        new_tree.merge_internal(other)
    }

    pub fn from_key_value(key: &str, value: &str, highest_id: usize) -> Node {
        let mut key_node = Node::new(key.to_string(), highest_id);
        let value_node = Node::new(value.to_string(), highest_id + 1);
        key_node.add_child(value_node);
        key_node
    }

    pub fn delete_children_filtered(&self, filter: impl Fn(&String) -> bool) -> Tree {
        let mut new_tree = self.hard_clone();
        let mut children = new_tree.children();
        children.retain(|child| !filter(&child.name()));
        new_tree.root.set_children(children);
        new_tree
    }


    pub fn set_children(&mut self, children: Vec<Node>) {
        self.root.set_children(children);
    }

    pub fn from_children(children: &[Tree], highest_id: usize) -> Tree {
        let mut root = Node::new("root".to_string(), highest_id);
        for child in children {
            root.add_child(child.root.clone());
        }
        Tree::new(root, highest_id)
    }

    fn hard_clone(&self) -> Tree {
        let new_tree = Tree::new(self.root.clone(), self.get_highest_id());
        new_tree
    }

    fn merge_internal(mut self, other: &Tree) -> Tree {
        let mut new_uids = self.get_highest_id();
        for child in &other.root.children() {
            let mut new_child = child.clone();
            new_child.set_unique_id(new_uids);
            new_uids += 1;
            self.root.add_child(child.clone());
        }

        return self;
    }  
}

impl Tree {
    pub fn serialize(&self) -> String {
        let mut result = String::new();
        self.serialize_node(&self.root, &mut result, 0);
        result
    }

    fn serialize_node(&self, node: &Node, result: &mut String, depth: usize) {
        let indent = "  ".repeat(depth); // For indentation to maintain readability
        let children = node.children();

        if children.is_empty() {
            // It's a leaf node, simply add the node's name
            result.push_str(&format!("{}{}\n", indent, node.name()));
        } else if children.len() == 1 && node.name() != "root" {
            // It's a key-value pair
            result.push_str(&format!("{}{} = {}\n", indent, node.name(), children[0].name()));
        } else {
            // It's a nested structure
            if node.name() != "root" {
                result.push_str(&format!("{}{} = {{\n", indent, node.name()));
            }
            for child in children {
                self.serialize_node(&child, result, depth + 1);
            }
            if node.name() != "root" {
                result.push_str(&format!("{}}}\n", indent));
            }
        }
    }
}