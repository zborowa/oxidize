@contributors{Adrian Zborowski - 11336544 - adrian.zborowski@uva.nl}

module main

// Standard libraries
import Prelude;
import ParseTree;
import Exception;
import vis::ParseTree;
import analysis::grammars::Ambiguity;

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

public void main(){
	datetime project_start = now();
	list[loc] files = Walk(|project://oxidize/cvs-rs|, ".rs");
	ParseStats(files);
	Duration duration = now() - project_start;
	println("<duration.hours>h <duration.minutes>m <duration.seconds>s <duration.milliseconds>ms");
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
	list[loc] ambi = [];

	for(file <- files){
		// Show the file to parse
		println(file);
		str input_file = readFile(file);
		
		try{
			Tree parse_tree = [start[Crate]]input_file;
			parsed += file;
			
			if(/t:amb(_) := parse_tree){ 
			    //println("AMB: \'<t>\'");
				ambi += file;
			}
		}catch ParseError(_):
			failed += file;
	}
	
	println("Failed files:");
	iprintln(take(16, failed));
	
	println("Amb files:");
	iprintln(take(8, ambi));
	
	println("Total files: 	<size(files)>");
	println("Parsed: 	<size(parsed)>");
	println("Failed: 	<size(failed)>");
	println("Amb: 		<size(ambi)>");
}
