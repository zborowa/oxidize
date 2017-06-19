module util::Idiomatic

import IO;
import ParseTree;
import lang::rust::\syntax::Rust;

public start[Crate] idiomatic(start[Crate] crate) = innermost visit(crate){

	/*
	Delete unused lifetime from statements which use it.
	*/
	case (Statement) `<Lifetime lt>: while <Expression expr> <Block block>` => 
		 (Statement) `while <Expression expr> <Block block>`
	  when !used_lifetime(crate, (Lifetime) `<Lifetime lt>`)
	  
	case (Statement) `<Lifetime lt>: while let <Pattern ptr> = <Expression expr> <Block block>` => 
		 (Statement) `while let <Pattern ptr> = <Expression expr> <Block block>`
	  when !used_lifetime(crate, (Lifetime) `<Lifetime lt>`)
	  
	case (Statement) `<Lifetime lt>: loop <Block block>` => 
		 (Statement) `loop <Block block>`
	  when !used_lifetime(crate, (Lifetime) `<Lifetime lt>`)
	  
	case (Statement) `<Lifetime lt>: for <Pattern ptr> in <Expression expr> <Block block>` => 
		 (Statement) `for <Pattern ptr> in <Expression expr> <Block block>`
	  when !used_lifetime(crate, (Lifetime) `<Lifetime lt>`)

	/*
	Transform `loop` statements containing a conditional statement with a `break` statement into a `while` statement. 
	*/
	case (Statement) `loop {if !(<Expression cond>) {break;} <Statement+ stmts>}` => 
		 (Statement) `while <Expression cond> {<Statement+ stmts>}`
	
	/*
	Ensure the safety of the pointer already being checked and not being null.
	*/
	case (Statement) `if !<Identifier id>.is_null() {<Statement+ stmts>}` => 
		 (Statement) `if !<Identifier id>.is_null() {let <Identifier id> = NonZero::new(<Identifier id>); <Statement+ stmts>}`
	  when !in_stmts(stmts, (Statement) `let <Identifier id> = NonZero::new(<Identifier id>);`)
	  
	case (Statements) `let <Pattern pat> <Type_ascription? typ> = *<Expression expr>;` => 
		 (Statements) `let <Pattern pat> <Type_ascription typ> = *<Expression expr>; let <Pattern expr> = NonZero::new(<Expression expr>);`

	/* * * * * * * * * * * * * * * * * * * Additional clean-up transformations * * * * * * * * * * * * * * * * * * */

	case (Statement) `while true {<Statement+ body>}`=>
		 (Statement) `loop {<Statement+ body>}`
		 
	//case (Statement) `if <Expression _> {}`=>   
	//	 (Statement) `{}`
};

// TODO: Create a new visit for empty curlies
// TODO: Create a new visit for checking if something is present more than one time in the tree

public bool in_stmts(Statement+ stmts, Statement stmt){
	int count = 0;
	
	visit(stmts){
		case _stmt: (Statement) `let <Identifier id1> = NonZero::new(<Identifier id2>);`:{
			println("found");
			count+=1;
		}
	}
	
	return (count==0)?false:true;
}

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
