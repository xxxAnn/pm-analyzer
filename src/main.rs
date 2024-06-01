#![allow(dead_code)]

mod parser;
mod scanner;
mod data;

use parser::Tree;
use data::Attribute::*;

fn main() -> Result<(), String> {
    let data = scanner::scan()?;

    //dbg!(data.get_pm("pm_improved_food_manufactories"));
    //dbg!(data.get_pmg("pmg_base_building_food_industry"));
    //dbg!(data.get_building("building_food_industry"));
    //dbg!(data.get_building("pm_basic_distillation_liquor"));
    let b = data.get_building("building_food_industry").ok_or("Building not found".to_string())?;
    //println!("PMs: {:?}", b.get_pm_names(&data));
    println!("Efficiency of PM {}: {}", "pm_improved_food_manufactories", b.get_pm_data(&data, "pm_improved_food_manufactories").ok_or("PM Data not found.")?.to_string());
    Ok(())
}