use std::{cell::RefCell, collections::HashMap, io::Write, rc::Rc};

pub use country::{Country, LawSetBy};
pub use law::{Law, LawGroup, ScriptedEffectLawsTemplate};
use vic3_parser::Tree;

use crate::{consts, scanner::Scanner};

mod law;
mod country;

pub struct Data {
    countries: HashMap<String, Country>,
    laws: HashMap<String, Law>,
    law_groups: HashMap<String, LawGroup>,
    scripted_effects: HashMap<String, ScriptedEffectLawsTemplate>
}

pub struct WriteAction { 
    file: String,
    is_country: IsCountry,
    content: Tree
}

pub enum IsCountry {
    Yes(String),
    No
}

impl WriteAction {
    pub fn new(file: &str, content: Tree, is_country: IsCountry) -> Self {
        WriteAction {
            file: file.to_string(),
            is_country,
            content
        }
    }

    pub fn get_file(&self) -> &str {
        &self.file
    }

    pub fn get_content(&self) -> &Tree {
        &self.content
    }

    // Consumes the WriteAction and writes the content to the file

    pub fn write(self) -> std::io::Result<()> {
        let mut file = std::fs::File::create(&self.file)?;
        let mut serialized = self.content.serialize();
        if let IsCountry::Yes(country) = self.is_country {
            serialized = serialized.replace(&country, &country.replace("=", "?="));
        }
        write!(file, "{}", serialized)?;
        Ok(())
    }
}

impl Data {
    pub fn get_country(&self, name: &str) -> Option<&Country> {
        self.countries.get(name)
    }

    pub fn new_template(&self) -> ScriptedEffectLawsTemplate {
        ScriptedEffectLawsTemplate::new()
    }

    pub fn insert_or_overwrite_template(&mut self, name: &str, template: ScriptedEffectLawsTemplate) {
        self.scripted_effects.insert(name.to_string(), template);
    }

    pub fn get_country_mut(&mut self, name: &str) -> Option<&mut Country> {
        self.countries.get_mut(name)
    }

    pub fn apply_template_to_country(&mut self, template_name: &str, country_name: &str) -> Result<(), String> {
        let scripted_effect = self.get_scripted_effect(template_name).ok_or("Template not found")?.clone();
        let country = self.get_country_mut(country_name).ok_or("Country not found")?;
        country.apply_template(&scripted_effect, template_name)
    }

    pub fn generate_templates_tree(&self, scanner: &Scanner) -> WriteAction {
        let mut tree = Tree::default();
        let mut highest_id = 98;
        for (name, template) in &self.scripted_effects {
            let mut template_tree = Tree::with_named_root(&name);
            for (law, group) in template.get_laws() {
                template_tree.add_child(Tree::from_key_value(consts::ACTIVATE_LAW_TAG, &format!("law:{}", law), highest_id));
                highest_id += 1;
            }
            tree.add_child_tree(template_tree);
        }
        WriteAction::new(&(scanner.get_mod_path() + consts::TEMPLATE_FILE), tree, IsCountry::No)
    }

    pub fn country_to_tree(&self, scanner: &Scanner, country: impl Into<String>) -> WriteAction {
        let country = self.get_country(&country.into()).unwrap();
        country.to_tree(scanner, self)
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