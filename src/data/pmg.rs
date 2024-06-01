use crate::Tree;

use super::Data;

#[derive(Debug)]
pub struct PMG {
    name: String,
    pms: Vec<String>,
}

impl PMG {
    pub fn from_tree(tree: &Tree) -> Result<Self, String> {
        let name = tree.get_name();
        let pms = tree.get("production_methods")?.get_children_names();
        Ok(PMG { name, pms })
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn get_default(&self, data: &Data) -> Option<String> {
        let mut result_pm = self.pms.first()?;
        for pm in &self.pms {
            if let Some(pm) = data.get_pm(pm) {
                if pm.default() {
                    result_pm = pm.name();
                }
            }
        }
        Some(result_pm.clone())
    }

    pub fn get_pms(&self) -> Vec<String> {
        self.pms.clone()
    }
}