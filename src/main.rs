#![allow(dead_code)]

mod parser;
mod scanner;
mod data;

use data::Data;
use parser::Parser;

fn main() -> Result<(), String> {
    
    let text = std::fs::read_to_string("01_industry.txt").unwrap();

    let pms_parser = Parser::new();
    let pms_tree = pms_parser.parse(text);

    let goods_parser = Parser::new();
    let goods_tree = goods_parser.parse(std::fs::read_to_string("00_goods.txt").unwrap());

    let data = Data::new(pms_tree, goods_tree).unwrap();

    dbg!(data.get_pm("pm_improved_food_manufactories"));
    

    //let scan = scanner::scan();
    Ok(())
}