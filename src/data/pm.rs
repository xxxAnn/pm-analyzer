use super::goods::Goods;
use crate::parser::Tree;

use super::Attribute;

#[derive(Debug)]
pub struct PM {
    name: String,
    input: f32,
    output: f32,
    labor: f32,
    is_default: bool,
}

impl PM {
    pub fn from_tree(tree: &Tree, goods: &Goods) -> Result<Self, String> {

        let building_modifiers = tree.get("building_modifiers")?;
        let level_scaled = building_modifiers.get("level_scaled")?;
        let worker_scaled = building_modifiers.get("workforce_scaled")?;

        let mut input_cost = 0.;
        for input in worker_scaled.get_children_names_filtered(|name| name.starts_with("goods_input_")) {            
            input_cost += goods.get_cost(&input.replace("goods_input_", "").replace("_add", "")) as f32
               * worker_scaled.get(input.clone())?.value()?.parse::<f32>().map_err(|_| "Error parsing output cost")?;
        }

        let mut outpust_cost = 0.;
        for input in worker_scaled.get_children_names_filtered(|name| name.starts_with("goods_output_")) {
            outpust_cost += goods.get_cost(&input.replace("goods_output_", "").replace("_add", "")) as f32
               * worker_scaled.get(input.clone())?.value()?.parse::<f32>().map_err(|_| "Error parsing output cost")?;
        }

        let mut labor = 0.;
        for input in level_scaled.get_children_names_filtered(|name| name.starts_with("building_employment_")) {
            labor += level_scaled.get(input.clone())?.value()?.parse::<f32>().map_err(|_| "Error parsing output cost")?;
        }

        let mut is_default = false; 
        if let Ok(is_default_str) = tree.get("is_default") {
            if let Ok(is_default_value) = is_default_str.value() {
                is_default = is_default_value == "yes";
            }
        }

        return Ok(PM::new(tree.get_name(), input_cost, outpust_cost, labor, is_default));
    }
}

impl PM {

    pub fn new(name: String, input: f32, output: f32, labor: f32, is_default: bool) -> PM {
        PM {
            name,
            input,
            output,
            labor,
            is_default
        }
    }

    pub fn name(&self) -> &String {
        self.get_name()
    }

    pub fn get(&self, key: Attribute) -> f32 {
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

    pub fn default(&self) -> bool {
        self.is_default()
    }
} 

// Private impl
impl PM {

    fn get_name(&self) -> &String {
        &self.name
    }

    fn is_default(&self) -> bool {
        self.is_default
    }

    fn get_input(&self) -> f32 {
        self.input
    }

    fn get_output(&self) -> f32 {
        self.output
    }

    fn get_labor(&self) -> f32 {
        self.labor
    }
    
    fn get_net_output(&self) -> f32 {
        self.get_output() - self.get_input()
    }

    fn get_efficiency_per_worker(&self) -> f32 {
        todo!() // Requires the whole building tree
    }

    fn get_efficiency_per_construction(&self) -> f32 {
        todo!() // Requires the whole building tree
    }
}

