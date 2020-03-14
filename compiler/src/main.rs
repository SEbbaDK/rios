#![feature(box_syntax)]

use std::fs;
mod parser;
mod ast;
mod structures;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use ansi_term::{Style, Colour};

use clap::{Arg, App, crate_version, crate_authors};

struct Args {
	file: String,
	quiet: bool,
	show_ast: bool,
	show_st: bool,
}

fn main()
{
	let input = App::new("Rios Compiler")
		.version(crate_version!())
		.author(crate_authors!())
		.about("Compiles Rios source files into executable programs.")
		.arg(Arg::with_name("file")
			.required(true)
//			.index(1)
			.help("The source-code file to compile"))
		.arg(Arg::with_name("show_syntax_tree")
			.short("syntax-tree")
			.long("show-syntax-tree")
			.help("Print the syntax tree"))
		.arg(Arg::with_name("show_abstract_syntax_tree")
			.short("a")
			.long("show-ast")
			.help("Print the abstract syntax tree"))
		.arg(Arg::with_name("quiet")
			.short("q")
			.long("quiet")
			.help("Print no output except for errors"))
		.get_matches();

	macro_rules! printer {
		($($arg:tt)*) => { if !input.is_present("quiet") { println!($($arg)*) } };
	}

	let header = Style::new().bold().fg(Colour::Blue);

	println!("q: {}  st: {}  ast: {}", input.is_present("quiet"), input.is_present("show_syntax_tree"), input.is_present("show_abstract_syntax_tree"));

	let file = input.value_of("file").expect("Couldn't get filename");
	printer!("Compiling {}", file);
	let code = fs::read_to_string(file).expect("Cannot read file");
	let parsetree = parser::parse(&code);
	if input.is_present("show_syntax_tree") {
		printer!("{}:\n{:#?}", header.paint("Result of parsing file"), parsetree);
	}

	let ast = ast::build_ast(parsetree);
	if input.is_present("show_abstract_syntax_tree") {	
		printer!("{}:\n{:#?}", header.paint("Result of building AST"), ast);
	}
}
