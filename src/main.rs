mod pm;
mod parser;

use std::rc::Rc;
use std::cell::RefCell;
use parser::Parser;
use parser::print_tree;

fn main() {
    let text = std::fs::read_to_string("test.txt").unwrap();

    let parser = Parser::new();
    let root = parser.parse(text);

    print_tree(&root, 0);
}