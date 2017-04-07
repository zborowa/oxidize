@contributors{Adrian Zborowski - 11336544 - adrian.zborowski@uva.nl}

module main

import Prelude;

import analysis::grammars::Ambiguity;

import vis::ParseTree;

import lang::rust::\syntax;

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
	str rust_file = readFile(|project://oxidize/src/test/input/fn_main_println.rs|);
	renderParsetree([start[Crate]]rust_file);
	iprintln(diagnose([start[Crate]]rust_file));
}
