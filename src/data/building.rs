use std::collections::HashMap;

use crate::Tree;

use super::{Attribute::{self, *}, Data};

#[derive(Debug)]
pub struct Building {
    name: String,
    pmgs: Vec<String>,
    cost: f32,
}

#[derive(Debug)]
pub struct EfficiencyData {
    name: String,
    input: f32,
    output: f32,
    labor: f32,
    cost: f32
}

impl Building {
    pub fn from_tree(tree: &Tree, building_values: &HashMap<String, f32>) -> Result<Self, String> {
        let name = tree.get_name();
        let pmgs = tree.get("production_method_groups")?.get_children_names();
        let value =  { 
            if let Ok(constr_val) = tree.get("required_construction") {
                constr_val.value().unwrap_or("0.0".to_owned())
            } else {
                "0.0".to_owned()
            }
        };
        let cost = match value.parse::<f32>() {
            Ok(cost) => {
                cost
            }
            Err(_) => {
                *building_values.get(&value).unwrap_or(&0.)
            }
        };
        Ok(Building { name, pmgs, cost } )
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn get_default_data(&self, data: &Data) -> Option<EfficiencyData> {
        let mut input = 0.;
        let mut output = 0.;
        let mut labor = 0.;
        for pmg in &self.pmgs {
            if let Some(pmg) = data.get_pmg(pmg) {
                if let Some(pm) = pmg.get_default(data) {
                    if let Some(pm) = data.get_pm(&pm) {
                        input += pm.get(Input);
                        output += pm.get(Output);
                        labor += pm.get(Labor);
                    }
                }
            }
        }
        Some(EfficiencyData { name: self.name().clone(), input, output, labor, cost: self.cost })
    }

    pub fn get_pm_data(&self, data: &Data, pm_name: &str) -> Option<EfficiencyData> {
        let mut input = 0.;
        let mut output = 0.;
        let mut labor = 0.;
        for pmg in &self.pmgs {
            if let Some(pmg) = data.get_pmg(pmg) {
                let pm;
                if pmg.get_pms().contains(&pm_name.to_owned()) {
                    pm = pm_name.to_owned();
                } else {
                    pm = pmg.get_default(data)?;
                }
                if let Some(pm) = data.get_pm(&pm) {
                    input += pm.get(Input);
                    output += pm.get(Output);
                    labor += pm.get(Labor);
                }
            }
        }
        Some(EfficiencyData { name: self.name().clone(), input, output, labor, cost: self.cost })
    }

    pub fn get_pm_names(&self, data: &Data) -> Vec<String> {
        // combines all the pmgs into a single vector
        // by taking their get_pms() and flattening the result
        self.pmgs.iter().filter_map(|pmg| data.get_pmg(pmg).map(|pmg| pmg.get_pms())).flatten().collect()
    }
}

impl EfficiencyData {
    pub fn get(&self, attribute: Attribute) -> f32 {
        match attribute {
            Input => self.input,
            Output => self.output,
            Labor => self.labor,
            Construction => self.cost,
            EfficiencyPerWorker => (self.output - self.input) / self.labor,
            NetOutput => self.output - self.input,
            EfficiencyPerConstruction => (self.output - self.input) / self.cost,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl ToString for EfficiencyData {
    fn to_string(&self) -> String {
        format!("{}: Net Value: {:.2}, Efficiency per ten construction: {:.2}, Efficiency per hundred workers: {:.2}", self.name, self.get(NetOutput), self.get(EfficiencyPerConstruction) * 10.0, self.get(EfficiencyPerWorker) * 100.0)
    }
}