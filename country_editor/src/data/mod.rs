use std::collections::HashMap;

pub use country::Country;
pub use law::{Law, LawGroup, ScriptedEffectLawsTemplate};

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