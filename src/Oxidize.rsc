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
import util::Raii;
import util::Parse;
import util::Timer;
import util::Correct;
import util::Cleanup;
import util::Idiomatic;

/* NOTES:
Example of how to run in the terminal:
	java -Xmx1G -Xss32m -jar rascal.jar Oxidize.rsc -v /Users/AdrianZ/Projects/cwi/cvs-rs
	
Example of how to run in Eclipse:
	Oxidize(|file:///Users/AdrianZ/Projects/cwi/cvs-rs|, verbose=true);

If the NonZero is used don't forget to add the following line of code at the top of the main file
(in case of library in `lib.rs` and in case of an application in `main.rs`)
	#![feature(nonzero)]

Don't forget to add the following line of code at the top of each crate which is using NonZero:
	extern crate core;
	use self::core::nonzero::NonZero;
	extern crate mbox;
	use self::mbox::MArray;
	
If the pointer variable which is valid for the nonzero transformation is used somewhere else it 
should not be transformed into a nonzero pointer. This is for the transformation safety.
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
	str usage = "usage:\tjava -Xmx1G -Xss32m -jar rascal.jar Oxidize.rsc [-v] /\<path-from-root\>/\<target\>";
	str insufficient_parameters = "There seems to be an insufficient amount of passed through variables. Please consult the usage guide.";
	loc project_path;
	bool verbose = false;
	
	if("-v" in args){
		verbose = true;
	}
	
	try{
		loc target = |file:///| + args[-1];
		if(exists(target) && isDirectory(target)){
			project_path = target;
		}
	} catch IndexOutOfBounds(int i):
		println(insufficient_parameters);
	  catch EmptyList():
		println(insufficient_parameters);
	
	if(project_path? && project_path.path?){
		Oxidize(project_path, verbose=verbose);
	}else{
		println(usage);
	}
}

public void Oxidize(loc project_loc, str extension=".rs", bool verbose=false){
	datetime timer_start = now();
	
	list[loc] source_locs = Walk(project_loc, extension);
	
	list[Tree] source_trees = Parse(source_locs, verbose=verbose);
	
	int count = 0;
	for(st <- source_trees){
	
		if(verbose){
			count += 1;
			print("\rProcessing file <count> out of <size(source_trees)>...");
		}
	
		str project_path = project_loc.path;
		str file_path = st@\loc.path;
		
		str new_project_path = (project_loc.parent + (project_loc.file + "_idiom")).path;
		loc new_file_path = |file:///| + replaceFirst(file_path, project_path, new_project_path);
		
		Tree idiomatic = idiomatic(st);
		Tree raii = raii(idiomatic);
		Tree correct = correct(raii);
		Tree cleanup = cleanup(correct);
		
		writeFile(new_file_path, cleanup);
	}
	if(verbose){
		print("\n");
	}
	
	Duration timer_duration = createDuration(timer_start, now());
	println(Timer(timer_duration));
}
