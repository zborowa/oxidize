module \test::TestLiteralInteger

import Prelude;

import lang::rust::\syntax;
import \test::input::LoadFile;

test bool Test_LetLiteralInteger(){
	str input_file = LoadFile("let_integers");
	Tree parse_tree = parse(#Crate, input_file);
	
	return /amb(_) !:= parse_tree;
}