module \test::Literals

import Prelude;

import lang::rust::\syntax::Rust;
import \test::func::LoadFile;

test bool LiteralInteger(){
	str input_file = LoadFile("let_integers");
	Tree parse_tree = parse(#Crate, input_file);
	
	return /amb(_) !:= parse_tree;
}

test bool LiteralString(){
	str input_file = LoadFile("let_strings");
	Tree parse_tree = parse(#Crate, input_file);
	
	return /amb(_) !:= parse_tree;
}
