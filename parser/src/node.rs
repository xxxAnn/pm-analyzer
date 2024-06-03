use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct RawNode {
    name: String,
    unique_id: usize,
    children: Vec<Rc<RefCell<RawNode>>>,
}

#[derive(Debug)]
// Node is a wrapper around RawNode that provides a more user-friendly API
pub struct Node {
    node: Rc<RefCell<RawNode>>,
}

// This clones the Rc, not the underlying node
// We ensure that we are still passing around references to the same node
// This is important because we want to be able to modify the tree in place
// This is probably the default behavior of deriving Clone for a struct that contains an Rc anyway
// But I'm not sure about that idk
impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            node: self.node.clone(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.node.borrow().unique_id == other.node.borrow().unique_id
    }
}

impl Node {
    pub fn new(name: String, unique_id: usize) -> Node {
        Node {
            node: RawNode::new(name, unique_id),
        }
    }

    pub fn set_children(&mut self, children: Vec<Node>) {
        self.node.borrow_mut().children = children.iter().map(|child| child.node.clone()).collect();
    }

    pub fn name(&self) -> String {
        self.node.borrow().name()
    }

    pub fn children(&self) -> Vec<Node> {
        // Convert the children from RawNode to Node
        // This still modifies the original children because we're cloning the Rc, not the underlying node
        self.node.borrow().children().iter().map(|child| Node { node: child.clone() }).collect()
    }

    pub fn add_child(&mut self, child: Node) {
        self.node.borrow_mut().add_child(child.node.clone());
    }

    pub fn set_unique_id(&mut self, unique_id: usize) {
        self.node.borrow_mut().set_unique_id(unique_id);
    }
}

impl PartialEq for RawNode {
    fn eq(&self, other: &Self) -> bool {
        self.unique_id == other.unique_id
    }
}

impl RawNode {
    fn new(name: String, unique_id: usize) -> Rc<RefCell<RawNode>> {
        Rc::new(RefCell::new(RawNode {
            name,
            unique_id,
            children: Vec::new(),
        }))
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn children(&self) -> Vec<Rc<RefCell<RawNode>>> {
        // Clone the children (Rc is a reference-counted pointer, so cloning it just increments the reference count)
       self.children.iter().cloned().collect()
    }

    fn add_child(&mut self, child: Rc<RefCell<RawNode>>) {
        self.children.push(child);
    }

    fn set_unique_id(&mut self, unique_id: usize) {
        self.unique_id = unique_id;
    }

}