use crate::Tree;
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
}