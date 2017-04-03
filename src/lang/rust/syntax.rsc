@contributors{Adrian Zborowski - 11336544 - adrian.zborowski@uva.nl}

module lang::rust::\syntax

layout Whitespace = [\ \t\r\n]* !>> [\ \t\r\n];

lexical Identifier = [a-zA-Z][a-zA-Z0-9]*;

lexical Hash = "#";
lexical Shebang = Hash "!";

lexical Item 
= Statement_item 
| Item_macro;

lexical Statement_item 
= Item_static
| Item_const
| Item_type
| Block_item
| View_item;

lexical Item_static
= 'static' Identifier ':' Type '=' Expression ';'
| 'static' 'mut' Identifier ':' Type '=' Expression ';';

lexical Item_const
= 'const' Identifier ':' Type '=' Expression ';';

lexical Item_macro
= Path_expression '!' Maybe_identifier Parens_delimited_token_trees ';'
| Path_expression '!' Maybe_identifier Braces_delimited_token_trees
| Path_expression '!' Maybe_identifier Brackets_delimited_token_trees ';';

lexical View_item
= Use_item
| Extern_fn_item
| 'extern' 'crate' Identifier ';'
| 'extern' 'crate' Identifier 'as' Identifier ';';

lexical Extern_fn_item 
= 'extern' Maybe_abi Item_fn;

lexical Use_item
= 'use' View_path ';';

lexical View_path
= Path_no_types_allowed
| Path_no_types_allowed '::' '{' '}'
|                       '::' '{' '}'
| Path_no_types_allowed '::' '{' Identifiers_or_self '}'
|                       '::' '{' Identifiers_or_self '}'
| Path_no_types_allowed '::' '{' Identifiers_or_self ',' '}'
|                       '::' '{' Identifiers_or_self '}'
| Path_no_types_allowed '::' '*'
|                            '{' '}'
|                            '{' Identifiers_or_self '}'
|                            '{' Identifiers_or_self ',' '}'
| Path_no_types_allowed 'as' Identifier;

lexical Block_item
= Item_fn
| Item_unsafe_fn
| Item_mod
| Item_foreign_mod
| Item_struct
| Item_enum
| Item_trait
| Item_impl;

lexical Maybe_type_ascription
= ':' Type_sum
| /*empty*/;

lexical Maybe_init_expression
= '=' Expression
| /*empty*/;

lexical Item_struct
= 'struct' Identifier Generic_params Maybe_where_clause Struct_decl_args
| 'struct' Identifier Generic_params Struct_tuple_args Maybe_where_clause ';'
| 'struct' Identifier Generic_params Maybe_where_clause ';';

//struct_decl_args
//: '{' struct_decl_fields '}'                  { $$ = $2; }
//| '{' struct_decl_fields ',' '}'              { $$ = $2; }
//;

/* Types */
lexical Type 
= Primitive_type;

lexical Primitive_type
//: %prec IDENT path_generic_args_without_colons
//| %prec IDENT MOD_SEP path_generic_args_without_colons
//| %prec IDENT SELF MOD_SEP path_generic_args_without_colons
= 'Box' Type
| '*' Mut_Const Type
| '&' Type
| '&' 'mut' Type
| '&&' Type
| '&&' 'mut' Type
//| '&' lifetime maybe_mut ty
//| ANDAND lifetime maybe_mut ty
| '[' Type ']'
| '[' Type ',' '..' Expression']'
| '[' Type ';' Expression ']'
| 'typeof' '(' Expression ')'
| '_'
//| ty_bare_fn
//| ty_proc
//| for_in_type
;

lexical Type_fn_bare 
= 'fn';

lexical Lifetimes
= Lifetime_bounds
| Lifetimes ',' Lifetime_bounds;

lexical Lifetime_and_bounds
= '\''
| '\'static';

lexical Maybe_lifetime_bounds
//: %prec SHIFTPLUS
//  ':' ltbounds
= /*empty*/;

lexical Lifetime_bounds
= Lifetime
| Lifetime_bounds '+' Lifetime;

lexical Lifetime
= '\''
| '\'static';

lexical Mut_Const 
= 'mut'
| 'const'
| /*empty*/;
