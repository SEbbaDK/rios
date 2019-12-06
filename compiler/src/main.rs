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
	Changed, NotChanged,
	Old,
	Deref,
}

#[derive(Debug)]
enum Type
{
	Serial,
	Pin,
	Proc,
	Func { from : Option<Vec<Type>>, to : Box<Type> },
	Array(Box<Type>),
	Boolean,
	Char,	String,
	Time,
	Float,	Double,
	Int { signed: bool, length: i8 },
}

#[derive(Debug)]
enum Time<'a>
{
	Millis(Box<AST<'a>>),
	Micros(Box<AST<'a>>),
	None
}

#[derive(Debug)]
enum AST<'a>
{
	State { name: &'a str, onenter: Option<Vec<AST<'a>>>, states: Vec<AST<'a>>, vars: Vec<AST<'a>>, reactions: Vec<AST<'a>> },
	Variable { t: Type, mutable: bool, name: &'a str, initial: Box<AST<'a>> },
	Reaction { time: Time<'a>, expr: Option<Box<AST<'a>>>, stmts: Vec<AST<'a>> },
	Expr { t: Option<Type>, a: Box<AST<'a>>, op: Operator, b: Option<Box<AST<'a>>> },
	AssignStmt { target: Box<AST<'a>>, value: Box<AST<'a>> },
	EnterStmt { state: &'a str },
	RunStmt { expr: Box<AST<'a>> },
	Call { expr: Box<AST<'a>>, parameters: Vec<AST<'a>> },
	Con { t: Type },
}

fn main()
{
	let code = fs::read_to_string("src/fade.rios").expect("Cannot read file");
	let parsetree: pest::iterators::Pairs<Rule> = RiosParser::parse(Rule::Program, &code).expect("Parse failure");
	//println!("Result of parsing file: {:#?}", parsetree);

	let ast = build_ast(parsetree);
	println!("Result of building AST: {:#?}", ast);
}

fn build_ast(mut pairs: pest::iterators::Pairs<Rule>) -> AST
{
	let name = "Global";
	let decs = build_ast_decs(pairs.next().unwrap());
	let (onenter, states, vars, reactions) = decs;

	AST::State { name, onenter, states, vars, reactions }
}

fn build_ast_state(pair: pest::iterators::Pair<Rule>) -> AST
{
	let mut iter = pair.into_inner().into_iter();
	let name = iter.next().unwrap().as_str();
	let decs = build_ast_decs(iter.next().unwrap());
	let (onenter, states, vars, reactions) = decs;

	AST::State { name, onenter, states, vars, reactions }
}

fn build_ast_decs(pair: pest::iterators::Pair<Rule>) -> (Option<Vec<AST>>, Vec<AST>, Vec<AST>, Vec<AST>)
{
	let mut onenter = None;
	let mut states = Vec::new();
	let mut vars = Vec::new();
	let mut reactions = Vec::new();
	for inner in pair.into_inner() {
		match inner.as_rule() {
			Rule::ReactOnenter => onenter = Some(build_ast_stmts(inner.into_inner().next().unwrap())),
			Rule::ReactAlways | Rule::ReactEvery | Rule::ReactAfter | Rule::ReactWhen
			=> reactions.append(&mut build_ast_reaction(inner)),
			Rule::VarDec => vars.push(build_ast_var(inner)),
			Rule::StateDec => states.push(build_ast_state(inner)),
			_ => { println!("{:#?}",inner);unreachable!() }
		}
	}

	(onenter, states, vars, reactions)
}

fn build_ast_var(pair: pest::iterators::Pair<Rule>) -> AST
{
	let mut iter = pair.into_inner().into_iter();

	let mutable = if iter.peek().unwrap().as_rule() == Rule::Mutable {
		iter.next();
		true
	} else { false };

	let t: Type = match iter.next().unwrap().as_str().trim() {
		"bool"  => Type::Boolean,
		"float" => Type::Float,
		"double"=> Type::Double,
		"pin"   => Type::Pin,
		"serial"=> Type::Serial,
		"proc"  => Type::Proc,
		"string"=> Type::String,
		"char"  => Type::Char,
		"time"  => Type::Time,
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

	AST::Variable { t, mutable, name, initial: Box::new(initial) }
}

fn build_ast_stmts(pair: pest::iterators::Pair<Rule>) -> Vec<AST>
{
	let mut stmts = Vec::new();
	for inner in pair.into_inner() {
		match inner.as_rule() {
			Rule::VarDec => stmts.push(build_ast_var(inner)),
			Rule::Assign => {
				let mut iter = inner.into_inner();
				let target = box iter.next().unwrap();
				let value = box iter.next().unwrap();
				stmts.push(AST::AssignStmt { target, value })
			}
			Rule::Enter =>
			Rule::Run =>
		}
	}
	stmts
}

fn build_ast_reaction(pair: pest::iterators::Pair<Rule>) -> Vec<AST>
{
	let mut reacts = Vec::new();

	match pair.as_rule() {
		Rule::ReactAlways => reacts.push(AST::Reaction { time: Time::None, expr: None, stmts: build_ast_stmts(pair.into_inner().next().unwrap()) }),
		Rule::ReactEvery | Rule::ReactAfter => {
			let mut inner = pair.into_inner().into_iter();
			let time_expr = Box::new(build_ast_expr(inner.next().unwrap()));
			let time = if inner.next().unwrap().as_str() == "µs"
			{ Time::Micros(time_expr) }
			else
			{ Time::Millis(time_expr) };
			let stmts = build_ast_stmts(inner.next().unwrap());
			reacts.push(AST::Reaction { time, expr: None, stmts })
		},
		Rule::ReactWhen => {
			let mut inner = pair.into_inner().into_iter();
			let common_expr = build_ast_expr(inner.next().unwrap());
			let common_op = if(inner.peek().unwrap().as_rule() == Rule::WhenOp)
			{ Some(build_ast_operator(inner.next().unwrap().into_inner().next().unwrap())) }
			else
			{ None };

			match inner.peek().unwrap().as_rule() {
				Rule::Case => {

				},
				Rule::Result => {

				},
				_ => unreachable!()
			}
		},
		_ => { println!("{:#?}", pair); unreachable!() }
	}

	reacts
}

fn build_ast_expr(pair: pest::iterators::Pair<Rule>) -> AST
{
	match pair.as_rule() {
		Rule::Expr => build_ast_expr(pair.into_inner().into_iter().next().unwrap()),
		Rule::ExprBOr |
		Rule::ExprBAnd |
		Rule::ExprBXor |
		Rule::ExprShift |
		Rule::ExprComp |
		Rule::ExprMult |
		Rule::ExprAdd => build_ast_binary_expr(pair),
		Rule::ExprNeg |
		Rule::ExprOld |
		Rule::ExprDeref => build_ast_unary_expr(pair),
		Rule::ExprCall => build_ast_call_expr(pair),
		Rule::ExprParen => {
			let inner = pair.into_inner().into_iter().next().unwrap();
			match inner.as_rule() {
				Rule::Expr => build_ast_expr(inner),
				Rule::Con => build_ast_con(inner),
				_ => unreachable!(),
			}
		}
		_ => unreachable!()
	}
}

fn build_ast_binary_expr(pair: pest::iterators::Pair<Rule>) -> AST
{
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
}

fn build_ast_unary_expr(pair: pest::iterators::Pair<Rule>) -> AST
{
	let mut iter = pair.into_inner().into_iter();
	match iter.peek().unwrap().as_rule() {
		Rule::BoolNegOp | Rule::BitNegOp | Rule::AritNegOp |
		Rule::OldOp | Rule::DerefOp => {
			let op = build_ast_operator(iter.next().unwrap());
			let expr = build_ast_expr(iter.next().unwrap());
			AST::Expr { t:None, a: Box::new(expr), op, b: None }
		}
		_ => {
			build_ast_expr(iter.next().unwrap())
		}
	}
}

fn build_ast_call_expr(pair: pest::iterators::Pair<Rule>) -> AST
{
	let mut iter = pair.into_inner().into_iter();
	let expr = build_ast_expr(iter.next().unwrap());
	if iter.peek().is_none() {
		expr
	}
	else {
		let mut parameters = Vec::new();
		for inner in iter {
			parameters.push(build_ast_expr(inner));
		}
		AST::Call { expr: Box::new(expr), parameters }
	}
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

		Rule::CompEOp	=> Operator::CompEquals,
		Rule::CompNEOp	=> Operator::CompNotEquals,
		Rule::CompLEOp	=> Operator::CompLessEquals,
		Rule::CompGEOp	=> Operator::CompGreatEquals,
		Rule::CompLOp	=> Operator::CompLess,
		Rule::CompGOp	=> Operator::CompGreat,
		Rule::ChangedOp	=> Operator::Changed,
		Rule::NotChangedOp=> Operator::NotChanged,
		Rule::OldOp	    => Operator::Old,
		Rule::DerefOp	=> Operator::Deref,
		_ => { println!("{:#?}", pair); unreachable!() }
	}
}

fn build_ast_con(pair: pest::iterators::Pair<Rule>) -> AST
{
	let con = pair.into_inner().into_iter().next().unwrap();
	match con.as_rule() {
		Rule::BinCon => {
			let chars = con.as_str().chars().skip(2);
			let value = parse_chars_in_base(2, chars);
			AST::Con { t: Type::Int{ signed: false, length: 32 } }
		},
		Rule::OctCon => {
			let chars = con.as_str().chars().skip(2);
			let value = parse_chars_in_base(8, chars);
			AST::Con { t: Type::Int { signed: false, length: 32 } }
		},
		Rule::HexCon => {
			let chars = con.as_str().chars().skip(2);
			let value = parse_chars_in_base(16, chars);
			AST::Con { t: Type::Int { signed: false, length: 32 } }
		}
		_ => AST::Con { t: Type::String }
	}
}

fn parse_chars_in_base<I>(base: u32, chars: I ) -> u32
	where I: Iterator<Item=char>
{
	let mut value : u32 = 0;
	for c in chars {
		value = value * base;
		value = value & char_to_num(c);
	}
	value
}

fn char_to_num(c: char) -> u32
{
	match c {
		'0' => 0,
		'1' => 1,
		'2' => 2,
		'3' => 3,
		'4' => 4,
		'5' => 5,
		'6' => 6,
		'7' => 7,
		'8' => 8,
		'9' => 9,
		'A'|'a' => 10,
		'B'|'b' => 11,
		'C'|'c' => 12,
		'D'|'d' => 13,
		'E'|'e' => 14,
		'F'|'f' => 15,
		_  => panic!("char2num: \"{}\" is not a number!", c)
	}
}
