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
						   '	<Statement* stmts> <Expression? expr>
						   '}` => 
		 (Expression_while) `while <Expression cond> {
						   '	<Statement* stmts> <Expression? expr>
						   '}`
	  when !used_lifetime(crate, lt)
	  
	// Expression_while_let
	case (Expression_while_let) `<Lifetime lt>: while let <Pattern ptn> = <Expression cond> {
						   		'	<Statement* stmts> <Expression? expr>
						   		'}` => 
		 (Expression_while_let) `while let <Pattern ptn> = <Expression cond> {
						   		'	<Statement* stmts> <Expression? expr>
						   		'}`
	  when !used_lifetime(crate, lt)
	
	// Expression_loop
	case (Expression_loop) `<Lifetime lt>: loop {
						   '	<Statement* stmts> <Expression? expr>
						   '}` => 
		 (Expression_loop) `loop {
						   '	<Statement* stmts> <Expression? expr>
						   '}`
	  when !used_lifetime(crate, lt)
	  
	// Expression_for
	case (Expression_for) `<Lifetime lt>: for <Pattern ptn> in <Expression cond> {
						  '	<Statement* stmts> <Expression? expr>
						  '}` => 
		 (Expression_for) `for <Pattern ptn> in <Expression cond> {
						  '	<Statement* stmts> <Expression? expr>
						  '}`
	  when !used_lifetime(crate, lt)

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
	
	/*
	Ensure the safety of the pointer already being checked and not being null.
	TODO: Check if the expression needs to be used in the in_stmts
	*/
	case (Statement) `if !<Identifier id>.is_null() {
					 '	<Statement* stmts> <Expression? expr>
					 '}` => 
		 (Statement) `if !<Identifier id>.is_null() {
		 			 '	let <Identifier id> = NonZero::new(<Identifier id>);
		 			 '	<Statement* stmts> <Expression? expr>
		 			 '}`
	  when !let_nonzero_in_stmt(stmts, id)
	
	case (Statements) `let <Path_expression id> <Type_ascription? typea> = *<Path_expression pt>;` =>
		 (Statements) `let <Path_expression id> <Type_ascription? typea> = *<Path_expression pt>;
		 			  'let <Path_expression pt> <Type_ascription? typea> = NonZero::new(<Path_expression pt>);`
		
	case (Let) `let <Path_expression id> <Type_ascription? typea> = <Expression e> as <Type t>;` => 
		 (Let) `let <Path_expression id> <Type_ascription? typea> = NonZero::new(<Expression e> as <Type t>);`

	/* * * * * * * * * * * * * * * * * * * Additional clean-up transformations * * * * * * * * * * * * * * * * * * */

	case (Block_expression) `while true <Block block>`=>
		 (Block_expression) `loop <Block block>`
	
	//case fn_item: (Block_item) `fn <Identifier identifier> <Generic_params? generic_params> <Fn_decl fn_decl> <Where_clause? where_clause> { 
	//                           '  <Statement+ pre_stmts> 
	//                           '  while true {<Statement+ body>} 
	//                           '  <Statement+ pos_stmts> 
	//                           '}` : {
	//	println((Block_item) `<Item_fn fn_item>`);
	//	println("Found a block item.");
	//}
};

//Block_item scoped(Block_item org_bi){
//	Block_item new_bi = org_bi;
//	
//	bottom-up-break visit(new_bi){
//		case (Expression_while) `<Lifetime lt>: while <Expression cond> {
//						   		'	<Statement* stmts> <Expression? expr>
//						   		'}`:{
//						   			if(used_lifetime_temp(new_bi, lt) > 1){
//							   			insert (Expression_while) `while <Expression cond> {
//							   									  '	<Statement* stmts> <Expression? expr>
//							   									  '}`;
//							   			return new_bi;
//						   			}
//			}
//	}
//	
//	return org_bi;
//}
//int used_lifetime_temp(Block_item bi, Lifetime lt) = (0 | it + 1 | /lt := bi);

bool let_nonzero_in_stmt(Statement* stmts, Identifier id){
	visit(stmts){
		case (Statement) `let <Identifier id1> = NonZero::new(<Identifier id2>);`:{
			if(id1 == id2 && id == id1){
				return true;
			}
		}
	}
	
	return false;
}

bool used_lifetime(start[Crate] crate, Lifetime lt){
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

// TODO: kijk hier naar
//bool let_nonzero_in_stmt(Statement* stms) = /(Statement) `let <Identifier _> = NonZero::new(<Identifier _>);` := stms;
//int counter(Statement *stms) = (0 | it + 1 | /(Statement) `let <Identifier id1> = NonZero::new(<Identifier id2>);` := stms);
