use std::collections::HashMap;


mod pm;
mod goods;
mod pmg;
mod building;

use building::Building;
use goods::Goods;
use pm::PM;
use pmg::PMG;

use crate::parser::Tree;

pub enum Attribute {
    Input,
    Output,
    Construction,
    Labor,
    EfficiencyPerWorker,
    NetOutput,
    EfficiencyPerConstruction
}


pub struct Data {
    pms: HashMap<String, PM>,
    pmgs: HashMap<String, PMG>,
    buildings: HashMap<String, Building>,
    goods: Goods,
}

impl Data {
    pub fn new(pms_tree: Tree, goods_tree: Tree, pmgs_tree: Tree, buildings_tree: Tree, building_values_tree: Tree) -> Result<Data, String> {

        let mut pms = HashMap::new();
        let mut pmgs = HashMap::new();
        let mut buildings = HashMap::new();

        let goods = Goods::from_tree(goods_tree)?;

        for child in pms_tree {
            match PM::from_tree(&child, &goods) {
                Ok(pm) => { pms.insert(pm.name().clone(), pm); },
                Err(e) => { println!("Error parsing {}: {}", child.get_name(), e) }
            }
        }
        for child in pmgs_tree {
            match PMG::from_tree(&child) {
                Ok(pmg) => { pmgs.insert(pmg.name().clone(), pmg); },
                Err(e) => { println!("Error parsing {}: {}", child.get_name(), e) }
            }
        }
        let building_values = Data::get_building_values(&building_values_tree);
        for child in buildings_tree {
            match Building::from_tree(&child, &building_values) {
                Ok(building) => { buildings.insert(building.name().clone(), building); },
                Err(e) => { println!("Error parsing {}: {}", child.get_name(), e) }
            }
        }
        Ok(Data { pms, goods, pmgs, buildings } )
    }

    fn get_building_values(tree: &Tree) -> HashMap<String, f32> {
        let mut values = HashMap::new();
        for child in tree.clone() {
            values.insert(child.get_name().clone(), child.value().unwrap().parse::<f32>().unwrap());
        }
        values
    }

    pub fn get_pm(&self, name: &str) -> Option<&PM> {
        self.pms.get(name)
    }

    pub fn get_pmg(&self, name: &str) -> Option<&PMG> {
        self.pmgs.get(name)
    }

    pub fn get_building(&self, name: &str) -> Option<&Building> {
        self.buildings.get(name)
    }

    pub fn get_cost(&self, key: &str) -> i32 {
        self.goods.get_cost(key)
    }
}