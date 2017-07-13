module util::Raii

import IO;
import Map;
import Set;
import ParseTree;
import lang::rust::\syntax::Rust;

alias Ids = set[Identifier];

//Tree raii(Tree crate) = visit(crate){
start[Crate] raii(start[Crate] crate) = bottom-up visit(crate){
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
						  '<Statements otc>
						  '}`
	when fi := find_Identifiers(stmts),
		 fdi := fi.def,
		 fii := fi.ini,
		 aid := fdi + fii,
		 fvf := find_variable_free(aid,stmts),
		 df  := delete_free(fvf,stmts),
		 fdi := fdi & fvf,
		 fii := fii & fvf,
		 mt  := modify_type(fvf,df),
		 mdi := marray_definition_identifiers(fdi,mt),
		 mii := marray_initialization_identifiers(fii,mt),
		 mid := mdi + mii,
		 vtn := void_to_none(mid, mt),
		 vac := value_assignment_correction(mid, vtn),
		 vpc := value_passing_correction(mid,vac),
		 vuc := value_uasage_correction(mid,vpc),
		 otc := option_type_correction(vuc)
			 
};

/* ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- */

tuple[Ids def,Ids ini] find_Identifiers(Statements stmts){
	Ids def = {};
	Ids ini = {};
	
	visit(stmts){
		case (Let) `let mut <Identifier id> : <Type_sum ts>;`:{
			if(/(Type_sum) `*mut u8` := ts){
				def += id;
			}else{
				if(id in def){
					def = def - {id};
				}
				if(id in ini){
					ini = ini - {id};
				}
			}
		}
		
		case (Let) `let mut <Identifier id> : <Type_sum ts> = <Expression _>;`:{
			if(/(Type_sum) `*mut u8` := ts){
				ini += id;
			}else{
				if(id in def){
					def = def - {id};
				}
				if(id in ini){
					ini = ini - {id};
				}
			}
		}
	}
	
	return <def, ini>;
}

Ids find_variable_free(Ids ids, Statements stmts){
	Ids fids = {};
	
	visit(stmts){
		case (Statement) `free(<Identifier id> as (*mut ::std::os::raw::c_void));`:{
			if(id in ids){
				fids += id;
			}
		}
	}
	
	return fids;
}

Statements delete_free(Ids ids, Statements stmts) = visit(stmts){
	case (Statement) `free(<Identifier id> as (*mut ::std::os::raw::c_void));` =>
		 (Statement) `{}`
	when id in ids
};

Statements modify_type(Ids ids, Statements stmts) = visit(stmts){
	case (Statement) `let mut <Identifier id> : *mut u8;` => 
		 (Statement) `let mut <Identifier id> : MArray\<u8\>;`
	when id in ids
	
	case (Statement) `let mut <Identifier id> : *mut u8 = <Expression expr>;` => 
		 (Statement) `let mut <Identifier id> : MArray\<u8\> = <Expression expr>;`
	when id in ids
};

Ids marray_definition_identifiers(Ids ids, Statements stmts){
	Ids mids = {};

	visit(stmts){
		case (Statement) `let mut <Identifier id> : MArray\<u8\>;`:{
			if(id in ids){
				mids += id;
			}			
		}
	};
	
	return mids;
}

Ids marray_initialization_identifiers(Ids ids, Statements stmts){
	Ids mids = {};

	visit(stmts){
		case (Statement) `let mut <Identifier id> : MArray\<u8\> = <Expression _>;`:{
			if(id in ids){
				mids += id;
			}			
		}
	};
	
	return mids;
}

Statements void_to_none(Ids ids, Statements stmts) = visit(stmts){
	case (Statement) `let mut <Identifier id> : MArray\<u8\> = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);` => 
		 (Statement) `let mut <Identifier id> : MArray\<u8\> = None;`
	when id in ids

	case (Statement) `<Identifier id> = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);` => 
		 (Statement) `<Identifier id> = None;`
	when id in ids
	
	case (Expression) `<Identifier id> == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)` => 
		 (Expression) `<Identifier id> == None`
	when id in ids
	
	case (Expression) `<Identifier id> != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)` => 
		 (Expression) `<Identifier id> != None`
	when id in ids
};

Statements value_assignment_correction(Ids ids, Statements stmts) = visit(stmts){
	case (Statement) `<Identifier id> = <Expression expr>;` => 
		 (Statement) `<Identifier id> = MArray::from_raw(<Expression expr>);`
	when id in ids,
		 /(Expression) `None` !:= expr
		 
	case (Statement) `let mut <Identifier id> : MArray\<u8\> = <Expression expr>;` => 
		 (Statement) `let mut <Identifier id> : MArray\<u8\> = MArray::from_raw(<Expression expr>);`
	when id in ids,
		 /(Expression) `None` !:= expr
};

Statements value_passing_correction(Ids ids, Statements stmts) = top-down-break visit(stmts){
	case (Expression) `<Expression l_expr>(<Expressions exprs>)` => 
		 (Expression) `<Expression l_expr>(<Expressions mod_exprs>)`
	when mod_exprs := identifier_to_mut_ptr(ids,exprs)
	
	case (Statement) `<Identifier l_id> = <Identifier r_id> as (*const u8);` => 
		 (Statement) `<Identifier l_id> = <Identifier r_id>.as_mut_ptr() as (*const u8);`
	when r_id in ids
	
	case (Statement) `<Identifier l_id> = <Identifier r_id>;` => 
		 (Statement) `<Identifier l_id> = <Identifier r_id>.as_mut_ptr();`
	when r_id in ids
	
	case (Statement) `let <Identifier l_id> = <Identifier r_id>;` => 
		 (Statement) `let <Identifier l_id> = <Identifier r_id>.as_mut_ptr();`
	when r_id in ids
	
	case (Statement) `let <Identifier l_id> = *<Identifier r_id>;` => 
		 (Statement) `let <Identifier l_id> = *<Identifier r_id>.as_mut_ptr();`
	when r_id in ids
};

	Expressions identifier_to_mut_ptr(Ids ids, Expressions exprs) = visit(exprs){
		case (Expression) `<Identifier id>` => 
			 (Expression) `<Identifier id>.as_mut_ptr()`
		when id in ids
	};

Statements value_uasage_correction(Ids ids, Statements stmts) = top-down-break visit(stmts){
	case (Expression) `<Expression l_expr>(<Expressions exprs>)` => 
		 (Expression) `<Expression mod_expr>(<Expressions exprs>)`
	when mod_expr := identifier_to_mut_ptr(ids,l_expr)
	
	case (Expression) `<Identifier id>.is_null()` => 
		 (Expression) `<Identifier id>.as_mut_ptr().is_null()`
	when id in ids
	
	case (Expression) `!<Identifier id>.is_null()` => 
		 (Expression) `!<Identifier id>.as_mut_ptr().is_null()`
	when id in ids
};

	Expression identifier_to_mut_ptr(Ids ids, Expression expr) = visit(expr){
		case (Expression) `<Identifier id>` => 
			 (Expression) `<Identifier id>.as_mut_ptr()`
		when id in ids
	};
	
Statements option_type_correction(Statements stmts){
	Ids ids = {};

	stmts = visit(stmts){
		case (Statement) `let mut <Identifier id> : MArray\<u8\> = None;`:{
			ids += id;
			insert (Statement) `let mut <Identifier id> : Option\<MArray\<u8\>\> = None;`;
		}
	};
	
	stmts = visit(stmts){
		case (Statements) `let mut <Identifier id> : MArray\<u8\> = <Expression expr>;
						  '<Statement* stmts>`:{
			if(is_assigned_none(id,stmts) || is_compared_none(id,stmts)){
				ids += id;
				insert (Statements) `let mut <Identifier id> : Option\<MArray\<u8\>\> = <Expression expr>;
									'<Statement* stmts>`;
			}
		}
	};
	
	stmts = innermost visit(stmts){
		case (Statements) `let mut <Identifier id> : MArray\<u8\>;
						  '<Statement* stmts>`:{
			if(is_assigned_none(id,stmts) || is_compared_none(id,stmts)){
				ids += id;
				insert (Statements) `let mut <Identifier id> : Option\<MArray\<u8\>\>;
						  			'<Statement* stmts>`;
			}
		}
	};
	
	stmts = innermost visit(stmts){
		case (Statements) `<Statement* pre_stmts>
						  'let mut <Identifier id> : MArray\<u8\>;
						  '<Statement* stmts>`:{
			if(is_assigned_none(id,stmts) || is_compared_none(id,stmts)){
				ids += id;
				insert (Statements) `<Statement* pre_stmts>
									'let mut <Identifier id> : Option\<MArray\<u8\>\>;
						  			'<Statement* stmts>`;
			}
		}
	};
	
	stmts = visit(stmts){
		case (Statement) `let mut <Identifier id> : MArray\<u8\>;`:{
			if(is_assigned_none(id,stmts) || is_compared_none(id,stmts)){
				ids += id;
				insert (Statement) `let mut <Identifier id> : Option\<MArray\<u8\>\>;`;
			}
		}
	};
	
	stmts = visit(stmts){
		case (Statement) `<Identifier l_id> = <Expression expr>;` => 
			 (Statement) `<Identifier l_id> = Some(<Expression expr>);`
		when l_id in ids,
			 /(Expression) `None` !:= expr,
			 /(Expression) `Some(<Expression _>)` !:= expr
			 
		case (Statement) `let mut <Identifier l_id> : <Type_sum ts> = <Expression expr>;` => 
			 (Statement) `let mut <Identifier l_id> : <Type_sum ts> = Some(<Expression expr>);`
		when l_id in ids,
			 /(Expression) `None` !:= expr,
			 /(Expression) `Some(<Expression _>)` !:= expr
	};
	
	stmts = visit(stmts){
		case (Expression) `<Identifier id>.as_mut_ptr()` => 
			 (Expression) `<Identifier id>.unwrap().as_mut_ptr()`
		when id in ids
	};

	return stmts;
}

	bool is_assigned_none(Identifier id, Statement* stmts) 
		=  /(Statement) `<Identifier l_id> = None;` := stmts
		&& l_id == id;
		
	bool is_compared_none(Identifier id, Statement* stmts)
		=  /(Expression) `<Identifier l_id1> == None` := stmts
		&& l_id1 == id
		|| /(Expression) `<Identifier l_id2> != None` := stmts
		&& l_id2 == id;
		
	bool is_assigned_none(Identifier id, Statements stmts) 
		=  /(Statement) `<Identifier l_id> = None;` := stmts
		&& l_id == id;
		
	bool is_compared_none(Identifier id, Statements stmts)
		=  /(Expression) `<Identifier l_id1> == None` := stmts
		&& l_id1 == id
		|| /(Expression) `<Identifier l_id2> != None` := stmts
		&& l_id2 == id;
