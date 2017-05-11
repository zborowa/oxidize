module parse

import Prelude;
import ParseTree;
import lang::rust::\syntax::Rust;

public list[Tree] Parse(loc parse_loc){
	list[loc] parse_locs = Walk(parse_loc, ".rs");
	
	list[Tree] parse_trees= [];
	for(file_loc <- parse_locs){
		parse_tree = parse(#start[Crate], file_loc, allowAmbiguity=true);
		//parse_trees += parse_tree;
	}
	
	return parse_trees;
}

private list[loc] Walk(loc a, str pattern){
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
