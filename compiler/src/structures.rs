#[derive(Debug, Copy, Clone)]
pub enum Operator
{
	AritAdd, AritSub, AritMult, AritDiv, AritMod, AritNeg, AritPos,
	BitAnd, BitOr, BitXor, BitShiftLeft, BitShiftRight, BitNeg,
	BoolOr,	BoolAnd, BoolXor, BoolNeg,
	CompEquals, CompNotEquals, CompLessEquals, CompGreatEquals, CompLess, CompGreat,
	Changed, NotChanged,
	Old,
	Deref,
}

#[derive(Debug, Clone)]
pub enum Type
{
	Serial,
	Pin { pintype: Option<PinType>, direction: Option<PinDirection> },
	Proc,
	Func { from : Vec<Type>, to : Box<Type> },
	Array(Box<Type>),
	Boolean,
	Char,	String,
	Time,
	Float,	Double,
	Int { signed: bool, length: i8 },
}

#[derive(Debug, Copy, Clone)]
pub enum PinType
{
	Analog,
	Digital
}

#[derive(Debug, Copy, Clone)]
pub enum PinDirection
{
	Input { pullup: bool },
	Output
}

#[derive(Debug)]
pub enum Time<'a>
{
	Millis(Box<AST<'a>>),
	Micros(Box<AST<'a>>)
}

#[derive(Debug)]
pub enum AST<'a>
{
	State { name: &'a str, onenter: Option<Vec<AST<'a>>>, states: Vec<AST<'a>>, vars: Vec<AST<'a>>, reactions: Vec<AST<'a>> },
	Variable { t: Type, mutable: bool, name: &'a str, initial: Box<AST<'a>> },
	Reaction { time: Option<Time<'a>>, expr: Option<Box<AST<'a>>>, stmts: Vec<AST<'a>> },
	Expr { t: Option<Type>, a: Box<AST<'a>>, op: Operator, b: Option<Box<AST<'a>>> },
	AssignStmt { target: Box<AST<'a>>, op: Option<Operator>, value: Box<AST<'a>> },
	EnterStmt { state: &'a str },
	RunStmt { expr: Box<AST<'a>> },
	Call { expr: Box<AST<'a>>, parameters: Vec<AST<'a>> },
	Con { t: Type },
	Reference { t: Option<Type>, name: &'a str}
}
impl<'a> Clone for AST<'a> {
	fn clone(&self) -> Self {
		match self {
			AST::Expr { t, a, op, b } => AST::Expr { t: t.clone(), a: a.clone(), op: op.clone(), b: b.clone() },
			AST::Reference { t, name } => AST::Reference { t: t.clone(), name: name.clone() },
			_ => { println!("{:#?}", self); unimplemented!() }
		}
	}
}
