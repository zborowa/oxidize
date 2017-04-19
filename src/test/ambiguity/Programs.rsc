module \test::ambiguity::Programs

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

test bool RustMain(){
	str input_file = LoadFile("rust_main");
	Tree parse_tree = parse(#Crate, input_file, allowAmbiguity=true);
	
	return /amb(_) !:= parse_tree;
}

test bool EmptyFunction(){
	str input_file = LoadFile("fn_main");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool AdjustNever(){
	str input_file = LoadFile("adjust_never");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool BootstrapMain(){
	str input_file = LoadFile("bootstrap_main");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool Metadata(){
	str input_file = LoadFile("metadata");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool Install(){
	str input_file = LoadFile("install");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool Clean(){
	str input_file = LoadFile("clean");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool Channel(){
	str input_file = LoadFile("channel");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool LibMacro(){
	str input_file = LoadFile("lib_macro");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool Driver(){
	str input_file = LoadFile("driver");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool FewOnes(){
	str input_file = LoadFile("few-ones");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool ManyDigits(){
	str input_file = LoadFile("many-digits");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool LibAlloc(){
	str input_file = LoadFile("lib_alloc");
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}

test bool Oom(){
	str input_file = LoadFile("oom");
	renderParsetree([start[Crate]]input_file);
	Tree parse_tree = [start[Crate]]input_file;
	
	return /amb(_) !:= parse_tree;
}
