use std::{collections::{btree_map::Entry, HashMap}, default};

use vic3_parser::{Parser, Tree, utils::*};

struct Country {
    laws: HashMap<String, String>
}

#[derive(Debug)]
struct Law {
    name: String,
    group: String
}

#[derive(Debug)]
struct LawGroup {
    name: String,
    category: String,
    laws: Vec<Law>
}

impl LawGroup { 
    fn new(name: String, category: String) -> Self {
        LawGroup {
            name,
            category,
            laws: Vec::new()
        }
    }

    fn push(&mut self, law: Law) {
        self.laws.push(law);
    }
}

pub fn scan() -> Result<(), String>{ 
    let (mod_path, game_path) = get_scan_paths();

    let scripted_effect_files = vec![mod_path.clone() + r#"\common\scripted_effects\template_automated_law_setup.txt"#];
    let scripted_effect_trees = generate_tree(scripted_effect_files);

    let country_files = get_paths(&mod_path, &game_path, r#"\common\history\countries"#);
    let country_trees = generate_trees(country_files);
    let country_tree = merge_trees(&country_trees.into_iter().filter_map(|tree| tree.get("COUNTRIES").ok()).collect::<Vec<Tree>>());

    let law_files = get_paths(&mod_path, &game_path, r#"\common\laws"#);
    let law_tree = generate_tree(law_files);

    let law_group_files = get_paths(&mod_path, &game_path, r#"\common\law_groups"#);
    let law_group_tree = generate_tree(law_group_files);
    let mut law_group_categories = HashMap::new();
    for child in law_group_tree {
        if let Ok(category_tree) = child.get("law_group_category") {
            if let Ok(category) = category_tree.value() {
                law_group_categories.insert(child.get_name(), category.clone());
            }
        }
    }

    let mut default_laws = HashMap::new();
    
    for child in law_tree {
        if let Ok(group_tree) = child.get("group") {
            if let Ok(group) = group_tree.value() {
                let law = Law {
                    name: child.get_name(),
                    group: group.clone()
                };
                
                default_laws.entry(group.clone()).or_insert(LawGroup::new(group.clone(), law_group_categories.get(&group).unwrap_or(&"N/A".to_owned()).clone())).push(law);
            }
        }
    }

    dbg!(default_laws);
    todo!();
}