module util::Cleanup

import IO;
import Map;
import Set;
import ParseTree;
import lang::rust::\syntax::Rust;

//Tree cleanup(Tree crate) = visit(crate){
start[Crate] cleanup(start[Crate] crate){
	
	crate = innermost visit(crate){
		case (Statements) `let mut tmp : *mut ::std::os::raw::c_void = <Identifier _> as (*mut ::std::os::raw::c_void);
						  '<Statement* stmts1>
						  'free(tmp);
						  '<Statement* stmts2>` =>
			 (Statements) `<Statement* stmts1>
			 			  '<Statement* stmts2>`
			 			  
		case (Statements) `<Statement* stmts3>
						  'let mut tmp : *mut ::std::os::raw::c_void = <Identifier _> as (*mut ::std::os::raw::c_void);
						  '<Statement* stmts4>
						  'free(tmp);` =>
			 (Statements) `<Statement* stmts3>
			 			  '<Statement* stmts4>`
			 			  
		case (Statements) `<Statement* stmts5>
						  'let mut tmp : *mut ::std::os::raw::c_void = <Identifier _> as (*mut ::std::os::raw::c_void);
						  '<Statement* stmts6>
						  'free(tmp);
						  '<Statement* stmts7>` =>
			 (Statements) `<Statement* stmts5>
			 			  '<Statement* stmts6>
			 			  '<Statement* stmts7>`
	}
	
	crate = innermost visit(crate){
		case (Statement) `if <Expression _> {{}}` => 
			 (Statement) `{}`
	};
	
	return crate;
}
