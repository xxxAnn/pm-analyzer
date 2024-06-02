use std::collections::HashMap;

use super::law::LawGroup;

#[derive(Debug)]
pub struct Country {
    name: String,
    laws: HashMap<String, String>
}



impl Country {
    pub fn from_default(name: &str, default_laws: &HashMap<String, LawGroup>) -> Self {
        let mut laws = HashMap::new();
        for (group, law_group) in default_laws {
            for law in law_group.get_laws() {
                laws.insert(group.clone(), law.get_name().to_string());
            }
        }
        Country {
            name: name.to_string(),
            laws
        }
    }
    pub fn set_law(&mut self, law: &str, group: &str) {
        self.laws.insert(group.to_string(), law.to_string());
    }
}

