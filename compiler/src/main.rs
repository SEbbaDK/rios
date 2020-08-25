#![feature(box_syntax)]

use std::fs;
mod parser;
mod ast;
mod structures;
mod symbols;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use ansi_term::{Style, Colour};

use clap::{Arg, App, crate_version, crate_authors};

struct Args<'a> {
	file: &'a str,
	quiet: bool,
	verbose: bool,
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
			.short("t")
			.long("show-syntax-tree")
			.conflicts_with("quiet")
		)
		.arg(Arg::with_name("show_abstract_syntax_tree")
			.help("Print the abstract syntax tree")
			.short("a")
			.long("show-ast")
			.conflicts_with("quiet")
		)
		.arg(Arg::with_name("verbose")
			.help("Print the different steps taken by the compiler")
			.short("v")
			.long("verbose")
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

	// TODO: This should be changed when clap 3 releases
	let args = Args {
		file: input.value_of("file").expect("File will always be given soooo...."),
		quiet: input.is_present("quiet"),
		verbose: input.is_present("verbose"),
		show_ast: input.is_present("show_abstract_syntax_tree"),
		show_st: input.is_present("show_syntax_tree"),
	};

	macro_rules! printer {
		($($arg:tt)*) => { if !args.quiet { println!($($arg)*) } };
	}

	macro_rules! vprinter {
		($($arg:tt)*) => { if args.verbose { printer!($($arg)*) } };
	}

	let header = Style::new().bold().fg(Colour::Blue);

	vprinter!("Compiling {}", args.file);
	vprinter!("Parsing file");
	let code = fs::read_to_string(args.file).expect("Cannot read file");
	let parsetree = parser::parse(&code);
	if args.show_st {
		printer!("{}:\n{:#?}", header.paint("Result of parsing file"), parsetree);
	}

	vprinter!("Building abstract syntax tree");
	let ast = ast::build_ast(parsetree);
	if args.show_ast {	
		printer!("{}:\n{:#?}", header.paint("Result of building AST"), ast);
	}

	vprinter!("Finding declared symbols and adding to the symbol table");
	let symbol_table = symbols::find_symbols();
}
