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
			.help("The source-code file to compile")
			.required(true)
		)
		.arg(Arg::with_name("show_syntax_tree")
			.help("Print the syntax tree")
			.short("syntax-tree")
			.long("show-syntax-tree")
			.conflicts_with("quiet")
		)
		.arg(Arg::with_name("show_abstract_syntax_tree")
			.help("Print the abstract syntax tree")
			.short("a")
			.long("show-ast")
			.conflicts_with("quiet")
		)
		.arg(Arg::with_name("quiet")
			.help("Print no output except for errors")
			.short("q")
			.long("quiet")
		)
		.arg(Arg::with_name("completion")
			.help("Completion output for the given input")
			.short("c")
			.long("completion")
			.empty_values(true)
			.value_name("completion")
		)
		.get_matches();

	if input.is_present("completion") {
		println!("Hello");
		return;
	}

	macro_rules! printer {
		($($arg:tt)*) => { if !input.is_present("quiet") { println!($($arg)*) } };
	}

	let header = Style::new().bold().fg(Colour::Blue);

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
