extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;

use pest::Parser;

#[derive(Parser)]
#[grammar = "rios.pest"]
pub struct RiosParser;

#[derive(Debug)]
enum Operator
{
	AritAdd, AritSub, AritMult, AritDiv, AritMod, AritNeg, AritPos,
	BitAnd, BitOr, BitXor, BitShiftLeft, BitShiftRight, BitNeg,
	BoolOr,	BoolAnd, BoolXor, BoolNeg,
	CompEquals, CompNotEquals, CompLessEquals, CompGreatEquals, CompLess, CompGreat,
	Change, ChangeNot,
	Old,
	Deref,
}

#[derive(Debug)]
enum Type
{
	Serial,
	Pin,
	Proc,
	Array(Box<Type>),
	Boolean,
	Char,	String,
	Float,	Double,
	Int8,	Int16,	Int32,	Int64,
	UInt8,	UInt16,	UInt32,	UInt64,
}

#[derive(Debug)]
enum AST<'a>
{
	State { name: &'a str, states: Vec<AST<'a>>, vars: Vec<AST<'a>>, reactions: Vec<AST<'a>> },
	Variable { t: Type, name: &'a str, initial: Box<AST<'a>> },
	Reaction {  },
	Expr { t: Type, a: &'a AST<'a>, op: Operator, b: Option<&'a AST<'a>> },
	Con { t: Type },
}

fn main()
{
	let code = fs::read_to_string("src/fade.rios").expect("Cannot read file");
	let parsetree: pest::iterators::Pairs<Rule> = RiosParser::parse(Rule::Program, &code).expect("Parse failure");
	println!("Result of parsing file: {:#?}", parsetree);

	let ast = build_ast(parsetree);
	println!("Result of building AST: {:#?}", ast);
}

fn build_ast(mut pairs: pest::iterators::Pairs<Rule>) -> AST
{
	let name = "Global";
	let (states, vars, reactions) = build_ast_decs(pairs.next().unwrap());
	return AST::State { name, states, vars, reactions };

	//let globalnode = pest::iterators::Pair {  };
}

fn build_ast_state(pair: pest::iterators::Pair<Rule>) -> AST
{
	let mut name = "STANDARDSTATENAME";
	let mut decs = (Vec::new(), Vec::new(), Vec::new());

	for inner in pair.into_inner() {
		match inner.as_rule() {
			Rule::StateName => name = inner.as_str(), //as_span().as_str()
			Rule::Decs => decs = build_ast_decs(inner),
			_ => unreachable!(),
		}
	}
	let (states, vars, reactions) = decs;
	return AST::State { name, states, reactions, vars }
}

fn build_ast_var(pair: pest::iterators::Pair<Rule>) -> AST
{
	let mut name = "ERROR: No Name Specified";
	let mut t = Type::String;
	let mut initial = AST::Con { t: Type::String };

	for inner in pair.into_inner() {
		match inner.as_rule() {
			Rule::VarName => name = inner.as_str(),
			Rule::Type => t = match inner.as_str().trim() {
				"bool" => Type::Boolean,
				_ => Type::String,
			},
			Rule::Expr => initial = build_ast_expr(inner),
			_ => unreachable!(),
		}
	}
	AST::Variable { t, name, initial: Box::new(initial) }
}

fn build_ast_reaction(pair: pest::iterators::Pair<Rule>) -> AST
{
	AST::Reaction {  }
}

fn build_ast_expr(pair: pest::iterators::Pair<Rule>) -> AST
{
	AST::Expr { t: Type::String, a: &AST::Con { t: Type::String }, op: Operator::BoolOr, b: None }
}
//fn build_ast_var(pair: pest::iterators::Pair<Rule>) -> AST


fn build_ast_decs(pair: pest::iterators::Pair<Rule>) -> (Vec<AST>, Vec<AST>, Vec<AST>)
{
	let mut states = Vec::new();
	let mut vars = Vec::new();
	let mut reactions = Vec::new();
	for inner in pair.into_inner() {
		match inner.as_rule() {
			Rule::ReactDec => reactions.push(build_ast_reaction(inner)),
			Rule::VarDec => vars.push(build_ast_var(inner)),
			Rule::StateDec => states.push(build_ast_state(inner)),
			_ => unreachable!()
		}
	}
	return (states, vars, reactions);
}