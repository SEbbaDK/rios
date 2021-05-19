use super::parser::Rule;
use super::structures::{ AST, Operator, Type, PinType, PinDirection, Time };

pub fn build_ast(mut pairs: pest::iterators::Pairs<Rule>) -> AST
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
			Rule::ReactOnenter => onenter = Some(build_ast_stmts(inner.into_inner().next().unwrap().into_inner().next().unwrap())),
			Rule::ReactAlways | Rule::ReactEvery | Rule::ReactAfter | Rule::ReactWhen
			=> reactions.append(&mut build_ast_reaction(inner)),
			Rule::VarDec => vars.push(build_ast_var(inner)),
			Rule::StateDec => states.push(build_ast_state(inner)),
			_ => { println!("{:#?}", inner); unreachable!() }
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

	let type_dec = iter.next().unwrap();
	let t: Type = match type_dec.as_rule() {
		Rule::BoolType  => Type::Boolean,
		Rule::FloatType => Type::Float,
		Rule::DoubleType=> Type::Double,
		Rule::PinType   => {
			match type_dec.into_inner().into_iter().next().unwrap().as_rule() {
				Rule::PinAnalog => Type::Pin { pintype: Some(PinType::Analog), direction: None },
				Rule::PinDigital => Type::Pin { pintype: Some(PinType::Digital), direction: None },
				_ => unreachable!()
			}
		},
		Rule::SerialType=> Type::Serial,
		Rule::ProcType  => Type::Proc,
		Rule::StringType=> Type::String,
		Rule::CharType  => Type::Char,
		Rule::TimeType  => Type::Time,
		Rule::IntType   => {
			match type_dec.as_str().trim() {
				"uint8" => Type::Int {signed: false, length: 8},
				"uint16"=> Type::Int {signed: false, length: 16},
				"uint32"=> Type::Int {signed: false, length: 32},
				"uint64"=> Type::Int {signed: false, length: 64},
				"int8"  => Type::Int {signed: true, length: 8},
				"int16" => Type::Int {signed: true, length: 16},
				"int32" => Type::Int {signed: true, length: 32},
				"int64" => Type::Int {signed: true, length: 64},
				_ => unreachable!()
			}
		},
		_ => { println!("{:#?}", type_dec); unreachable!() }
	};
	let name = iter.next().unwrap().as_str();
	let initial = build_ast_expr(iter.next().unwrap());

	AST::Variable { t, mutable, name, initial: Box::new(initial) }
}

fn build_ast_stmts(pair: pest::iterators::Pair<Rule>) -> Vec<AST>
{
	debug_assert!(pair.as_rule() == Rule::Stmts);
	let mut stmts = Vec::new();
	for inner in pair.into_inner() {
		stmts.push(build_ast_stmt(inner));
	}
	stmts
}

fn build_ast_stmt(pair: pest::iterators::Pair<Rule>) -> AST
{
	debug_assert_eq!(pair.as_rule(), Rule::Stmt);
	let stmt = pair.into_inner().next().unwrap();
	match stmt.as_rule() {
		Rule::VarDec => build_ast_var(stmt),
		Rule::Assign => {
			let mut iter = stmt.into_inner();
			let target = Box::new(build_ast_expr(iter.next().expect("Expected left hand side of assignment to be an expression.")));
			if iter.peek().unwrap().as_rule() != Rule::Expr {
				let op = build_ast_operator(iter.next().unwrap());
				let right = Some(Box::new(build_ast_expr(iter.next().unwrap())));
				let t: Option<Type> = match &*target {
					AST::Expr{ t, a: _, op: _, b: _} => t.clone(),
					AST::Reference { t, name } => t.clone(),
					_ => {println!("{:#?}", *target);unreachable!()}
				};
				let value = Box::new(AST::Expr { t, a: Box::new(*target.clone()), op, b: right});
				AST::AssignStmt { target, op: Some(op), value }
			}
			else {
				let value = Box::new(build_ast_expr(iter.next().expect("Expected right hand side of assignment to be an expression.")));
				AST::AssignStmt { target, op: None, value }
			}
		}
		Rule::Enter => {
			let state : &str = stmt.into_inner().next().expect("Expected name of state after 'enter' statement.").as_str();
			AST::EnterStmt { state }
		}
		Rule::Run => {
			let expr = Box::new(build_ast_expr(stmt.into_inner().next().expect("Expected expr after 'run' keyword.")));
			AST::RunStmt { expr }
		}
		_ => { println!("{:#?}", stmt.as_rule()); println!("{:#?}", stmt.as_str()); unreachable!() }
	}
}

fn build_ast_reaction(pair: pest::iterators::Pair<Rule>) -> Vec<AST>
{
	let mut reacts = Vec::new();

	match pair.as_rule() {
		Rule::ReactAlways => reacts.push(AST::Reaction { time: None, expr: None, stmts: build_ast_stmts(pair.into_inner().next().unwrap()) }),
		Rule::ReactEvery | Rule::ReactAfter => {
			let mut inner = pair.into_inner().into_iter();
			let time_expr = Box::new(build_ast_expr(inner.next().unwrap()));
			let time = Some(if inner.next().unwrap().as_str() == "µs"
			{ Time::Micros(time_expr) }
			else
			{ Time::Millis(time_expr) }
			);
			let result = inner.next().expect("Expected resulting stmts after Reaction");
			let stmts = build_ast_stmts(result.into_inner().next().expect("Expected stmts in a result"));
			reacts.push(AST::Reaction { time, expr: None, stmts })
		},
		Rule::ReactWhen => {
			let mut inner = pair.into_inner().into_iter();
			let common_expr = build_ast_expr(inner.next().unwrap());
			let common_op = if inner.peek().unwrap().as_rule() == Rule::WhenOp
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
		_ => { println!("{:#?}", pair); unimplemented!() }
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
				_ => { println!("{:#?}", inner); unreachable!() },
			}
		},
		_ => { println!("{:#?}", pair); unreachable!() }
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
	match con.as_rule() { //TODO: Fix these assumptions about signs
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
		},
		Rule::DecCon => {
			let chars = con.as_str().chars();
			let value = parse_chars_in_base(10, chars);
			AST::Con { t: Type::Int { signed: false, length: 32 } }
		},
		Rule::PinCon => {
			let mut iter = con.into_inner();
			let pintype = if iter.peek().unwrap().as_rule() == Rule::PinPinType {
				Some(match iter.next().unwrap().into_inner().next().unwrap().as_rule() {
					Rule::PinAnalog => PinType::Analog,
					Rule::PinDigital => PinType::Digital,
					_ => unreachable!()
				})
			} else {
				None
			};
			let direction = Some(match iter.next().unwrap().as_rule() {
				Rule::PinOutput => PinDirection::Output,
				Rule::PinInput => {
					let pullup: bool = iter.peek().unwrap().as_rule() == Rule::PinPullup;
					PinDirection::Input { pullup }
				},
				_ => unreachable!()
			});
			AST::Con { t: Type::Pin { pintype, direction } }
		},
		Rule::VarName => {
			AST::Reference { t: None, name: con.as_str() }
		},
		_ => { println!("{:#?}", con); unimplemented!() }
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
