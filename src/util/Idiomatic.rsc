module util::Idiomatic

import IO;
import ParseTree;
import lang::rust::\syntax::Rust;

public start[Crate] idiomatic(start[Crate] crate) = innermost visit(crate){

	/*
	Delete unused lifetime from statements which declare it.
	*/
	//Expression_while
	case (Expression_while) `<Lifetime lt>: while <Expression cond> {
						   '	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
						   '}` => 
		 (Expression_while) `while <Expression cond> {
						   '	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
						   '}`
	  when !used_lifetime(crate, (Lifetime) `<Lifetime lt>`)
	  
	// Expression_while_let
	case (Expression_while_let) `<Lifetime lt>: while let <Pattern ptr> = <Expression cond> {
						   		'	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
						   		'}` => 
		 (Expression_while_let) `while let <Pattern ptr> = <Expression cond> {
						   		'	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
						   		'}`
	  when !used_lifetime(crate, (Lifetime) `<Lifetime lt>`)
	
	// Expression_loop
	case (Expression_loop) `<Lifetime lt>: loop {
						   '	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
						   '}` => 
		 (Expression_loop) `loop {
						   '	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
						   '}`
	  when !used_lifetime(crate, (Lifetime) `<Lifetime lt>`)
	  
	// Expression_for
	case (Expression_for) `<Lifetime lt>: for <Pattern ptr> in <Expression cond> {
						  '	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
						  '}` => 
		 (Expression_for) `for <Pattern ptr> in <Expression cond> {
						  '	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
						  '}`
	  when !used_lifetime(crate, (Lifetime) `<Lifetime lt>`)

	/*
	Transform `loop` statements containing a conditional `if` statement with a `break` statement into a `while` statement. 
	*/
	case (Block_expression) `loop {
							'	if !(<Expression cond>) {
							'		break;
							'	}
							'	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
							'}` => 
		 (Block_expression) `while <Expression cond> {
		 					'	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
		 					'}`
	
	/*
	Ensure the safety of the pointer already being checked and not being null.
	TODO: Check if the expression needs to be used in the in_stmts
	*/
	case (Statement) `if !<Identifier id>.is_null() {
					 '	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
					 '}` => 
		 (Statement) `if !<Identifier id>.is_null() {
		 			 '	let <Identifier id> = NonZero::new(<Identifier id>);
		 			 '	<Statement* stmts> <Expression!blockExpr!blockStmt? expr>
		 			 '}`
	  when !in_stmts(stmts)
	
	case (Statements) `let <Path_expression id> <Type_ascription? typea> = *<Path_expression pt>;` =>
		 (Statements) `let <Path_expression id> <Type_ascription? typea> = *<Path_expression pt>;
		 			  '	let <Path_expression pt> <Type_ascription? typea> = NonZero::new(<Path_expression pt>);`
		
	case (Let) `let <Path_expression pe> <Type_ascription? typea> = <Expression e> as <Type t>;` => 
		 (Let) `let <Path_expression pe> <Type_ascription? typea> = NonZero::new(<Expression e> as <Type t>);`

	/* * * * * * * * * * * * * * * * * * * Additional clean-up transformations * * * * * * * * * * * * * * * * * * */

	case (Block_expression) `while true <Block block>`=>
		 (Block_expression) `loop <Block block>`
	
	// TODO: This works! Is just a test.
	//case (Block_item) `<Item_fn item>`:
	//	 println((Block_item) `<Item_fn item>`);
	
	//case fn_item: (Block_item) `fn <Identifier identifier> <Generic_params? generic_params> <Fn_decl fn_decl> <Where_clause? where_clause> { 
	//                           '  <Statement+ pre_stmts> 
	//                           '  while true {<Statement+ body>} 
	//                           '  <Statement+ pos_stmts> 
	//                           '}` : {
	//	println((Block_item) `<Item_fn fn_item>`);
	//	println("Found a block item.");
	//}
};

//bool in_stmts(Statement* stmts){
//	int count = 0;
//	
//	visit(stmts){
//		case (Statement) `let <Identifier id1> = NonZero::new(<Identifier id2>);`:{
//			count+=1;
//		}
//	}
//	
//	return (count==0)?false:true;
//}

// TODO: kijk hier naar
bool in_stmts(Statement* stms) = /(Statement) `let <Identifier _> = NonZero::new(<Identifier _>);` := stms;
//int counter(Statement *stms) = (0 | it + 1 | /(Statement) `let <Identifier id1> = NonZero::new(<Identifier id2>);` := stms);

public bool used_lifetime(start[Crate] crate, Lifetime lt){
	int count = 0;
	
	visit(crate){
		case _lt: (Lifetime) `<Lifetime _lt>`:{
			if(_lt := (Lifetime) `<Lifetime lt>`){
				count+=1;
			}
		}
	}
	
	return (count==1)?false:true;
}
