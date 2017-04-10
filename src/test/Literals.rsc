module \test::Literals

import Prelude;
import vis::ParseTree;

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

test bool LiteralByteString(){
	str input_file = LoadFile("let_byte_strings");
	Tree parse_tree = parse(#Crate, input_file);
	
	return /amb(_) !:= parse_tree;
}

test bool LiteralByteStringRaw(){
	str input_file = LoadFile("let_byte_strings_raw");
	Tree parse_tree = parse(#Crate, input_file);
	
	return /amb(_) !:= parse_tree;
}

test bool LiteralByte(){
	str input_file = LoadFile("let_bytes");
	Tree parse_tree = parse(#Crate, input_file);
	
	return /amb(_) !:= parse_tree;
}

test bool GuessingGame(){
	str input_file = LoadFile("guessing_game");
	//Tree parse_tree = parse(Crate, input_file);
	renderParsetree([start[Crate]]input_file);
	
	//return /amb(_) !:= parse_tree;
	return true;
}
