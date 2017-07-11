module util::Raii

import IO;
import Map;
import Set;
import ParseTree;
import lang::rust::\syntax::Rust;

//Tree raii(Tree crate) = visit(crate){
start[Crate] raii(start[Crate] crate) = visit(crate){
	/*
	Apply RAII if possible, this is based on the use of the `free` keyword in a specific case, this is also a specific 
	case of Corrode optimalization
	*/
	case org:(Block_item) `unsafe extern <String? st> fn <Identifier fn_id> <Generic_params? gp> <Fn_decl params> <Where_clause? wc> { 
						  '<Inner_attribute* ia>
						  '<Statements stmts>
						  '}` =>
			 (Block_item) `unsafe extern <String? st> fn <Identifier fn_id> <Generic_params? gp> <Fn_decl params> <Where_clause? wc> {
						  '<Inner_attribute* ia>
						  '<Statements fun>
						  '}`
		when /(Statements) `free(<Identifier f_id> as (*mut ::std::os::raw::c_void));` := stmts,
			 id_in_scope(stmts, f_id),
			 ids := detect_identifiers(stmts),
			 rf := remove_free(stmts, ids),
			 pd := ptr_decl(rf, ids),
			 cv := correct_void(pd, ids),
			 cp := correct_ptrs(cv, ids),
			 cn := correct_isnull(cp, ids),
			 cf := correct_fcall(cn, ids),
			 fun := correct_expr(cf)
};

/* ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- */

bool id_in_scope(Statements stmts, Identifier id) = 
	/(Let) `let <Binding_mode bm> <Identifier did> <Type_ascription ty>;` := stmts && did := id;
	
set[Identifier] detect_identifiers(Statements stmts){
	set[Identifier] ids = {};
	visit(stmts){
		case (Statement) `free(<Identifier id> as (*mut ::std::os::raw::c_void));`:{
			ids += id;
		}
	}
	
	return ids;
}

Statements remove_free(Statements stmts, set[Identifier] ids) = visit(stmts){
	case org:(Statement) `free(<Identifier id> as (*mut ::std::os::raw::c_void));` => 
		 	 (Statement) `{}`
		when id in ids
};

Statements ptr_decl(Statements stmts, set[Identifier] ids) = visit(stmts){
	case (Let) `let <Binding_mode bm> <Identifier id> : <Type_sum ts>;` => 
		 (Let) `let <Binding_mode bm> <Identifier id> : MArray\<u8\>;`
		when id in ids,
			 (Type_sum) `*mut u8` := ts

	case (Let) `let <Binding_mode bm> <Identifier id> : <Type_sum ts> = <Expression expr>;` => 
		 (Let) `let <Binding_mode bm> <Identifier id> : <Type_sum oma> = <Expression mod_expr>;`
		when id in ids,
			 (Type_sum) `*mut u8` := ts,
			 mod_expr := encap_expr(expr),
			 oma := option_marray(mod_expr)
};

Expression encap_expr(Expression expr) = bottom-up-break visit(expr){
	case (Expression) `0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)` =>
		 (Expression) `None`
};

Type_sum option_marray(Expression expr){
	if((Expression) `None` := expr){
		return (Type_sum) `Option\<MArray\<u8\>\>`;
	}
	
	return (Type_sum) `MArray\<u8\>`;
}

Statements correct_void(Statements stmts, set[Identifier] ids) = visit(stmts){
	case (Statement) `<Identifier id> = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);` => 
		 (Statement) `<Identifier id> = None;`
		when id in ids
};

Statements correct_ptrs(Statements stmts, set[Identifier] ids) = visit(stmts){
	case (Expression) `<Expression f_call>(<Expressions exprs>)` => 
		 (Expression) `<Expression f_call>(<Expressions mod_exprs>)`
		when mod_exprs := ptr_fn(exprs, ids)
		
	case (Statement) `<Identifier aid> = <Identifier id>;` => 
		 (Statement) `<Identifier aid> = <Identifier id>.as_mut_ptr();`
		when id in ids 
};

Expressions ptr_fn(Expressions exprs, set[Identifier] ids) = visit(exprs){
	case (Expression) `<Identifier id> as (*const u8)` =>
		 (Expression) `<Identifier id>.as_mut_ptr()`
		when id in ids
};

Statements correct_isnull(Statements stmts, set[Identifier] ids) = visit(stmts){
	case (Expression) `<Identifier id>.is_null()` => 
		 (Expression) `<Identifier id>.as_mut_ptr().is_null()`
		when id in ids
		
	case (Expression) `!<Identifier id>.is_null()` => 
		 (Expression) `!<Identifier id>.as_mut_ptr().is_null()`
		when id in ids
};

Statements correct_fcall(Statements stmts, set[Identifier] ids) = visit(stmts){
	case (Statement) `<Identifier id> = <Expression expr>;` => 
		 (Statement) `<Identifier id> = <Expression mod_expr>;`
		when id in ids,
			 mod_expr := not_none_raw(expr)
		
	case (Statement) `let <Binding_mode bm> <Identifier id> : MArray\<u8\> = <Expression expr>;` => 
		 (Statement) `let <Binding_mode bm> <Identifier id> : MArray\<u8\> = <Expression mod_expr>;`
		when id in ids,
			 mod_expr := not_none_raw(expr)
};

Expression not_none_raw(Expression expr){
	if((Expression) `None` !:= expr){
		return (Expression) `MArray::from_raw(<Expression expr>)`;
	}
	
	return expr;
}

Statements correct_expr(Statements stmts){
	stmts = visit(stmts){
		case (Statements) `let mut <Identifier id> : Option\<MArray\<u8\>\> = None;
						  '<Statement* post_stmts>` => 
			 (Statements) `let mut <Identifier id> : Option\<MArray\<u8\>\> = None;
						  '<Statement* ptr_stmts>`
			when ass_stmts := correct_assignments(post_stmts, id),
				 ptr_stmts := correct_pcalls(ass_stmts, id)
	};
	
	stmts = visit(stmts){
		case (Statements) `<Statement* pre_stmts>
						  'let mut <Identifier id> : MArray\<u8\>;
						  '<Statement* post_stmts>` => 
			 (Statements) `<Statement* pre_stmts>
						  'let mut <Identifier id> : MArray\<u8\>;
						  '<Statement* cc>`
			when cc := correct_comparisons(post_stmts, id)
	};
	
	stmts = innermost visit(stmts){
		case (Statements) `<Statement* pre_stmts>
						  'let mut <Identifier id> : MArray\<u8\>;
						  '<Statement* post_stmts>` => 
			 (Statements) `<Statement* pre_stmts>
			 			  'let mut <Identifier id> : Option\<MArray\<u8\>\>;
			 			  '<Statement* mod_stmts>`
			when check_none(post_stmts, id),
				 cs := correct_assignments(post_stmts, id),
				 cc := correct_comparisons(cs, id),
				 mod_stmts := correct_pcalls(cc, id)
	};
	
	stmts = visit(stmts){
		case (Statements) `<Statement* pre_stmts>
						  'let mut <Identifier id> : Option\<MArray\<u8\>\>;
						  '<Statement* post_stmts>` =>
			 (Statements) `<Statement* pre_stmts>
						  'let mut <Identifier id> : Option\<MArray\<u8\>\>;
						  '<Statement* mod_stmts>`
			when mod_stmts := correct_var_uses(post_stmts, id)
	};

	return stmts;
}

Statement* correct_var_uses(Statement* stmts, Identifier id){
	stmts = visit(stmts){
		case (Expression) `<Expression l_expr> as <Type t>` => 
			 (Expression) `<Expression mod_expr> as <Type t>`
			when mod_expr := adjust_to_type(l_expr, id, t)
	};
	
	return stmts;
}

Expression adjust_to_type(Expression expr, Identifier id, Type t) = visit(expr){
	case (Expression) `<Identifier f_id>` => 
		 (Expression) `<Identifier f_id>.unwrap().as_mut_ptr()`
		when f_id := id,
			 (Type) `(*const u8)` := t
};

bool check_none(Statement* stmts, Identifier id) = 
	/(Statement) `<Identifier f_id> = None;` := stmts 
	|| /(Expression) `<Identifier f_id> == None` := stmts
	|| /(Expression) `<Identifier f_id> != None` := stmts
	&& f_id := id;

Statement* correct_assignments(Statement* stmts, Identifier id) = visit(stmts){
	case (Statement) `<Identifier f_id> = <Expression expr>;` => 
		 (Statement) `<Identifier f_id> = Some(<Expression expr>);`
		when f_id := id,
			 (Expression) `None` !:= expr,
			 (Expression) `Some(<Expression _>)` !:= expr
			 
	case (Statement) `<Identifier l_id> = <Identifier r_id> as (*const u8);` => 
		 (Statement) `<Identifier l_id> = <Identifier r_id>.unwrap().as_mut_ptr();`
		when r_id := id
};

Statement* correct_comparisons(Statement* stmts, Identifier id) = visit(stmts){
	case (Expression) `<Identifier f_id> == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)` => 
		 (Expression) `<Identifier f_id> == None`
		when f_id := id
		
	case (Expression) `<Identifier f_id> != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)` => 
		 (Expression) `<Identifier f_id> != None`
		when f_id := id
};

Statement* correct_pcalls(Statement* stmts, Identifier id) = visit(stmts){
	case (Expression) `<Identifier f_id>.as_mut_ptr()` => 
		 (Expression) `<Identifier f_id>.unwrap().as_mut_ptr()`
		when f_id := id
		
	case (Expression) `!<Identifier f_id>.as_mut_ptr()` => 
		 (Expression) `!<Identifier f_id>.unwrap().as_mut_ptr()`
		when f_id := id
};

//Expression not_none_raw(Expression expr) = bottom-up-break visit(expr){
//	case (Expression) `<Expression expr2>` => 
//		 (Expression) `MArray::from_raw(<Expression expr2>)`
//		when (Expression) `None` !:= expr2
//};
