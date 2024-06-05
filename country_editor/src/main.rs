// Provides data for a website
// That lets you modify the country files of Victoria 3
use rocket::fs::FileServer;
use rocket::fs::relative;

#[macro_use] extern crate rocket;

mod scanner;
mod data;
mod consts;
mod web;
mod test;

mod dds_converter;

#[launch]
fn rocket() -> _ {

    let scanner = scanner::Scanner::new();
    let data = scanner.scan().expect("Failed to scan");

    rocket::build()
        .manage(data)
        .manage(scanner)        
        .mount("/", routes![web::index, web::get_resource])
        .mount("/api", routes![web::default_state, web::get_country_name])
        .mount("/css", FileServer::from(relative!("/web/templates/css")))
        .mount("/data", FileServer::from(relative!("/web/data")))
        .mount("/js", FileServer::from(relative!("/web/templates/js")))
}

