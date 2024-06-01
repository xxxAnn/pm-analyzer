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
    let b = data.get_building("building_food_industry").ok_or("Building not found".to_string())?.get_default_data(&data).ok_or("No defaults found")?;
    println!("Building: {}", b.name());
    println!("Input: {}", b.get(Input));
    println!("Output: {}", b.get(Output));
    println!("Labor: {}", b.get(Labor));
    println!("Cost: {}", b.get(Construction));
    println!("Efficiency per hundred worker: {}", b.get(EfficiencyPerWorker) * 100.0);
    println!("Net Output: {}", b.get(NetOutput));
    println!("Efficiency per ten construction: {}", b.get(EfficiencyPerConstruction) * 10.0);

    Ok(())
}