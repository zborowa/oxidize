module util::Idiomatic

import IO;
import Map;
import ParseTree;
import lang::rust::\syntax::Rust;

Tree idiomatic(Tree crate) = innermost visit(crate){
//start[Crate] idiomatic(start[Crate] crate) = innermost visit(crate){

	/*
	Delete unused lifetime from statements which declare it.
	*/
	//Expression_while
	case (Expression_while) `<Lifetime lt>: while <Expression cond> {
						   '	<Statement* stmts> <Expression? expr>
						   '}` => 
		 (Expression_while) `while <Expression cond> {
						   '	<Statement* stmts> <Expression? expr>
						   '}`
	  when !used_lifetime(stmts, lt)
	  
	// Expression_while_let
	case (Expression_while_let) `<Lifetime lt>: while let <Pattern ptn> = <Expression cond> {
						   		'	<Statement* stmts> <Expression? expr>
						   		'}` => 
		 (Expression_while_let) `while let <Pattern ptn> = <Expression cond> {
						   		'	<Statement* stmts> <Expression? expr>
						   		'}`
	  when !used_lifetime(stmts, lt)
	
	// Expression_loop
	case (Expression_loop) `<Lifetime lt>: loop {
						   '	<Statement* stmts> <Expression? expr>
						   '}` => 
		 (Expression_loop) `loop {
						   '	<Statement* stmts> <Expression? expr>
						   '}`
	  when !used_lifetime(stmts, lt)
	  
	// Expression_for
	case (Expression_for) `<Lifetime lt>: for <Pattern ptn> in <Expression cond> {
						  '	<Statement* stmts> <Expression? expr>
						  '}` => 
		 (Expression_for) `for <Pattern ptn> in <Expression cond> {
						  '	<Statement* stmts> <Expression? expr>
						  '}`
	  when !used_lifetime(stmts, lt)

	/*
	Transform `loop` statements containing a conditional `if` statement with a `break` statement into a `while` statement. 
	*/
	case (Block_expression) `loop {
							'	if !(<Expression cond>) {
							'		break;
							'	}
							'	<Statement* stmts> <Expression? expr>
							'}` => 
		 (Block_expression) `while <Expression cond> {
		 					'	<Statement* stmts> <Expression? expr>
		 					'}`

	/* * * * * * * * * * * * * * * * * * * Additional clean-up transformations * * * * * * * * * * * * * * * * * * */

	case (Block_expression) `while true <Block block>`=>
		 (Block_expression) `loop <Block block>`
};

bool used_lifetime(Statement* stmts, Lifetime lt) = /lt := stmts;

// TODO: kijk hier naar
//bool let_nonzero_in_stmt(Statement* stms) = /(Statement) `let <Identifier _> = NonZero::new(<Identifier _>);` := stms;
//int counter(Statement *stms) = (0 | it + 1 | /(Statement) `let <Identifier id1> = NonZero::new(<Identifier id2>);` := stms);
//int used_lifetime_temp(Block_item bi, Lifetime lt) = (0 | it + 1 | /lt := bi);
