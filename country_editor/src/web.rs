use std::path::{PathBuf, Path};

use rocket::{fs::NamedFile, State};
use rocket::serde::json::Json;

use crate::{consts, dds_converter};
use crate::data::Data;
use crate::scanner::Scanner;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("web/index.html").await.ok()
}

#[get("/resources/<file..>")]
pub async fn get_resource(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("resources/").join(file)).await.ok()
}

#[get("/countryname/<country>")]
pub async fn get_country_name(country: String, scanner: &State<Scanner>) -> Json<String> {
    Json(scanner.get_country_display_name(country))
}

#[get("/defaultstate")]
pub async fn default_state(data: &State<Data>, scanner: &State<Scanner>) -> Json<String> {
    // First select a random country
    let country = data.get_countries().into_iter().next().unwrap();
    // We want to create JSON like this:
    /*
    {
        "countries": [...List of country names...],
        "laws": {
            "group1": {
                "law1": {
                    "name": "law1",
                    "texture": "texture_path",
                },
                "law2": {
                    "name": "law2",
                    "texture": "texture_path",
                }
            },
            }
        }
     */

    let country_names = data.get_countries().into_iter().map(|s| s.get_name().to_owned()).collect::<Vec<String>>();
    let mut laws = std::collections::HashMap::new();
    for (col, law_category) in vec!["power_structure", "economy", "human_rights"].into_iter().enumerate() {
        for law_group in data.get_law_compendium().values() {
            if law_group.get_category() == law_category {
                let mut laws_in_group = std::collections::HashMap::new();

                for law in law_group.get_laws() {
                    let texture_path = generate_texture_path(&law.get_texture_path(), scanner);
                    laws_in_group.insert(law.get_name().to_owned(), texture_path);
                }
                laws.insert(law_group.get_name().to_string().to_owned(), (laws_in_group, col));
            }
        }
    }
    
    let default_state = serde_json::json!({
        "countries": country_names,
        "laws": laws
    });

    Json(default_state.to_string())
}

fn generate_texture_path(texture_path: &str, scanner: &Scanner) -> String {
    // Steps: Check if the file_name in the path already exists in resource and return it if it does
    // If it doesn't try to find the resource path in either the game files or the mod files
    // Then use the DDS converter to add it to the resources
    let mut path = "resources/interventionism.png".to_owned();
    let unr_texture_path = texture_path.replace('"', "");
    let file_name = Path::new(&unr_texture_path).file_name().unwrap().to_str().unwrap().replace(".dds", "");
    if !Path::new("resources/").join(file_name.clone() + ".png").exists() {
        // Check if the file exists in the game files
        let mut root_path = None;
        if Path::new(&scanner.get_mod_path()).join(&unr_texture_path).exists() {
            root_path = Some(scanner.get_mod_path());
        } else if Path::new(&scanner.get_game_path()).join(&unr_texture_path).exists() {
            root_path = Some(scanner.get_game_path());
        }
        if root_path.is_some() {
            dds_converter::save_to_resources(format!("{}/{}", root_path.unwrap(), &unr_texture_path)).unwrap();
            path = format!("resources/{}.png", file_name.clone());
        }        
        
    } else {
        path = format!("resources/{}.png", file_name.clone());
    }

    path
}