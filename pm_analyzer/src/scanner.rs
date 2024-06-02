use std::{ffi::OsString, fs, io::Write, path::PathBuf};
use vic3_parser::{Parser, Tree, utils::*};
use crate::data::Data;

pub fn scan() -> Result<Data, String> { 
    let (mod_path, game_path) = get_scan_paths();


    let pm_files = get_paths(&mod_path, &game_path, r#"\common\production_methods"#);
    let goods_files = get_paths(&mod_path, &game_path, r#"\common\goods"#);
    let pmg_files = get_paths(&mod_path, &game_path, r#"\common\production_method_groups"#);
    let buildings_files = get_paths(&mod_path, &game_path, r#"\common\buildings"#);

    let pm_tree = generate_tree(pm_files);

    let goods_tree = generate_tree(goods_files);

    let pmg_tree = generate_tree(pmg_files);

    let buildings_tree = generate_tree(buildings_files);

    let building_values = format!("{}{}",  { if std::path::Path::new(&format!("{}{}", mod_path, r#"\common\script_values\building_values.txt"#)).exists() {
        mod_path
    } else {
        game_path
    } }, r#"\common\script_values\building_values.txt"#);
    let building_values_tree = parse_file(&building_values);

    Ok(Data::new(pm_tree, goods_tree, pmg_tree, buildings_tree, building_values_tree)?)
}

