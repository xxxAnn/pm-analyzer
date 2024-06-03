use std::collections::HashMap;
use vic3_parser::{Tree, utils::*};

use crate::{consts, data::{Country, Data, Law, LawGroup, LawSetBy, ScriptedEffectLawsTemplate}};

pub struct Scanner {
    mod_path: String,
    game_path: String
}

impl Scanner {
    pub fn new() -> Self {
        let (mod_path, game_path) = get_scan_paths();
        Scanner {
            mod_path,
            game_path
        }
    }

    pub fn get_mod_path(&self) -> String {
        self.mod_path.clone()
    }

    pub fn scan(&self) -> Result<Data, String> {
        let scripted_effect_tree = self.generate_scripted_effect_tree();
        let country_tree = self.generate_country_tree();
        let law_tree = self.generate_law_tree();
        let law_group_categories = self.generate_law_group_categories();

        let (law_compendium, default_laws) = self.generate_law_compendium(&law_tree, &law_group_categories);
        let scripted_effects = self.generate_scripted_effects(&scripted_effect_tree, &law_compendium);

        let countries: HashMap<String, Country> = self.generate_countries(&country_tree, &law_compendium, &default_laws, &scripted_effects);

        Ok(Data::new(countries, law_compendium, default_laws, scripted_effects))
    }

    fn generate_scripted_effect_tree(&self) -> Tree {
        let scripted_effect_files = vec![self.mod_path.clone() + consts::TEMPLATE_FILE];        
        generate_tree(scripted_effect_files)
    }

    pub fn countries_per_file(&self) -> Vec<(String, Tree)> { // Path of file & Tree with COUNTRIES moved up to root 
        let country_files = get_paths(&self.mod_path, &self.mod_path, consts::COUNTRIES_FOLDER);
        let country_trees = generate_trees_and_remember_files(country_files);

        country_trees.into_iter().filter_map(|(file_name, tree)| {
            Some((file_name, tree.move_up(consts::COUNTRIES_TAG)))
        }).collect()
    }
    fn generate_country_tree(&self) -> Tree {
        let country_files = get_paths(&self.mod_path, &self.mod_path, consts::COUNTRIES_FOLDER);
        let binding = generate_trees_and_remember_files(country_files);

        for (x, y) in &binding {
            //println!("{}", x);
            if x.contains("swe") || x.contains("can") {
                //println!("{}: {}", x, y.to_string());
                //println!("{}", y.move_up(consts::COUNTRIES_TAG).to_string());
                //println!("{:?}", y.move_up(consts::COUNTRIES_TAG).search_child(|s| s.contains("c:") ));
            }
        }


        let valid_trees: Vec<Tree> = binding.into_iter().map(|(_, tree)| tree)
            .filter_map(|tree| {
                tree.move_up(consts::COUNTRIES_TAG).search_child(|s| {
                    s.contains("c:")
                })
            })
            .collect();

        Tree::from_children(&valid_trees, 9999)
    }

    fn generate_law_tree(&self) -> Tree {
        let law_files = get_paths(&self.mod_path, &self.game_path, consts::LAWS_FOLDER);
        generate_tree(law_files)
    }

    fn generate_law_group_categories(&self) -> HashMap<String, String> {
        let law_group_files = get_paths(&self.mod_path, &self.game_path, consts::LAWS_GROUP_FOLDER);
        let law_group_tree = generate_tree(law_group_files);
        let mut law_group_categories = HashMap::new();
        for child in law_group_tree {
            if let Ok(category_tree) = child.get(consts::LAW_GROUP_CATEGORY_TAG) {
                if let Ok(category) = category_tree.value() {
                    law_group_categories.insert(child.get_name(), category.clone());
                }
            }
        }
        law_group_categories
    }

    fn generate_law_compendium(
        &self,
        law_tree: &Tree,
        law_group_categories: &HashMap<String, String>,
    ) -> (HashMap<String, Law>, HashMap<String, LawGroup>) {
        let mut law_compendium = HashMap::new();
        let mut default_laws = HashMap::new();

        for child in law_tree.clone() {
            if let Ok(group_tree) = child.get(consts::LAW_GROUP_TAG) {
                if let Ok(group) = group_tree.value() {
                    if let Some(law_name) = child.get_name().split(':').last() {
                        let law = Law::new(
                            law_name.to_string(),
                            group.clone()
                        );
                        law_compendium.insert(law_name.to_string(), law.clone());
                        default_laws
                            .entry(group.clone())
                            .or_insert(LawGroup::new(group.clone(), law_group_categories.get(&group).unwrap_or(&"N/A".to_owned()).clone()))
                            .push(law);
                    }
                }
            }
        }
        (law_compendium, default_laws)
    }

    fn generate_scripted_effects(
        &self,
        scripted_effect_tree: &Tree,
        law_compendium: &HashMap<String, Law>,
    ) -> HashMap<String, ScriptedEffectLawsTemplate> {
        let mut scripted_effects = HashMap::new();
        for child in scripted_effect_tree.clone() {
            let mut new_scripted_effect_laws_template = ScriptedEffectLawsTemplate::new();
            let child_name = child.get_name();
            for activate_law in child.into_iter_filtered(|s| s.starts_with(consts::ACTIVATE_LAW_TAG)) {
                if let Ok(raw_law) = activate_law.value() {
                    let law = raw_law.split(':').last().unwrap().trim().to_owned();
                    let mut group = "N/A".to_owned();
                    if let Some(law_in_compendium) = law_compendium.get(&law) {
                        group = law_in_compendium.get_group().to_string();
                    }
                    new_scripted_effect_laws_template.push(Law::new(
                        law.clone(),
                        group
                    ));
                }
            }
            scripted_effects.insert(child_name, new_scripted_effect_laws_template);
        }
        scripted_effects
    }

    fn generate_countries(
        &self,
        country_tree: &Tree,
        law_compendium: &HashMap<String, Law>,
        default_laws: &HashMap<String, LawGroup>,
        scripted_effects: &HashMap<String, ScriptedEffectLawsTemplate>,
    ) -> HashMap<String, Country> {
        let mut countries = HashMap::new();
        for child in country_tree.clone() {
            if let Some(country_name) = child.clone().get_name().split(':').last() {

                let mut new_country = Country::from_default(country_name, default_laws);

                // Handle raw law activation
                for law in child.clone().into_iter_filtered(|s| s.starts_with(consts::ACTIVATE_LAW_TAG)) {
                    if let Ok(raw_law) = law.value() {
                        if let Some(law) = raw_law.split(':').last().map(|s| s.trim().to_owned()) {
                            if let Some(res) = law_compendium.get(&law) {
                                new_country.set_law(&law,  &res.get_group(), LawSetBy::Manual);
                            }
                        }
                    }
                }

                // Handle scripted effects
                for scripted_effect in child.into_iter_filtered(|s| !s.starts_with(consts::ACTIVATE_LAW_TAG)) {
                    let scripted_effect_name = scripted_effect.get_name();
                    if let Some(scripted_effect_template) = scripted_effects.get(&scripted_effect_name) {
                        for (law, group) in scripted_effect_template.get_laws() {
                            new_country.set_law(law, group, LawSetBy::ScriptedEffect(scripted_effect_name.to_string()));
                        }
                    }
                }
                countries.insert(country_name.to_string(), new_country);
            }
        }
        countries
    }
}
