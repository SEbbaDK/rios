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
	Int { signed: bool, length: i8 },
}

#[derive(Debug)]
enum AST<'a>
{
	State { name: &'a str, states: Vec<AST<'a>>, vars: Vec<AST<'a>>, reactions: Vec<AST<'a>> },
	Variable { t: Type, name: &'a str, initial: Box<AST<'a>> },
	Reaction {  },
	Expr { t: Type, a: Box<AST<'a>>, op: Operator, b: Option<Box<AST<'a>>> },
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
	let mut iter = pair.into_inner().into_iter();
	let name = iter.next().unwrap().as_str();
	let decs = build_ast_decs(iter.next().unwrap());
	let (states, reactions, vars) = decs;
	return AST::State { name, states, reactions, vars }
}

fn build_ast_var(pair: pest::iterators::Pair<Rule>) -> AST
{
	let mut iter = pair.into_inner().into_iter();
	let t: Type = match iter.next().unwrap().as_str().trim() {
		"bool"  => Type::Boolean,
		"float" => Type::Float,
		"double"=> Type::Double,
		"pin"   => Type::Pin,
		"serial"=> Type::Serial,
		"proc"  => Type::Proc,
		"string"=> Type::String,
		"char"  => Type::Char,
		"uint8" => Type::Int {signed: false, length: 8},
		"uint16"=> Type::Int {signed: false, length: 16},
		"uint32"=> Type::Int {signed: false, length: 32},
		"uint64"=> Type::Int {signed: false, length: 64},
		"int8"  => Type::Int {signed: true, length: 8},
		"int16" => Type::Int {signed: true, length: 16},
		"int32" => Type::Int {signed: true, length: 32},
		"int64" => Type::Int {signed: true, length: 64},
		_ => unreachable!(),
	};
	let name = iter.next().unwrap().as_str();
	let initial = build_ast_expr(iter.next().unwrap());

	AST::Variable { t, name, initial: Box::new(initial) }
}

fn build_ast_reaction(pair: pest::iterators::Pair<Rule>) -> AST
{
	AST::Reaction {  }
}

fn build_ast_expr(pair: pest::iterators::Pair<Rule>) -> AST
{
	match pair.as_rule() {
		Rule::Expr => build_ast_expr(pair.into_inner().into_iter().next().unwrap()),
		Rule::ExprBOr | Rule::ExprBAnd | Rule::ExprBXor |
		Rule::ExprComp | Rule::ExprMult | Rule::ExprAdd => {
			let mut iter = pair.into_inner().into_iter();
			let a = build_ast_expr(iter.next().unwrap());
			if iter.peek().is_none(){
				a
			}
			else {
				let op = build_ast_operator(iter.next().unwrap());
				let b = build_ast_expr(iter.next().unwrap());
				AST::Expr { t: None, a: Box::new(a), op, b: Some(Box::new(b))}
			}
		},
		Rule::ExprNeg | Rule::ExprOld | Rule::ExprDeref => {
			let mut iter = pair.into_inner().into_iter();
			let op = build_ast_operator(iter.next().unwrap());
			let expr = build_ast_expr(iter.next().unwrap());
			AST::Expr { t:None, a: Box::new(expr), op, b: None }
		},
		Rule::ExprSub | Rule::ExprCall => {

		},
		Rule::ExprParen => {
			let inner = pair.into_inner().into_iter().next().unwrap();
			match inner.as_rule() {
				Rule::Expr => build_ast_expr(inner),
				Rule::Con => build_ast_con(inner),
				_ => unreachable!(),
			}
		}
	}
	AST::Expr { t: Type::String, a: Box::new(AST::Con { t: Type::String }), op: Operator::BoolOr, b: None }
}

fn build_ast_operator(pair: pest::iterators::Pair<Rule>) -> Operator
{
	match pair.as_rule() {
		Rule::AritOp | Rule::BitOp | Rule::BoolOp | Rule::CompOp | Rule::ChangeOp
		=> build_ast_operator(pair.into_inner().into_iter().next().unwrap()),
		Rule::AritAddOp	=> Operator::AritAdd,
		Rule::AritSubOp	=> Operator::AritSub,
		Rule::AritMultOp=> Operator::AritMult,
		Rule::AritModOp	=> Operator::AritMod,
		Rule::AritDivOp	=> Operator::AritDiv,

		Rule::AritNegOp	=> Operator::AritNeg,
		Rule::AritPosOp	=> Operator::AritPos,

		Rule::BitNegOp	=> Operator::BitNeg,
		Rule::BitAndOp	=> Operator::BitAnd,
		Rule::BitOrOp	=> Operator::BitOr,
		Rule::BitXorOp	=> Operator::BitXor,
		Rule::BitShiftRightOp=> Operator::BitShiftRight,
		Rule::BitShiftLeftOp=> Operator::BitShiftLeft,

		Rule::BoolOrOp	=> Operator::BoolOr,
		Rule::BoolAndOp	=> Operator::BoolAnd,
		Rule::BoolXorOp	=> Operator::BoolXor,
		Rule::BoolNegOp => Operator::BoolNeg,

		Rule::CompEOp	=> Operator::CompE,
		Rule::CompNEOp	=> Operator::CompNE,
		Rule::CompLEOp	=> Operator::CompLE,
		Rule::CompGEOp	=> Operator::CompGE,
		Rule::CompLOp	=> Operator::CompL,
		Rule::CompGOp	=> Operator::CompG,
		Rule::ChangedOp	=> Operator::Changed,
		Rule::NotChangedOp=> Operator::NotChanged,
		Rule::OldOp	    => Operator::Old,
		Rule::DerefOp	=> Operator::Deref,
	}
}

fn build_ast_con(pair: pest::iterators::Pair<Rule>) -> AST
{

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