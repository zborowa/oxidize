module prec

import ParseTree;
import vis::ParseTree;

lexical I
	= [0-9]+
	;
	
syntax A
	= left ( A "*" A
		   | A "/" A
		   )
	| left ( A "+" A
		   | A "-" A
		   )
	> I
	;
	
syntax B
	= left B "*" B
	| left B "/" B
	> left B "+" B
	| left B "-" B
	| I
	;
	
syntax C
	= "12" "!"
	> I "!"
	;
	
start syntax P
	= C*
	;
