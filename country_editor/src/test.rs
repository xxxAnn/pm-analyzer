use crate::{data, scanner::Scanner};

fn test() {
    let scanner = Scanner::new();

    let mut data = scanner.scan().expect("Failed to scan");


     // EXPOSED API SHOULD HAVE
     // TEMPLATES: CREATE !, EDIT, GET !
     // COUNTRIES: APPLY TEMPLATE !, SET LAW !, GET !
     // INTERNAL API SHOULD BE ABLE TO WRITE COUNTRIES AND TEMPLATES BACK TO FILES !

    data.get_country_mut("SWE").expect("Failed to get country").set_law_manual("law_racial_segregation", "lawgroup_citizenship");

    let mut template: data::ScriptedEffectLawsTemplate = data.new_template();
    template.push(data.get_law(
        "law_appointed_bureaucrats"
    ).expect("Failed to get law"));

    println!("{:?}", data.get_law("law_appointed_bureaucrats"));

    //let tree = utils::generate_tree(vec![r#"C:\Users\annma\Documents\Paradox Interactive\Victoria 3\mod\CWP-Main\common\history\countries\swe - sweden.txt"#.into()]);
    //println!("{}", tree.to_string());
    data.insert_or_overwrite_template("test_template", template);
    
    data.apply_template_to_country("test_template", "SWE").expect("Failed to apply template");

    
    //data.country_to_tree(&scanner, "SWE").write().expect("Failed to write country"); 
    data.bulk_to_tree(&scanner, vec!["SWE".into()]).unwrap().into_iter().map(|s| s.write().expect("Failed to write country")).for_each(drop);
    data.generate_templates_tree(&scanner).write().expect("Failed to write templates"); 
}
