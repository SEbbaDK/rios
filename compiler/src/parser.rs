use pest::Parser;

#[derive(Parser)]
#[grammar = "rios.pest"]
pub struct RiosParser;

pub fn parse(input: &str) -> pest::iterators::Pairs<Rule> {
	RiosParser::parse(Rule::Program, input).expect("Parse failure")
}

