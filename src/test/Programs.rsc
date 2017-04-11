module \test::Programs

import Prelude;
import vis::ParseTree;
import analysis::grammars::Ambiguity;

import lang::rust::\syntax::Rust;
import \test::func::LoadFile;

test bool GuessingGame(){
	str input_file = LoadFile("guessing_game");
	Tree parse_tree = parse(#Crate, input_file, allowAmbiguity=true);
	
	return /amb(_) !:= parse_tree;
}
