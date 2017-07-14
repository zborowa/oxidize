module util::NonZero

import IO;
import Map;
import Set;
import ParseTree;
import lang::rust::\syntax::Rust;

alias Ids = set[Identifier];

start[Crate] nonzero(start[Crate] crate) = bottom-up visit(crate){
	case org:(Block_item) `unsafe extern <String? st> fn <Identifier fn_id> <Generic_params? gp> <Fn_decl params> <Where_clause? wc> { 
						  '<Inner_attribute* ia>
						  '<Statements stmts>
						  '}` =>
			 (Block_item) `unsafe extern <String? st> fn <Identifier fn_id> <Generic_params? gp> <Fn_decl params> <Where_clause? wc> {
						  '<Inner_attribute* ia>
						  '<Statements inc>
						  '}`
	when nci := null_check_id(stmts),
		 iis := id_in_scope(nci,stmts),
		 inc := is_null_checked(iis,stmts)
		 
};

Ids null_check_id(Statements stmts){
	Ids ids = {};
	
	visit(stmts){
		case (Statement) `if !<Identifier id>.is_null() <Block _>`:
			ids += id;
	};
	
	return ids;
}

Ids id_in_scope(Ids ids, Statements stmts){
	Ids sids = {};
	
	visit(stmts){
		case (Let) `let mut <Identifier id>: <Type_sum _>;`:{
			if(id in ids){
				ids += id;
			}
		}
			
		case (Let) `let mut <Identifier id>: <Type_sum _> = <Expression _>;`:{
			if(id in ids){
				ids += id;
			}
		}
	};
	
	return sids;
}

Statements is_null_checked(Ids ids, Statements stmts) = visit(stmts){
	case (Statement) `if !<Identifier id>.is_null() {
					 '<Statement* block_stmts>
					 '<Expression? block_expr>
					 '}` => 
		 (Statement) `if !<Identifier id>.is_null() {
		 			 'let <Identifier id> = NonZero::new(<Identifier id>);
		 			 '<Statement* block_stmts>
		 			 '<Expression? block_expr>
		 			 '}`
	when id in ids,
		 !let_nonzero_in_stmt(id,stmts)
};

// Pointer being safe if the code is compilable and thus this dereference being safe
//case (Statements) `let <Path_expression id> <Type_ascription? typea> = *<Path_expression pt>;` =>
//	 (Statements) `let <Path_expression id> <Type_ascription? typea> = *<Path_expression pt>;
//	 			  'let <Path_expression pt> <Type_ascription? typea> = NonZero::new(<Path_expression pt>);`
	
// Pointer being safe by construction
//case (Let) `let <Path_expression id> <Type_ascription? typea> = <Expression e> as <Type t>;` => 
//	 (Let) `let <Path_expression id> <Type_ascription? typea> = NonZero::new(<Expression e> as <Type t>);`
	
	bool let_nonzero_in_stmt(Identifier id, Statements stmts){
		visit(stmts){
			case (Statement) `let <Identifier id1> = NonZero::new(<Identifier id2>);`:{
				if(id1 == id2, id == id1){
					return true;
				}
			}
		}
		
		return false;
	}