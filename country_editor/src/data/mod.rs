use std::collections::HashMap;

pub use country::{Country, LawSetBy};
pub use law::{Law, LawGroup, ScriptedEffectLawsTemplate};
use vic3_parser::Tree;

mod law;
mod country;

pub struct Data {
    countries: HashMap<String, Country>,
    laws: HashMap<String, Law>,
    law_groups: HashMap<String, LawGroup>,
    scripted_effects: HashMap<String, ScriptedEffectLawsTemplate>
}

impl Data {
    pub fn get_country(&self, name: &str) -> Option<&Country> {
        self.countries.get(name)
    }

    pub fn new_template(&self, name: &str) -> ScriptedEffectLawsTemplate {
        ScriptedEffectLawsTemplate::from_default(&self.law_groups)
    }

    pub fn insert_or_overwrite_template(&mut self, name: &str, template: ScriptedEffectLawsTemplate) {
        self.scripted_effects.insert(name.to_string(), template);
    }

    pub fn generate_templates_tree(&self) -> Tree {
        let mut tree = Tree::default();
        let mut highest_id = 98;
        for (name, template) in &self.scripted_effects {
            let mut template_tree = Tree::with_named_root(&name);
            for (law, group) in template.get_laws() {
                template_tree.add_child(Tree::from_key_value("activate_law", &format!("law:{}", law), highest_id));
                highest_id += 1;
            }
            tree.add_child_tree(template_tree);
        }
        tree
    }

    pub fn get_law(&self, name: &str) -> Option<&Law> {
        self.laws.get(name)
    }

    pub fn get_law_group(&self, name: &str) -> Option<&LawGroup> {
        self.law_groups.get(name)
    }

    pub fn get_scripted_effect(&self, name: &str) -> Option<&ScriptedEffectLawsTemplate> {
        self.scripted_effects.get(name)
    }

    pub fn new(countries: HashMap<String, Country>, laws: HashMap<String, Law>, law_groups: HashMap<String, LawGroup>, scripted_effects: HashMap<String, ScriptedEffectLawsTemplate>) -> Self {
        Data {
            countries,
            laws,
            law_groups,
            scripted_effects
        }
    }
}