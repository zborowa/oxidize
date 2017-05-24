@contributors{Adrian Zborowski - ak.zborowski@gmail.com}

module Oxidize

// Std
import IO;
import List;
import String;
import DateTime;
import ParseTree;

// Project
import util::Parse;
import util::Timer;
import util:: Walk;

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
	bool duration = false;
	
	if(size(args) > 0){
		project_path = toLocation(args[0]);
	}
	
	if(size(args) > 1){
		project_path = toLocation(args[1]);
	}
	
	if(size(args) > 2){
		project_path = toLocation(args[2]);
	}
	
	if(!isEmpty(project_path.authority)){
		Oxidize(project_path, extension=extension, duration=duration);
	}else{
		println("There was no location string provided. Examples usage:\n\tjava -Xmx1G -Xss32m -jar rascal.jar Oxidize.rsc \"|file://\<path\>/rs-project|\"");
	}
}

public void Oxidize(loc project_path, str extension=".rs", bool duration=true){
	datetime timer_start = now();
	
	list[loc] source_locs = Walk(project_path, extension);
	list[Tree] source_trees = Parse(source_locs);
	
	Duration timer_duration = now() - timer_start;
	println(Timer(timer_duration));
}
