module util::Idiomatic

import lang::rust::\syntax::Rust;

start[Crate] idiomatic(start[Crate] crate) = innermost visit(crate){

	case (Statement) `<Lifetime lt>: loop {if !(<Expression cond>) {break;} <Statement+ stmts>}` => 
		 (Statement) `while <Expression cond> {<Statement+ stmts>}`
		 
	case (Statement) `while true{<Statement+ body>}` => 
		 (Statement) `loop{<Statement+ body>}`
};
