#![feature(box_syntax)]

use std::fs;
mod parser;
mod ast;
mod structures;

extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main()
{
	let code = fs::read_to_string("src/fade.rios").expect("Cannot read file");
	let parsetree = parser::parse(&code);
	println!("Result of parsing file: {:#?}", parsetree);

	let ast = ast::build_ast(parsetree);
	println!("Result of building AST: {:#?}", ast);
}
