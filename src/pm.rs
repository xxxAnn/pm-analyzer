pub type Number = i32;

use crate::{goods::Goods, parser::Tree};

pub enum Attribute {
    Name,
    Input,
    Output,
    Construction,
    Labor,
    EfficiencyPerWorker,
    NetOutput,
    EfficiencyPerConstruction
}

#[derive(Debug)]
pub struct PM {
    name: String,
    input: Number,
    output: Number,
    labor: Number,
}

impl PM {
    pub fn from_tree(tree: Tree, goods: Goods) -> Result<Self, String> {

        let building_modifiers = tree.get("building_modifiers")?;
        let level_scaled = building_modifiers.get("level_scaled")?;
        let worker_scaled = building_modifiers.get("workforce_scaled")?;

        let mut input_cost = 0;
        for input in worker_scaled.get_children_names_filtered(|name| name.starts_with("goods_input_")) {            
            input_cost += goods.get_cost(&input.replace("goods_input_", "").replace("_add", "")) 
               * worker_scaled.get(input.clone())?.value()?.parse::<i32>().unwrap();
        }

        let mut outpust_cost = 0;
        for input in worker_scaled.get_children_names_filtered(|name| name.starts_with("goods_output_")) {
            outpust_cost += goods.get_cost(&input.replace("goods_output_", "").replace("_add", "")) 
               * worker_scaled.get(input.clone())?.value()?.parse::<i32>().unwrap();
        }

        let mut labor = 0;
        for input in level_scaled.get_children_names_filtered(|name| name.starts_with("building_employment_")) {
            labor += level_scaled.get(input.clone())?.value()?.parse::<i32>().unwrap();
        }

        return Ok(PM::new(tree.get_name(), input_cost, outpust_cost, labor));
    }
}

impl PM {

    pub fn new(name: String, input: Number, output: Number, labor: Number) -> PM {
        PM {
            name,
            input,
            output,
            labor,
        }
    }

    pub fn name(&self) -> &String {
        self.get_name()
    }

    pub fn get(&self, key: Attribute) -> Number {
        match key {
            Attribute::Input => self.get_input(),
            Attribute::Output => self.get_output(),
            Attribute::Labor => self.get_labor(),
            Attribute::NetOutput => self.get_net_output(),
            Attribute::EfficiencyPerConstruction => self.get_efficiency_per_construction(),
            Attribute::EfficiencyPerWorker => self.get_efficiency_per_worker(),
            _ => panic!("Invalid attribute")
        }
    }
} 

// Private impl
impl PM {

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_input(&self) -> Number {
        self.input
    }

    fn get_output(&self) -> Number {
        self.output
    }

    fn get_labor(&self) -> Number {
        self.labor
    }
    
    fn get_net_output(&self) -> Number {
        self.get_output() - self.get_input()
    }

    fn get_efficiency_per_worker(&self) -> Number {
        todo!() // Requires the whole building tree
    }

    fn get_efficiency_per_construction(&self) -> Number {
        todo!() // Requires the whole building tree
    }
}

