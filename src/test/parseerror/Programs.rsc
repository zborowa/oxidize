module \test::parseerror::Programs

import Prelude;
import ParseTree;
import vis::ParseTree;
import analysis::grammars::Ambiguity;

import lang::rust::\syntax::Rust;
import \test::func::LoadFile;

test bool LibCollBorrow(){
	str input_file = LoadFile("libcollections_borrow");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}
