module util::Raii

import IO;
import Map;
import Set;
import ParseTree;
import lang::rust::\syntax::Rust;

void raii(start[Crate] crate){

	start[Crate] pre = crate;

	bla = visit(crate){
		/*
		Apply RAII if possible, this is based on the use of the `free` keyword in a specific case, this is also a specific 
		case of Corrode optimalization
		*/
		// Use this for the detection of the `free` keyword
		//case (Statement) `free(<Identifier free_id> as (*mut ::std::os::raw::c_void));`:
		//	 println("\n\n\n\n<free_id>\n\n\n\n");
		// Use this for the detection of a full function scope
		case org:(Block_item) `unsafe extern <String? st> fn <Identifier fn_id> <Generic_params? gp>
							  '<Fn_decl params> <Where_clause? wc>
							  '{ 
							  '<Inner_attribute* ia>
							  '<Statements stmts>
							  '}` =>
				 (Block_item) `unsafe extern <String? st> fn <Identifier fn_id> <Generic_params? gp>
							  '<Fn_decl params> <Where_clause? wc>
							  '{ 
							  '<Inner_attribute* ia>
							  '<Statements fun>
							  '}`
			when /(Statements) `free(<Identifier _> as (*mut ::std::os::raw::c_void));` := stmts,
				 ids := detect_identifiers(stmts),
				 fun := correct_ptrs(stmts, ids)
	};
	
	start[Crate] post = crate;
	
	println(bla);
}
//
set[Identifier] detect_identifiers(Statements stmts){
	set[Identifier] ids = {};
	visit(stmts){
		case (Statement) `free(<Identifier id> as (*mut ::std::os::raw::c_void));`: ids += id;
	}
	return ids;
}

Statements correct_ptrs(Statements fun, set[Identifier] ids) = visit(fun){
	//case (Statement) `` :;
	case (Expression) `<Expression f_call> (<Expressions exprs>)` => 
		 (Expression) `<Expression f_call> (<Expressions mod_exprs>)`
		when mod_exprs := ptr_fn(exprs, ids)
};

Expressions ptr_fn(Expressions exprs, set[Identifier] free_ids) = visit(exprs){
	case (Expression) `<Identifier id> as (*const u8)` =>
		 (Expression) `<Identifier id>.as_mut_ptr()`
		when id in free_ids
};
