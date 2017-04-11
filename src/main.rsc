@contributors{Adrian Zborowski - 11336544 - adrian.zborowski@uva.nl}

module main

// Standard libraries
import Prelude;
import analysis::grammars::Ambiguity;
import vis::ParseTree;

// Custom libraries
import lang::rust::\syntax::Rust;
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

// TODO: if (/amb(_) := t) { do your thing; } check in a loop if all std of Rust are parsable

public void main(list[str] param = []){
	
}
