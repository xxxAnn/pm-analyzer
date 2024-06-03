// Provides data for a website
// That lets you modify the country files of Victoria 3
mod scanner;
mod data;
mod consts;

use data::Law;
use dds_converter::save_to_resources;
use scanner::Scanner;
use vic3_parser::utils;
mod dds_converter;

fn main() {
    save_to_resources(r#"C:\Users\annma\Documents\Games\Victoria.3.v1.6.2\game\gfx\interface\icons\law_icons\interventionism.dds"#).expect("Failed to save");  
}

fn test() {
    let scanner = Scanner::new();

    let mut data = scanner.scan().expect("Failed to scan");


     // EXPOSED API SHOULD HAVE
     // TEMPLATES: CREATE !, EDIT, GET !
     // COUNTRIES: APPLY TEMPLATE !, SET LAW !, GET !
     // INTERNAL API SHOULD BE ABLE TO WRITE COUNTRIES AND TEMPLATES BACK TO FILES !

    data.get_country_mut("SWE").expect("Failed to get country").set_law_manual("law_racial_segregation", "lawgroup_citizenship");

    let mut template = data.new_template();
    template.push(Law::new("law_appointed_bureaucrats", "lawgroup_bureaucracy"));

    //let tree = utils::generate_tree(vec![r#"C:\Users\annma\Documents\Paradox Interactive\Victoria 3\mod\CWP-Main\common\history\countries\swe - sweden.txt"#.into()]);
    //println!("{}", tree.to_string());
    data.insert_or_overwrite_template("test_template", template);
    
    data.apply_template_to_country("test_template", "SWE").expect("Failed to apply template");

    
    data.country_to_tree(&scanner, "SWE").write().expect("Failed to write country"); 
    data.generate_templates_tree(&scanner).write().expect("Failed to write templates"); 
}
