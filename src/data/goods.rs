use std::collections::HashMap;
use crate::parser::Tree;

pub struct Goods {
    cost: HashMap<String, i32>,
}

impl Goods {
    pub fn new() -> Goods {
        Goods {
            cost: HashMap::new(),
        }
    }

    pub fn get_cost(&self, key: &str) -> i32 {
        *self.cost.get(key).unwrap_or(&0)
    }

    fn set_cost(&mut self, key: &str, value: i32) {
        self.cost.insert(key.to_string(), value);
    }
}

impl Goods {
    pub fn from_tree(tree: Tree) -> Result<Self, String> {
        let mut goods = Goods::new();
        for child in tree.get_children_names() {
            let value = tree.get(child.clone())?.get("cost")?.value()?.parse::<i32>().unwrap();
            goods.set_cost(&child, value);
        }
        Ok(goods)
    }
}

impl From<Tree> for Goods {
    fn from(tree: Tree) -> Self {
        Goods::from_tree(tree).unwrap()
    }
}