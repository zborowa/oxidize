@contributors{Adrian Zborowski - ak.zborowski@gmail.com}

module util::Timer

import IO;
import String;
import DateTime;

@doc{
.Synopsis
Visual print of the projects duration timer

.Usage
Timer(<Duration>);

.Description
A visual representation of a Duration type. Only showing the values which are not 0 (zero). With addition extension 
appropriate for its value.
}
public str Timer(Duration timer_duration){
	timer_string = "Oxidize completed after:\n\t\t";
	
	timer_string += ((timer_duration.hours!=0) ? "<timer_duration.hours>h " : "");
	timer_string += ((timer_duration.minutes!=0) ? "<timer_duration.minutes>m " : "");
	timer_string += ((timer_duration.seconds!=0) ? "<timer_duration.seconds>s " : "");
	timer_string += ((timer_duration.milliseconds!=0) ? "<timer_duration.milliseconds>ms" : "");
	return timer_string;
}
