use vic3_parser::{Parser, Tree, utils::*};

pub fn scan() { 
    let (mod_path, game_path) = get_scan_paths();

    let scripted_effects = get_paths(&mod_path, &game_path, r#"\common\scripted_effects"#);

    todo!();
}