use std::collections::HashMap;


mod pm;
mod goods;
mod pmg;

use goods::Goods;
use pm::PM;
use pmg::PMG;

use crate::parser::Tree;


pub struct Data {
    pms: HashMap<String, PM>,
    pmgs: HashMap<String, PMG>,
    goods: Goods,
}

impl Data {
    pub fn new(pms_tree: Tree, goods_tree: Tree, pmgs_tree: Tree) -> Result<Data, String> {

        let mut pms = HashMap::new();
        let mut pmgs = HashMap::new();
        let goods = Goods::from_tree(goods_tree)?;
        for child in pms_tree {
            let pm = PM::from_tree(&child, &goods)?;
            pms.insert(pm.name().clone(), pm);
        }
        for child in pmgs_tree {
            let pmg = PMG::from_tree(&child)?;
            pmgs.insert(pmg.name().clone(), pmg);
        }
        Ok(Data { pms, goods, pmgs } )
    }

    pub fn get_pm(&self, name: &str) -> Option<&PM> {
        self.pms.get(name)
    }
}