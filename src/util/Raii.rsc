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
	// Use this for the detection of the `free` keyword
	//case (Statement) `free(<Identifier free_id> as (*mut ::std::os::raw::c_void));`:
	//	 println("\n\n\n\n<free_id>\n\n\n\n");
	
	// Use this for the detection of a full function scope
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
			 fun := correct_fcall(cp, ids)
};

//bool id_in_scope(Statements stmts, Identifier id) = visit(stmts){
//	case (Let) `let <Binding_mode bm> <Identifier did> <Type_ascription ty>;`:{
//		 if(did :=  id){
//		 	println("true: <id>");
//		 	return true;
//		 }
//		 println("false: <id>");
//	}
//};

bool id_in_scope(Statements stmts, Identifier id) = 
	/(Let) `let <Binding_mode bm> <Identifier did> <Type_ascription ty>;` := stmts && did := id /*&& bprintln("<did := id>: <id>")*/;
	
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
	case (Let) `let <Binding_mode bm> <Identifier id> <Type_ascription ty>;` => 
		 (Let) `let <Binding_mode bm> <Identifier id> : MArray\<u8\>;`
		when id in ids,
			 (Type_ascription) `: *mut u8` := ty

	case (Let) `let <Binding_mode bm> <Identifier id> <Type_ascription ty> = <Expression expr>;` => 
		 (Let) `let <Binding_mode bm> <Identifier id> : MArray\<u8\> = <Expression expr>;`
		when id in ids,
			 (Type_ascription) `: *mut u8` := ty
};

Statements correct_void(Statements stmts, set[Identifier] ids) = visit(stmts){
	case (Statement) `<Identifier id> = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);` => 
		 (Statement) `<Identifier id> = None;`
		when id in ids
};

Statements correct_ptrs(Statements stmts, set[Identifier] ids) = visit(stmts){
	case (Expression) `<Expression f_call>(<Expressions exprs>)` => 
		 (Expression) `<Expression f_call>(<Expressions mod_exprs>)`
		when mod_exprs := ptr_fn(exprs, ids)
};

Statements correct_fcall(Statements stmts, set[Identifier] ids) = visit(stmts){
	case (Statement) `<Identifier id> = dir_append(<Expressions exprs>);` => 
		 (Statement) `<Identifier id> = MArray::from_raw(dir_append(<Expressions exprs>));`
		when id in ids
		
	case (Statement) `let <Binding_mode bm> <Identifier id> : MArray\<u8\> = dir_append(<Expressions exprs>);` => 
		 (Statement) `let <Binding_mode bm> <Identifier id> : MArray\<u8\> = MArray::from_raw(dir_append(<Expressions exprs>));`
		when id in ids
		
	case (Statement) `<Identifier id> = Xstrdup(<Expressions exprs>);` => 
		 (Statement) `<Identifier id> = MArray::from_raw(Xstrdup(<Expressions exprs>));`
		when id in ids
		
	case (Statement) `let <Binding_mode bm> <Identifier id> : MArray\<u8\> = Xstrdup(<Expressions exprs>);` => 
		 (Statement) `let <Binding_mode bm> <Identifier id> : MArray\<u8\> = MArray::from_raw(Xstrdup(<Expressions exprs>));`
		when id in ids
		
	case (Statement) `<Identifier id> = dir_name(<Expressions exprs>);` => 
		 (Statement) `<Identifier id> = MArray::from_raw(dir_name(<Expressions exprs>));`
		when id in ids
		
	case (Statement) `let <Binding_mode bm> <Identifier id> : MArray\<u8\> = dir_name(<Expressions exprs>);` => 
		 (Statement) `let <Binding_mode bm> <Identifier id> : MArray\<u8\> = MArray::from_raw(dir_name(<Expressions exprs>));`
		when id in ids
		
	case (Statement) `<Identifier id> = format_cmdline(<Expressions exprs>);` => 
		 (Statement) `<Identifier id> = MArray::from_raw(format_cmdline(<Expressions exprs>));`
		when id in ids
		
	case (Statement) `let <Binding_mode bm> <Identifier id> : MArray\<u8\> = format_cmdline(<Expressions exprs>);` => 
		 (Statement) `let <Binding_mode bm> <Identifier id> : MArray\<u8\> = MArray::from_raw(format_cmdline(<Expressions exprs>));`
		when id in ids
};

Expressions ptr_fn(Expressions exprs, set[Identifier] ids) = visit(exprs){
	case (Expression) `<Identifier id> as (*const u8)` =>
		 (Expression) `<Identifier id>.as_mut_ptr()`
		when id in ids
};
