#![allow(dead_code)]

mod pm;
mod parser;
mod goods;

use pm::PM;
use goods::Goods;
use parser::Parser;

fn main() -> Result<(), String> {
    let text = std::fs::read_to_string("01_industry.txt").unwrap();

    let pms_parser = Parser::new();
    let tree = pms_parser.parse(text);

    let goods_parser = Parser::new();
    let goods_tree = goods_parser.parse(std::fs::read_to_string("00_goods.txt").unwrap());

    let goods: Goods = goods_tree.into();

    let pm_discr_manuf_batteries: PM = PM::from_tree(tree.get("pm_discrete_manufacturing_batteries")?, goods)?;

    dbg!(pm_discr_manuf_batteries);

    Ok(())
}