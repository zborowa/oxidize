@contributors{Adrian Zborowski - ak.zborowski@gmail.com}

module Oxidize

// Std
import IO;
import DateTime;
import ParseTree;

// Project
import util::Parse;
import util::Timer;
import util:: Walk;

public void Oxidize(loc project_path, str extension=".rs", bool duration=true){
	datetime timer_start = now();
	
	list[loc] source_locs = Walk(project_path, extension);
	list[Tree] source_trees = Parse(source_locs);
	
	Duration timer_duration = now() - timer_start;
	println(Timer(timer_duration));
}
