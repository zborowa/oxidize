@contributors{Adrian Zborowski - 11336544 - adrian.zborowski@uva.nl}

module main

// Standard libraries
import Prelude;
import analysis::grammars::Ambiguity;
import vis::ParseTree;

// Custom libraries
import lang::rust::\syntax;
import \test::func::LoadFile;

//
//lexical Aas = [a-z]+ !>> [a-z];
//
//
//syntax Variable = Aas type Aas name;
//
//syntax Declaration = Variable*; 
//
//layout L = [\ ]*;

public void main(list[str] param = []){
	str rust_file = LoadFile("let_integers");
	renderParsetree([start[Crate]]rust_file);
	iprintln(diagnose([start[Crate]]rust_file));
}
