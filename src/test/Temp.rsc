module \test::Temp

import Prelude;
import vis::ParseTree;
import analysis::grammars::Ambiguity;

import lang::rust::\syntax::Rust;
import \test::func::LoadFile;

test bool Macro_rules(){
	str input_file = LoadFile("macro_rules");
	renderParsetree([start[Crate]]input_file);
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}