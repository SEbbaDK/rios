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
	let parsetree = RiosParser::parse(Rule::Program, &code).unwrap();
	println!("Result of parsing file: {:#?}", parsetree);
	let ast = buildAST(&parsetree);
	println!("Result of building AST: {:#?}", ast);
}

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
	BinaryExpr { t: Type, a: &'static AST, b: &'static AST, op: Operator },
	UnaryExpr { t: Type, a: &'static AST, op: Operator },
	Con { t: Type }
}

fn buildAST(parsetree : &pest::iterators::Pairs<'_, Rule>) -> AST
{
	return AST::Con { t : Type::Int16 };
}
