@contributors{Adrian Zborowski - 11336544 - adrian.zborowski@uva.nl}

module main

// Standard libraries
import Prelude;
import analysis::grammars::Ambiguity;
import vis::ParseTree;

// Custom libraries
import lang::rust::\syntax::Rust;
//import \test::func::LoadFile;

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

public void main(){
	list[loc] files = Walk(|project://oxidize/rust|, ".rs");
	ParseStats(files);
}

public list[loc] Walk(loc a, str pattern){
	list[loc] files = [];

	for (entry <- listEntries(a)){
		if(endsWith(entry, pattern)){
			files += (a+entry);
		}elseif(isDirectory(a+entry)){
			files += Walk(a+entry, pattern);
		}
	}
			
	return files;
}

public void ParseStats(list[loc] files){
	list[loc] parsed = [];
	list[loc] failed = [];
	list[loc] amb = [];

	for(file <- files){
		str input_file = readFile(file);
		
		try{
			Tree parse_tree = [start[Crate]]input_file;
			parsed += file;
			
			if(/amb(_) := parse_tree){
				amb += file;
			}
		}catch ParseError(_):
			failed += file;
	}
	
	iprintln(take(8, failed));
	
	println("Total files: 	<size(files)>");
	println("Parsed: 	<size(parsed)>");
	println("Failed: 	<size(failed)>");
	println("Amb: 		<size(amb)>");
}
