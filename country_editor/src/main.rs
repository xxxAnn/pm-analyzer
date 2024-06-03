// Provides data for a website
// That lets you modify the country files of Victoria 3
mod scanner;
mod data;

use scanner::Scanner;


fn main() {
    let scanner = Scanner::new();

    let data = scanner.scan().expect("Failed to scan");

    let country = data.get_country("SWE").expect("SWE not found"); 

     // EXPOSED API SHOULD HAVE
     // TEMPLATES: CREATE !, EDIT, GET !
     // COUNTRIES: APPLY TEMPLATE !, SET LAW !, GET !
     // INTERNAL API SHOULD BE ABLE TO WRITE COUNTRIES AND TEMPLATES BACK TO FILES
    
    println!("{}", country.to_tree(&scanner, &data).serialize()); 
    println!("{}", data.generate_templates_tree().serialize()); 
}
