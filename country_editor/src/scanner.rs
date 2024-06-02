use std::collections::HashMap;
use vic3_parser::{Tree, utils::*};

use crate::data::{Country, Data, Law, LawGroup, ScriptedEffectLawsTemplate};

pub fn scan() -> Result<Data, String> {
    let (mod_path, game_path) = get_scan_paths();

    let scripted_effect_tree = generate_scripted_effect_tree(&mod_path);
    let country_tree = generate_country_tree(&mod_path, &game_path);
    let law_tree = generate_law_tree(&mod_path, &game_path);
    let law_group_categories = generate_law_group_categories(&mod_path, &game_path);

    let (law_compendium, default_laws) = generate_law_compendium(&law_tree, &law_group_categories);
    let scripted_effects = generate_scripted_effects(&scripted_effect_tree, &law_compendium);

    let countries = generate_countries(&country_tree, &law_compendium, &default_laws, &scripted_effects);

    Ok(Data::new(countries, law_compendium, default_laws, scripted_effects))
}

fn generate_scripted_effect_tree(mod_path: &str) -> Tree {
    let scripted_effect_files = vec![mod_path.to_string() + r#"\common\scripted_effects\template_automated_law_setup.txt"#];
    generate_tree(scripted_effect_files)
}

fn generate_country_tree(mod_path: &str, game_path: &str) -> Tree {
    let country_files = get_paths(mod_path, game_path, r#"\common\history\countries"#);
    let country_trees = generate_trees(country_files);

    let valid_trees: Vec<Tree> = country_trees
        .into_iter()
        .filter_map(|tree| {
            tree.get("COUNTRIES").ok().and_then(|t| {
                t.move_up("COUNTRIES").search_child(|s| s.starts_with("c:"))
            })
        })
        .collect();

    Tree::from_children(&valid_trees, 9999)
}

fn generate_law_tree(mod_path: &str, game_path: &str) -> Tree {
    let law_files = get_paths(mod_path, game_path, r#"\common\laws"#);
    generate_tree(law_files)
}

fn generate_law_group_categories(mod_path: &str, game_path: &str) -> HashMap<String, String> {
    let law_group_files = get_paths(mod_path, game_path, r#"\common\law_groups"#);
    let law_group_tree = generate_tree(law_group_files);
    let mut law_group_categories = HashMap::new();
    for child in law_group_tree {
        if let Ok(category_tree) = child.get("law_group_category") {
            if let Ok(category) = category_tree.value() {
                law_group_categories.insert(child.get_name(), category.clone());
            }
        }
    }
    law_group_categories
}

fn generate_law_compendium(
    law_tree: &Tree,
    law_group_categories: &HashMap<String, String>,
) -> (HashMap<String, Law>, HashMap<String, LawGroup>) {
    let mut law_compendium = HashMap::new();
    let mut default_laws = HashMap::new();

    for child in law_tree.clone() {
        if let Ok(group_tree) = child.get("group") {
            if let Ok(group) = group_tree.value() {
                let law = Law::new(
                    child.get_name(),
                    group.clone()
                );
                law_compendium.insert(child.get_name(), law.clone());
                default_laws
                    .entry(group.clone())
                    .or_insert(LawGroup::new(group.clone(), law_group_categories.get(&group).unwrap_or(&"N/A".to_owned()).clone()))
                    .push(law);
            }
        }
    }
    (law_compendium, default_laws)
}

fn generate_scripted_effects(
    scripted_effect_tree: &Tree,
    law_compendium: &HashMap<String, Law>,
) -> HashMap<String, ScriptedEffectLawsTemplate> {
    let mut scripted_effects = HashMap::new();
    for child in scripted_effect_tree.clone() {
        let mut new_scripted_effect_laws_template = ScriptedEffectLawsTemplate::new();
        let child_name = child.get_name();
        for activate_law in child.into_iter_filtered(|s| s.starts_with("activate_law")) {
            if let Ok(raw_law) = activate_law.value() {
                let law = raw_law.split(':').last().unwrap().trim().to_owned();
                let mut group = "N/A".to_owned();
                if let Some(law_in_compendium) = law_compendium.get(&law) {
                    group = law_in_compendium.get_group().clone().to_string();
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
            for law in child.clone().into_iter_filtered(|s| s.starts_with("activate_law")) {
                if let Ok(raw_law) = law.value() {
                    let law = raw_law.split(':').last().unwrap().trim().to_owned();
                    if let Some(res) = law_compendium.get(&law) {
                        new_country.set_law(&law, &res.get_group());
                    }
                }
            }

            // Handle scripted effects
            for scripted_effect in child.into_iter_filtered(|s| !s.starts_with("activate_law")) {
                let scripted_effect_name = scripted_effect.get_name();
                if let Some(scripted_effect_template) = scripted_effects.get(&scripted_effect_name) {
                    for (law, group) in scripted_effect_template.get_laws() {
                        new_country.set_law(law, group);
                    }
                }
            }
            countries.insert(country_name.to_string(), new_country);
        }
    }
    countries
}
