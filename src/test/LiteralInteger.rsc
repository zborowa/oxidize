module \test::LiteralInteger

import Prelude;

import lang::rust::\syntax;
import \test::func::LoadFile;

test bool LiteralInteger(){
	str input_file = LoadFile("let_integers");
	Tree parse_tree = parse(#Crate, input_file);
	
	return /amb(_) !:= parse_tree;
}
