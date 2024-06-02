#![allow(dead_code)]
mod scanner;
mod data;

use std::io::Write;

use vic3_parser::Tree;
use data::{Attribute::*, Data};

fn main() -> Result<(), String> {
    let data = scanner::scan()?;

    //dbg!(data.get_pm("pm_improved_food_manufactories"));
    //dbg!(data.get_pmg("pmg_base_building_food_industry"));
    //dbg!(data.get_building("building_food_industry"));
    //dbg!(data.get_building("pm_basic_distillation_liquor"));
    write_csv(&data);
    Ok(())
}
// Goes through each PM of each building 
// Then writes the EfficiencyData of each PM to a CSV file
fn write_csv(data: &Data) {
    let mut grid: Vec<Vec<String>>  = Vec::new();
    grid.push(vec!["Building".to_string(), "PM".to_string(), "Input".to_string(), "Output".to_string(), "Labor".to_string(), "NetOutput".to_string(), "Efficiency Per Hundred Worker".to_string(), "EfficiencyPerConstruction".to_string()]);
    let mut new_building;
    for building in data.get_all_buildings() {
        new_building = true;
        for pmg in building.get_pm_by_pmgs(data) {
            for ( i, pm) in pmg.1.iter().enumerate() {

                if let Some(pm_data) = building.get_pm_data(data, &pm) {
                    grid.push(vec![
                        if new_building { building.name().clone()  } else { "".to_owned() }, 
                        { if i == 0 {""} else { "    |"} }.to_owned() + &pm.clone(),
                        pm_data.get(Input).to_string(), 
                        pm_data.get(Output).to_string(), 
                        pm_data.get(Labor).to_string(), 
                        pm_data.get(NetOutput).to_string(), 
                        (pm_data.get(EfficiencyPerWorker) * 100.0).to_string(), 
                        pm_data.get(EfficiencyPerConstruction).to_string()]
                    );
                    new_building = false;
                }
            }
        }
    }

    // write csv raw (without external crate)
    let mut file = std::fs::File::create("output.csv").unwrap();
    for row in grid {
        let mut row = row.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<String>>().join(",");
        row.push('\n');
        file.write_all(row.as_bytes()).unwrap();
    }
}