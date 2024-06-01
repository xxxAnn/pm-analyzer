use std::collections::HashMap;


mod pm;
mod goods;

use goods::Goods;
use pm::PM;

use crate::parser::Tree;


pub struct Data {
    pms: HashMap<String, PM>,
    goods: Goods,
}

impl Data {
    pub fn new(pms_tree: Tree, goods_tree: Tree) -> Result<Data, String> {

        let mut pms = HashMap::new();
        let goods = Goods::from_tree(goods_tree)?;
        for child in pms_tree {
            let pm = PM::from_tree(&child, &goods)?;
            pms.insert(pm.name().clone(), pm);
        }
        Ok(Data { pms, goods } )
    }

    pub fn get_pm(&self, name: &str) -> Option<&PM> {
        self.pms.get(name)
    }
}