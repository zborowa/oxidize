@contributors{Adrian Zborowski - ak.zborowski@gmail.com}

module Oxidize

// Std
import IO;
import String;
import DateTime;
import ParseTree;
import Exception;

// Project
import lang::rust::\syntax::Rust;

public void Oxidize(loc project_path, str extension=".rs", bool duration=true){
	datetime timer_start = now();
	
	list[loc] source_locs = Walk(project_path, extension);
	list[Tree] source_trees = Parse(source_locs);
	
	Duration timer_duration = now() - timer_start;
	duration ? Timer(timer_duration);
}

@doc{
.Synopsis
Parsing of the source files by using the given location of the project directory.

.Usage
Parse(<list of locs>);

.Description
Return a list of parse trees parsed with the given grammar and the given starting ruleset. The parse trees have an 
attached location with them.
}
public list[Tree] Parse(list[loc] source_locs, bool verbose=true){
	list[loc] parsed = [];
	list[loc] failed = [];
	list[loc] ambi = [];
	
	list[Tree] source_trees = [];
	
	for(source_loc <- source_locs){
		try{
			Tree source_tree = parse(#start[Crate], source_loc, allowAmbiguity=true);
			
			parsed += source_loc;
			if(/t:amb(_) := source_tree){ 
				ambi += source_loc;
			}
			
			source_tree @ \loc = source_loc;
			source_trees += source_tree;
		}catch ParseError(_):{
			failed += source_loc;
			println("There was a parse error with the file \"<source_loc>\".");
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

@doc{
.Synopsis
Traverse the directories present in the given project path.

.Usage
Walk(|<Uri>|, extension);

.Description
Traversal function for the given project path. It recursively checks if an entry is a directory and traverser into it 
to find files.
}
public list[loc] Walk(loc project_path, str extension){
	list[loc] files = [];

	for (entry <- listEntries(project_path)){
		if(endsWith(entry, extension)){
			files += (project_path+entry);
		}elseif(isDirectory(project_path+entry)){
			files += Walk(project_path+entry, extension);
		}
	}
			
	return files;
}

@doc{
.Synopsis
Visual print of the projects duration timer

.Usage
Timer(<Duration>);

.Description
A visual representation of a Duration type. Only showing the values which are not 0 (zero). With addition extension 
appropriate for its value.
}
public void Timer(Duration timer_duration){
	timer_string = "";
	
	timer_string += ((timer_duration.hours!=0) ? "<timer_duration.hours>h " : "");
	timer_string += ((timer_duration.minutes!=0) ? "<timer_duration.minutes>m " : "");
	timer_string += ((timer_duration.seconds!=0) ? "<timer_duration.seconds>s " : "");
	timer_string += ((timer_duration.milliseconds!=0) ? "<timer_duration.milliseconds>ms " : "");
	!isEmpty(timer_string) ? println(timer_string);
}
