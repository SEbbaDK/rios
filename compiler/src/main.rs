extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "rios.pest"]
pub struct RiosParser;

use std::fs;

fn main() 
{
	let code = fs::read_to_string("fade.rios").expect("Cannot read file");
	let parse = RiosParser::parse(Rule::Program, &code);
	println!("Result of parsing file: {:#?}", parse);
}
