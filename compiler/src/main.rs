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
enum AST
{
	StateDec { name: &'static str, decs: Vec<AST> },
	BinaryExpr { t: Type, a: &'static AST, b: &'static AST, op: Operator },
	UnaryExpr { t: Type, a: &'static AST, op: Operator },
	Con { t: Type }
}

fn main()
{
	let code = fs::read_to_string("src/fade.rios").expect("Cannot read file");
	let parsetree = RiosParser::parse(Rule::Program, &code).expect("Parse failure");
	println!("Result of parsing file: {:#?}", parsetree);

	let ast = build_ast(parsetree);
	println!("Result of building AST: {:#?}", ast);
}

fn build_ast(pairs: pest::iterators::Pairs<'_, Rule>) -> AST
{
	let mut globaldecs: Vec<AST> = Vec::new();
	for pair in pairs {
		globaldecs.push(build_ast_dec(pair));
	}
	return AST::StateDec { name: "Global", decs: globaldecs}
}

fn build_ast_dec(pair: pest::iterators::Pair<Rule>) -> AST
{
	match pair.as_rule() {
		_ => AST::Con { t: Type::Char }
	}
}