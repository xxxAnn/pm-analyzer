use std::{collections::HashMap, default};

#[derive(Debug, Clone)]
pub struct Law {
    name: String,
    texture_path: String,
    group: String
}

impl Law {
    pub fn new(name: impl Into<String>, group: impl Into<String>, texture_path: impl Into<String>) -> Self {
        Law {
            name: name.into(),
            group: group.into(),
            texture_path: texture_path.into()
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_group(&self) -> &str {
        &self.group
    }

    pub fn get_texture_path(&self) -> &str {
        &self.texture_path
    }
}

#[derive(Debug)]
pub struct LawGroup {
    name: String,
    category: String,
    laws: Vec<Law>
}

#[derive(Debug, Clone)]
pub struct ScriptedEffectLawsTemplate {
    laws: HashMap<String, String>
}


impl ScriptedEffectLawsTemplate {
    
    pub fn push(&mut self, law: Law) {
        self.laws.insert(law.name.clone(), law.group.clone());
    }

    pub fn get_laws(&self) -> &HashMap<String, String> {
        &self.laws
    }

    pub fn new() -> Self {
        ScriptedEffectLawsTemplate {
            laws: HashMap::new()
        }
    }
}

impl LawGroup { 
    pub fn new(name: impl Into<String>, category: impl Into<String>) -> Self {
        LawGroup {
            name: name.into(),
            category: category.into(),
            laws: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_category(&self) -> &str {
        &self.category
    }
    
    pub fn push(&mut self, law: Law) {
        self.laws.push(law);
    }

    pub fn get_laws(&self) -> &Vec<Law> {
        &self.laws
    }

    pub fn get_default_law(&self) -> &Law {
        &self.laws.last().unwrap()
    }
}
