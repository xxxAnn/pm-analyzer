// Provides data for a website
// That lets you modify the country files of Victoria 3
mod scanner;
mod data;

use data::{Data, Country, LawGroup, Law, ScriptedEffectLawsTemplate};

fn main() {
    let data = scanner::scan().unwrap();

    let countries = data.get_country("USA").expect("USA not found");  
    dbg!(countries);
}
