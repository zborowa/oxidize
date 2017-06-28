@contributors{Adrian Zborowski - ak.zborowski@gmail.com}

module util::Parse

import IO;
import ParseTree;
import Exception;

import lang::rust::\syntax::Rust;

@doc{
.Synopsis
Parsing of the source files by using the given location of the project directory.

.Usage
Parse(<list of locs>);

.Description
Return a list of parse trees parsed with the given grammar and the given starting ruleset. The parse trees have an 
attached location with them.
}
public list[Tree] Parse(list[loc] source_locs, bool verbose=false){
	list[loc] parsed = [];
	list[loc] failed = [];
	list[loc] ambi = [];
	
	list[Tree] source_trees = [];
	
	for(source_loc <- source_locs){
		try{
			Tree source_tree = parse(#start[Crate], source_loc, allowAmbiguity=true);
			
			parsed += source_loc;
			if(/t:amb(_) := source_tree){
				//println("<source_loc>\n<t>");
				ambi += source_loc;
			}
			
			source_tree @ \loc = source_loc;
			source_trees += source_tree;
		}catch ParseError(_):{
			failed += source_loc;
		}
	}
	
	if(verbose){
		int show = 8;
		if(size(failed)>0){
			println("Example of <(size(failed)>show)?size(failed):show> failed files:");
			iprintln(take(show, failed));
		}
		
		if(size(ambi)>0){
			println("Example of <(size(ambi)>show)?size(ambi):show> ambiguous files:");
			iprintln(take(show, ambi));
		}
		
		println("Total files: 	<size(source_locs)>");
		println("Parsed: 	<size(parsed)>");
		println("Failed: 	<size(failed)>");
		println("Amb: 		<size(ambi)>");
	}
	
	return source_trees;
}
