@contributors{Adrian Zborowski - ak.zborowski@gmail.com}

module Oxidize

// Std
import IO;
import List;
import String;
import DateTime;
import ParseTree;

// Project
import util::Walk;
import util::Parse;
import util::Timer;
import util::Idiomatic;

/* NOTES:
Don't forget to add the following line of code at the top of each crate which is using NonZero:
	extern crate core;
	use self::core::nonzero::NonZero;

If the NonZero is used don't forget to add the following line of code at the top of the main file
(in case of library in `lib.rs` and in case of an application in `main.rs`)
	#![feature(nonzero)]
*/

@doc{
.Synopsis
Commandline `main` function which is required to run by the interpreter

.Usage
main([]);

.Description
This is a function needed for the interpreter as the access point to the project. It can only invoke the `Oxidize` 
function which runs the project.
}
public void main(list[str] args){
	loc project_path = |file:///|;
	str extension = ".rs";
	
	if(args[0]?){
		project_path = toLocation(args[0]);
	}
	
	if(args[1]?){
		extension = args[1];
	}
	
	if(!isEmpty(project_path.authority)){
		Oxidize(project_path, extension=extension);
	}else{
		println("There was no location string provided. Examples usage:\n\tjava -Xmx1G -Xss32m -jar rascal.jar Oxidize.rsc \"|file://\<path\>/rs-project|\"");
	}
}

public void Oxidize(loc project_loc, str extension=".rs", bool verbose=false){
	datetime timer_start = now();
	
	list[loc] source_locs = Walk(project_loc, extension);
	
	list[Tree] source_trees = Parse(source_locs);
	
	int count = 0;
	for(st <- source_trees){
	
		if(verbose){
			count += 1;
			print("\rProcessing file <count> out of <size(source_trees)>...");
		}
	
		str project_path = project_loc.path;
		str file_path = st@\loc.path;
		
		str new_project_path = (project_loc.parent + (project_loc.file + "_idiom")).path;
		loc new_file_path = toLocation("file://" + replaceFirst(file_path, project_path, new_project_path));
		
		Tree idiomatic = idiomatic(st);
		
		writeFile(new_file_path, idiomatic);
	}
	if(verbose){
		print("\n");
	}
	
	Duration timer_duration = now() - timer_start;
	println(Timer(timer_duration));
}
