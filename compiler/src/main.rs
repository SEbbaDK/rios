extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "rios.pest"]
pub struct RiosParser;

fn main() {
	loop
	{
		println!("Enter value to parse: ");
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		println!("Entered: <{}>", input);
		let parse = RiosParser::parse(Rule::con, &input);
		println!("{:?}\n", parse);
	}
}
