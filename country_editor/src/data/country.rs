use std::{cell::RefCell, collections::HashMap, fmt::{self, Display, Formatter, Write}, rc::Rc, result};
use vic3_parser::Tree;

use crate::{consts, data::IsCountry, scanner::Scanner};

use super::{law::LawGroup, Data, ScriptedEffectLawsTemplate, WriteAction};

#[derive(Debug)]
pub struct Country {
    name: String,
    laws: HashMap<String, (LawSetBy, String)>,
    scripted_effects: Vec<String>
}

#[derive(Debug)]
pub enum LawSetBy {
    Default,
    ScriptedEffect(String),
    Manual
}


impl Country {
    pub fn from_default(name: &str, default_laws: &HashMap<String, LawGroup>) -> Self {
        let mut laws = HashMap::new();
        for (group, law_group) in default_laws {
            laws.insert(group.clone(), (LawSetBy::Default, law_group.get_default_law().get_name().to_string()));
        }
        Country {
            name: name.to_string(),
            laws,
            scripted_effects: Vec::new()
        }
    }
    pub fn set_law(&mut self, law: &str, group: &str, set_by: LawSetBy) {
        // If the Law is set by a scripted effect, add it to the list of scripted effects
        // Check that it is not already in the list
        if let LawSetBy::ScriptedEffect(scripted_effect) = &set_by {
            if !self.scripted_effects.contains(scripted_effect) {
                self.scripted_effects.push(scripted_effect.to_string());
            }
        }
        self.laws.insert(group.to_string(), (set_by, law.to_string()));
    }

    pub fn get_law(&self, group: &str) -> Option<String> {
        self.laws.get(group).map(|(_, law)| law.to_string())
    }

    pub fn set_law_manual(&mut self, law: &str, group: &str) {
        self.set_law(law, group, LawSetBy::Manual);
    }

    pub fn apply_template(&mut self, template: &ScriptedEffectLawsTemplate, name: &str) -> Result<(), String> {
        for (law, group) in template.get_laws() {
            self.set_law(law, group, LawSetBy::ScriptedEffect(name.into()));
        }
        Ok(())
    }

    pub fn to_tree(&self, scanner: &Scanner, data: &Data) -> WriteAction {

        println!("Country: {}", self.to_string());
        // Steps 
        // 1. Scan all the country files and find the country
        let (f, t) = scanner.countries_per_file().iter().find_map(|(file_name, tree)| {
            tree.search_child(|s| s == &format!("c:{}", self.name)).map(|t| (file_name.to_owned(), t))
        }).unwrap(); // if this panics then the country was not found in any of the files
        
        // 2. Delete all laws setting in the tree 
        let mut res = t.delete_children_filtered(|s| s.starts_with(consts::ACTIVATE_LAW_TAG) || {
            // Check if there is a scripted effect that is in the list of scripted effects in Data
            data.get_scripted_effect(s).is_some()
        });

        // 3. Add the laws to the tree
        for (group, (set_by, law)) in &self.laws {
            match set_by {
                LawSetBy::Default => {
                    // The law is set by default, so we don't need to add it to the tree
                },
                LawSetBy::ScriptedEffect(_) => {
                    // The law is set by a scripted effect, so we need to add it to the tree later
                },
                LawSetBy::Manual => {
                    res.add_child(Tree::from_key_value(consts::ACTIVATE_LAW_TAG, &format!("law:{}", law), 0));
                }
            }
        }

        for scripted_effect in &self.scripted_effects {
            if let Some(_) = data.get_scripted_effect(scripted_effect) {
                res.add_child(Tree::from_key_value(&scripted_effect, "yes", 0));
            }
        }   

        let mut result_tree = Tree::with_named_root("COUNTRIES");
        result_tree.add_child_tree(res);

        WriteAction::new(&f, result_tree, IsCountry::Yes(format!("c:{} =", self.name)))
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Country: {}", self.name)?;
        for (group, (set_by, law)) in &self.laws {
            match set_by {
                LawSetBy::Default => writeln!(f, "    {}: {} (Default)", group, law)?,
                LawSetBy::ScriptedEffect(scripted_effect) => writeln!(f, "    {}: {} (Scripted Effect: {})", group, law, scripted_effect)?,
                LawSetBy::Manual => writeln!(f, "    {}: {} (Manual)", group, law)?,
            }
        }
        Ok(())
    }
}