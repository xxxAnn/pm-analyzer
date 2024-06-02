use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Law {
    name: String,
    group: String
}

impl Law {
    pub fn new(name: String, group: String) -> Self {
        Law {
            name,
            group
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_group(&self) -> &str {
        &self.group
    }
}

#[derive(Debug)]
pub struct LawGroup {
    name: String,
    category: String,
    laws: Vec<Law>
}

#[derive(Debug)]
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
    pub fn new(name: String, category: String) -> Self {
        LawGroup {
            name,
            category,
            laws: Vec::new()
        }
    }

    pub fn push(&mut self, law: Law) {
        self.laws.push(law);
    }

    pub fn get_laws(&self) -> &Vec<Law> {
        &self.laws
    }
}
