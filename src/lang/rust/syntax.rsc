@contributors{Adrian Zborowski - 11336544 - adrian.zborowski@uva.nl}

module lang::rust::\syntax

import Prelude;
import vis::ParseTree;

lexical Ident = [a-zA-Z_][a-zA-Z0-9_]* !>> [a-zA-Z0-9_];

layout Whitespace = [\ \t\r\n]* !>> [\ \t\r\n];
lexical Inner_doc_comment = "///[^\n]*\n" | "/**[^*][^*]**/";
lexical Linecomment = "//[^\n]*\n" | "////[^\n]*\n";
lexical Outer_doc_comment = "//![^\n]*\n" | "/*![^*][^*]*/";
lexical Blockcomment = "/**/";

lexical Literal_byte
= "b\'\\[nrt\\\'\\u0220]\'" 
| "b\'\\u00[0-9a-fA-F]{2}\'" 
| "b\'\\u[0-9a-fA-F]{4}\'" 
| "b\'\\U[0-9a-fA-F]{8}\'" 
| "b\'.\'";

lexical Literal_char 
= "\'\\[nrt\\\'\u0220]\'" 
| "\'\\u00[0-9a-fA-F]{2}\'" 
| "\'.\'" 
| "\'[\\u0080-\\u00ff]{2,4}\'";

lexical Literal_integer 
= "0x[0-9a-fA-F_]+" 
| "0o[0-8_]+" 
| "0b[01_]+" 
| "[0-9][0-9_]*" 
| "[0-9][0-9_]*.(.|[a-zA-Z])";

lexical Literal_float 
= "[0-9][0-9_]*.[0-9_]*([eE][-+]?[0-9_]+)?" 
| "[0-9][0-9_]*(.[0-9_]*)?[eE][-+]?[0-9_]+";

lexical Literal_string
= "\"[.\n]*\"";

lexical Literal_string_raw 
= "r\"[.\n]*\"";

lexical Literal_byte_string 
= "b\"[.\n]*\"";

lexical Literal_byte_string_raw 
= "br\"[].\n*\"";

lexical Hash = '#';
lexical Shebang = Hash '!' '[';
lexical Shebang_line = Hash '!' '^[\n]*\n';

/* #### #### Items and attributes #### #### */

syntax Crate
= Maybe_shebang Inner_attributes Maybe_mod_items
| Maybe_shebang Maybe_mod_items;

syntax Maybe_shebang
= Shebang_line
| /*empty*/;

syntax Maybe_inner_attributes
= Inner_attributes
| /*empty*/;

syntax Inner_attributes
= Inner_attribute
| Inner_attributes Inner_attribute;

syntax Inner_attribute
= Shebang '[' Meta_item ']'
| Inner_doc_comment;

syntax Maybe_outer_attributes
= Outer_attributes
| /*empty*/;

syntax Outer_attributes
= Outer_attributes
| Outer_attributes Outer_attribute;

syntax Outer_attribute
= '#' '[' Meta_item ']'
| Outer_doc_comment;

syntax Meta_item
= Identifier
| Identifier '=' Literal
| Identifier '(' Meta_sequence ')'
| Identifier '(' Meta_sequence ',' ')';

syntax Meta_sequence 
= /*empty*/
| Meta_item
| Meta_sequence ',' Meta_item;

syntax Maybe_mod_items
= Mod_items
| /*empty*/;

syntax Mod_items
= Mod_item
| Mod_items Mod_item;

syntax Attributes_and_vis
= Maybe_outer_attributes Visibility;

syntax Mod_item
= Attributes_and_vis Item;

syntax Item 
= Statement_item 
| Item_macro;

syntax Statement_item 
= Item_static
| Item_const
| Item_type
| Block_item
| View_item;

syntax Item_static
= 'static' Identifier ':' Type '=' Expression ';'
| 'static' 'mut' Identifier ':' Type '=' Expression ';';

syntax Item_const
= 'const' Identifier ':' Type '=' Expression ';';

syntax Item_macro
= Path_expression '!' Maybe_identifier Parens_delimited_token_trees ';'
| Path_expression '!' Maybe_identifier Braces_delimited_token_trees
| Path_expression '!' Maybe_identifier Brackets_delimited_token_trees ';';

syntax View_item
= Use_item
| Extern_fn_item
| 'extern' 'crate' Identifier ';'
| 'extern' 'crate' Identifier 'as' Identifier ';';

syntax Extern_fn_item 
= 'extern' Maybe_abi Item_fn;

syntax Use_item
= 'use' View_path ';';

syntax View_path
= Path_no_types_allowed
| Path_no_types_allowed '::' '{' '}'
| '::' '{' '}'
| Path_no_types_allowed '::' '{' Identifiers_or_self '}'
| '::' '{' Identifiers_or_self '}'
| Path_no_types_allowed '::' '{' Identifiers_or_self ',' '}'
| '::' '{' Identifiers_or_self '}'
| Path_no_types_allowed '::' '*'
| '{' '}'
| '{' Identifiers_or_self '}'
| '{' Identifiers_or_self ',' '}'
| Path_no_types_allowed 'as' Identifier;

syntax Block_item
= Item_fn
| Item_unsafe_fn
| Item_mod
| Item_foreign_mod
| Item_struct
| Item_enum
| Item_trait
| Item_impl;

syntax Maybe_type_ascription
= ':' Type_sum
| /*empty*/;

syntax Maybe_init_expression
= '=' Expression
| /*empty*/;

syntax Item_struct
= 'struct' Identifier Generic_params Maybe_where_clause Struct_decl_args
| 'struct' Identifier Generic_params Struct_tuple_args Maybe_where_clause ';'
| 'struct' Identifier Generic_params Maybe_where_clause ';';

syntax Structure_declaration_args
= '{' Structure_declaration_fields '}'
| '{' Structure_declaration_fields ',' '}';

syntax Structure_typle_args
= '(' Structure_tuple_fields ')'
| '(' Structure_tuple_fields ',' ')';

syntax Structure_declaration_fields
= Structure_declaration_field
| Structure_declaration_fields ',' Structure_declaration_field
| /*empty*/;

syntax Structure_decl_field
= Attributes_and_vis Identifier ':' Type_sum;

syntax Structure_tuple_fields
= Structure_tuple_field
| Structure_tuple_fields ',' Structure_tuple_field;

syntax Structure_tuple_field
= Attributes_and_vis Type_sum;

syntax Item_enum
= 'enum' Identifier Generic_params Maybe_where_clause '{' Enum_defs '}'
| 'enum' Identifier Generic_params Maybe_where_clause '{' Enum_defs ',' '}';

syntax Enum_defs 
= Enum_def
| Enum_defs ',' Enum_def
| /*empty*/;

syntax Enum_def
= Attributes_and_vis Identifier Enum_args;

syntax Enum_args
= '{' Structure_decl_fields '}'
| '{' Structure_decl_fields ',' '}'
| '{' Maybe_type_sum '}'
| '=' Expression
| /*empty*/;

syntax Item_mod
= 'mod' Identifier ';'
| 'mod' Identifier '{' Maybe_mod_items '}'
| 'mod' Identifier '{' Inner_attrs Maybe_mod_items '}';

syntax Item_foreign_mod
= 'extern' Maybe_abi '{' Maybe_foreign_items '}'
| 'extern' Maybe_abi '{' Inner_attrs Maybe_foreign_items '}';

syntax Maybe_abi
= String
| /*empty*/;

syntax Maybe_foreign_items
= Foreign_items
| /*empty*/;

syntax Foreign_items
= Foreign_item
| Foreign_items Foreign_item;

syntax Foreign_item
= Attributes_and_vis 'static' Item_foreign_static
| Attributes_and_vis Item_foreign_fn
| Attributes_and_vis 'unsafe' Item_foreign_fn;

syntax Item_foreign_static
= Maybe_mut Identifier ':' Types ';';

syntax Item_foreign_fn
= 'fn' Identifier Generic_params Fn_declaration_allow_variadic Maybe_where_clause ';';

syntax Fn_params_allow_variadic
= '(' ')'
| '(' Params ')'
| '(' Params ',' ')'
| '(' Params ',' '...' ')';

syntax Visibility
= 'pub'
| /*empty*/;

syntax Identifiers_or_self
= Identifier_or_self
| Identifier_or_self 'as' Identifier
| Identifiers_or_self ',' Identifier_or_self;

syntax Identifier_or_self
= Identifier
| 'self';

syntax Item_type
= 'type' Identifier Generic_params Maybe_where_clause '=' Type_sum ';';

syntax For_sized 
= 'for' '?' Identifier
| 'for' Identifier '?'
| /*empty*/;

syntax Item_trait
= Maybe_unsafe 'trait' Identifier Generic_params For_sized Maybe_type_param_bounds Maybe_where_clause \
  '{' Maybe_trait_items '}';
  
syntax Maybe_trait_items
= Trait_items
| /*empty*/;

syntax Trait_items
= Trait_item
| Trait_items Trait_item;

syntax Trait_item
= Trait_const
| Trait_type
| Trait_method;

syntax Trait_const
= Maybe_outer_attributes 'const' Identifier Maybe_type_ascription Maybe_const_defailt ';';

syntax Maybe_const_default
= '=' Expression
| /*empty*/;

syntax Trait_type
= Maybe_outer_attributes 'type' Type_param ';';

syntax Maybe_unsafe
= 'unsafe'
| /*empty*/;

syntax Trait_method
= Type_method
| Method;

syntax Type_method
= Attributes_and_vis Maybe_unsafe 'fn' Identifier Generic_params \
  Fn_decl_with_self_allow_anon_params Maybe_where_clause ';'
| Attributes_and_vis Maybe_unsafe 'extern' Maybe_abi 'fn' Identifier Generic_params \
  Fn_decl_with_self_allow_anon_params Maybe_where_clause ';';
  
syntax Method
= Attributes_and_vis Maybe_unsafe 'fn' Identifier Generic_params \
  Fn_decl_with_self_allow_anon_params Maybe_where_claus Inner_attributes_and_block
| Attributes_and_vis Maybe_unsafe 'extern' Maybe_abi 'fn' Identifier Generic_params \
  Fn_decl_with_self_allow_anon_params Maybe_where_claus Inner_attributes_and_block;

syntax Impl_method
= Attributes_and_vis Maybe_unsafe 'fn' Identifier Generic_params \
  Fn_decl_with_self Maybe_where_clause Inner_attributes_and_block
| Attributes_and_vis Maybe_unsafe 'extern' Maybe_abi 'fn' Identifier Generic_params \
  Fn_decl_with_self Maybe_where_clause Inner_attributes_and_block;
  
syntax Item_impl
= Maybe_unsafe 'impl' Generic_params Type_primitive_sum Maybe_where_clause '{' Maybe_inner_attributes Maybe_impl_items '}'
| Maybe_unsafe 'impl' Generic_params '(' Type ')' Maybe_where_clause '{' Maybe_inner_attributes Maybe_impl_items '}'
| Maybe_unsafe 'impl' Generic_params Trait_ref 'for' Type_sum Maybe_where_clause '{' Maybe_inner_attributes Maybe_impl_items '}'
| Maybe_unsafe 'impl' Generic_params '!' Trait_ref 'for' Type_sum Maybe_where_clause '{' Maybe_inner_attributes Maybe_impl_items '}'
| Maybe_unsafe 'impl' Generic_params Trait_ref 'for' '..' '{' '}'
| Maybe_unsafe 'impl' Generic_params '!' Trait_ref 'for' '..' '{' '}';

syntax Maybe_impl_items
= Impl_items
| /*empty*/;

syntax Impl_items
= Impl_item
| Impl_item Impl_items;

syntax Impl_item
= Impl_method
| Attributes_and_vis Item_macro
| Impl_const
| Impl_type;

syntax Impl_const
= Attributes_and_vis Item_const;

syntax Impl_type
= Attributes_and_vis 'type' Identifier Generic_params '=' Type_sum ';';

syntax Item_fn
= 'fn' Identifier Generic_params Fn_decl Maybe_where_clause Inner_attributes_and_block;

syntax Item_unsafe_fn
= 'unsafe' 'fn' Identifier Generic_params Fn_decl Maybe_where_clause Inner_attributes_and_block
| 'unsafe' 'extern' Maybe_abi 'fn' Identifier Generic_params Fn_decl Maybe_where_clause Inner_attributes_and_block;

syntax Fn_decl
= Fn_params Ret_type;

syntax Fn_decl_with_self
= Fn_params_with_self Ret_type;

syntax Fn_decl_with_self_allow_anon_params
= Fn_anon_params_with_self Ret_type;

syntax Fn_params
= '(' Maybe_params ')';

syntax Fn_anon_params
= '(' Anon_param Anon_params_allow_variadic_tail ')'
| '('  ')';

syntax Fn_params_with_self
= '(' Maybe_mut 'self' Maybe_type_ascription Maybe_comma_params ')'
| '(' '&' Maybe_mut 'self' Maybe_type_ascription Maybe_comma_params ')'
| '(' '&'  Lifetime Maybe_mut 'self' Maybe_type_ascription Maybe_comma_params ')'
| '(' Maybe_params ')';

syntax Fn_anon_params_with_self
= '(' Maybe_mut 'self' Maybe_type_ascription Maybe_comma_anon_params ')'
| '(' '&' Maybe_mut 'self' Maybe_type_ascription Maybe_comma_anon_params ')'
| '(' '&'  Lifetime Maybe_mut 'self' Maybe_type_ascription Maybe_comma_anon_params ')'
| '(' Maybe_anon_params ')';

syntax Maybe_params
= Params
| Params ','
| /*empty*/;

syntax Params
= Param
| Params ',' Param;

syntax Param
= Pattern ':' Type_sum;

syntax Inferrable_params
= Inferrable_param
| Inferrable_params ',' Inferrable_param;

syntax Inferrable_param
= Pattern Maybe_type_ascription;

syntax Maybe_unboxed_closure_kind
= /*empty*/
| ':'
| '&' Maybe_mut ':';

syntax Maybe_comma_params
= ','
| ',' Params
| ',' Params ','
| /*empty*/;

syntax Maybe_comma_anon_params
= ','
| ',' Anon_params
| ',' Anon_params ','
| /*empty*/;

syntax Maybe_anon_params
= Anon_params
| Anon_params ','
| /*empty*/;

syntax Anon_params
= Anon_param
| Anon_params ',' Anon_param;

syntax Anon_param
= Named_arg ':' Type
| Type;

syntax Anon_params_allow_variadic_tail
= ',' '...'
| ',' Anon_param Anon_params_allow_variadic_tail;

syntax Named_arg
= Identifier
| '_'
| '&' Identifier
| '&' '_'
| '&&' Identifier
| '&&' '_'
| 'mut' Identifier;

syntax Ret_type
= '-\>' '!'
| '-\>' Type
| Identifier /*empty*/
;

syntax Generic_params
= '\<' Lifetime '\>'
| '\<' Lifetime ',' '\>'
| '\<' Lifetime '\>\>'
| '\<' Lifetime ',' '\>\>'
| '\<' Lifetime ',' Type_params '\>'
| '\<' Lifetime ',' Type_params ',' '\>'
| '\<' Lifetime ',' Type_params '\>\>'
| '\<' Lifetime ',' Type_params ',' '\>\>'
| '\<' Type_params '\>'
| '\<' Type_params ',' '\>'
| '\<' Type_params '\>\>'
| '\<' Type_params ',' '\>\>'
| /*empty*/;

syntax Maybe_where_clause
= /*empty*/
| Where_clause;

syntax Where_clause
= 'where' Where_predicates
| 'where' Where_predicates ',';

syntax Where_predicates
= Where_predicate
| Where_predicates ',' Where_predicate;

syntax Where_predicate
= Maybe_for_lifetimes LifeTime ':' Bounds
| Maybe_for_lifetimes Type ':' Type_param_bounds;

syntax Maybe_for_lifetimes
= 'for' '\<' LifeTime '\>'
| 'for' /*empty*/
;

syntax Type_params
= Type_param
| Type_params ',' Type_param;

syntax Path_no_types_allowed
= Identifier
| '::' Identifier
| 'self'
| '::' 'self'
| Path_no_types_allowed '::' Identifier;

syntax Path_generic_args_without_colons
= Identifier
| Identifier Generic_args
| Identifier '(' Maybe_type_sums ')' Ret_type
| Path_generic_args_without_colons '::' Identifier
| Path_generic_args_without_colons '::' Identifier Generic_args
| Path_generic_args_without_colons '::' Identifier '(' Maybe_type_sums ')' Ret_type
;

syntax Generic_args
= '\<' Generic_values '\>'
| '\<' Generic_values '\>\>'
| '\<' Generic_values '\>='
| '\<' Generic_values '\>\>='
| '\<\<' Type_qualified_path_and_generic_values '\>'
| '\<\<' Type_qualified_path_and_generic_values '\>\>'
| '\<\<' Type_qualified_path_and_generic_values '\>='
| '\<\<' Type_qualified_path_and_generic_values '\>\>=';

syntax Generic_values
= Maybe_lifetimes Maybe_type_sums_and_or_bindings;

syntax Maybe_type_sums_and_or_bindings
= Type_sums
| Type_sums ','
| Type_sums ',' Bindings
| Bindings
| Bindings ','
| /*empty*/;

syntax Bindings
= ',' Bindings
| /*empty*/;

/* #### #### Patterns #### #### */

syntax Pattern
= '_'
| '&' Pattern
| '&' 'mut' Pattern
| '&&' Pattern
| '(' ')'
| '(' Pattern_tuple ')'
| '(' Pattern_tuple ',' ')'
| '[' Pattern_vector ']'
| Literal_or_path
| Literal_or_path '...' Literal_or_path
| Path_expression '{' Pattern_structure '}'
| Path_expression '(' '..' ')'
| Path_expression '(' Pattern_tuple ')'
| Path_expression '!' Maybe_identifier Delimited_token_trees
| Binding_mode Identifier
| Identifier '@' Pattern
| Binding_mode Identifier '@' Pattern
| 'box' Pattern
| '\<' Type_sum Maybe_as_trait_ref '\>' '::' Identifier
| '\<\<' Type_sum Maybe_as_trait_ref '\>' '::' Identifier Maybe_as_trait_ref '\>' '::' Identifier;

syntax Patterns_or
= Pattern
| Patterns_or '|' Pattern;

syntax Binding_mode
= 'ref'
| 'ref' 'mut'
| 'mut';

syntax Literal_or_path
= Path_expression
| Literal
| '-' Literal;

syntax Pattern_field
= Identifier
| Binding_mode Identifier
| 'box' Identifier
| 'box' Binding_mode Identifier
| Identifier ':' Pattern
| Binding_mode Identifier ':' Pattern;

syntax Pattern_fields
= Pattern_field
| Pattern_fields ',' Pattern_field;

syntax Pattern_structure
= Patterns_fields
| Pattern_fields ','
| Pattern_fields ',' '..'
| '..';

syntax Pattern_tuple
= Pattern
| Patterns_tuple ',' Pattern;

syntax Pattern_vector
= Pattern_vector_elts
| Pattern_vector_elts ','
| Pattern_vector_elts '..'
| Pattern_vector_elts ',' '..'
| Pattern_vector_elts '..' ',' Pattern_vector_elts
| Pattern_vector_elts '..' ',' Pattern_vector_elts ','
| Pattern_vector_elts ',' '..' ',' Pattern_vector_elts
| Pattern_vector_elts ',' '..' ',' Pattern_vector_elts ','
| '..' ',' Pattern_vector_elts
| '..' ',' Pattern_vector_elts ','
| '..'
| /*empty*/;

syntax Pattern_vector_elts
= Pattern
| Pattern_vector_elts ',' Pattern;

/* #### #### Types #### #### */

syntax Type 
= Type_primitive
| Type_closure
| '\<' Type_sum Maybe_as_trait_ref '\>' '::' Identifier
| '\<\<' Type_sum Maybe_as_trait_ref '\>\>' '::' Identifier Maybe_as_trait_ref '\>' '::' Identifier
| '(' Type_sums ')'
| '(' Type_sums ',' ')'
| '(' ')';

syntax Type_primitive
= Path_generic_args_without_colons
| '::' Path_Generic_args_without_colons
| 'self' '::' Path_generic_args_without_colons
|'Box' Type
| '*' Mut_Const Type
| '&' Type
| '&' 'mut' Type
| '&&' Type
| '&&' 'mut' Type
| '&' Lifetime Maybe_mut Type
| '&&' Lifetime Maybe_mut Type
| '[' Type ']'
| '[' Type ',' '..' Expression']'
| '[' Type ';' Expression ']'
| 'typeof' '(' Expression ')'
| '_'
| Type_bare_fn
| Type_proc
| For_in_type
;

syntax Type_bare_fn 
= 'fn' Type_fn_decl
| 'unsafe' 'fn' Type_fn_decl
| 'extern' Maybe_abi 'fn' Type_fn_decl
| 'unsafe' 'extern' Maybe_abi 'fn' Type_fn_decl;

syntax Type_fn_decl
= Generic_params Fn_anon_params Ret_type;

syntax Type_closure
= 'unsafe' '|' Anon_params '|' Maybe_bounds Ret_type
| '|' Anon_params '|' Maybe_bounds Ret_type
| 'unsafe' '||' Maybe_bounds Ret_type
| '||' Maybe_bounds Ret_type;

syntax Type_proc 
= 'proc' Generic_params Fn_params Maybe_bounds Ret_type;

syntax For_in_type
= 'for' '\<' Maybe_lifetime '\>' For_in_type_suffix;

syntax For_in_type_suffix
= Type_proc
| Type_bare_fn
| Trait_ref
| Type_closure;

syntax Maybe_mut
= 'mut'
| 'mut' /*empty*/;

syntax Maybe_mut_or_const
= 'mut'
| 'const' 
| /*empty*/;

syntax Type_qualified_path_and_generic_values
= Type_qualified_path Maybe_bindings
| Type_qualified_path ',' Type_sums Maybe_bindings;

syntax Type_qualified_path
= Type_sum 'as' Trait_ref '\>' '::' Identifier
| Type_sum 'as' Trait_ref '\>' '::' Identifier '+' Type_param_bounds;

syntax Maybe_type_sums
= Type_sums
| Type_sums ','
| /*empty*/;

syntax Type_sums
= Type_sum
| Type_sums ',' Type_sum;

syntax Type_sum
= Type
| Type '+' Type_param_bounds;

syntax Type_primitive_sum
= Type_primitive
| Type_primitive '+' Type_param_bounds;

syntax Maybe_type_param_bounds
= ':' Type_param_bounds
| /*empty*/;

syntax Type_param_bounds
= Bounds_sequence
| /*empty*/;

syntax Bounds_sequence
= Poly_bound
| Bound_sequence '+' Poly_bound;

syntax Poly_bound
= 'for' '\<' Maybe_lifetime '\>' Bound
| Bound
| '?' Bound;

syntax Bindings
= Binding
| Bindings ',' Binding;

syntax Binding
= Identifier '=' Type;

syntax Type_param
= Identifier Maybe_type_param_bounds Maybe_type_default
| Identifier '?' Identifier Maybe_type_param_bounds Maybe_type_default;

syntax Maybe_bounds
= ':' Bounds
| /*empty*/;

syntax Bounds
= Bound
| Bounds '+' Bound;

syntax Bound
= Lifetime
| Trait_ref;

syntax Maybe_ltbounds
= ':' Ltbounds
| /*empty*/;

syntax Ltbounds
= Lifetime
| LtBounds '+' Lifetime;

syntax Maybe_type_default
= '=' Type_sum
| /*empty*/;

syntax Maybe_lifetimes
= Lifetimes
| Lifetimes ','
| /*empty*/;

syntax Lifetimes
= Lifetime_and_bounds
| Lifetimes ',' Lifetime_and_bounds;

syntax Lifetime_and_bounds
= '\'' Maybe_ltbounds
| '\'static';

syntax Lifetime
= '\''
| '\'static';

syntax Trait_ref
= Path_generic_args_without_colons
| '::' Path_generic_args_without_colons;

/* #### #### Blocks, Statements, and expressions #### #### */

syntax Inner_attributes_and_block
= '{' Maybe_inner_attributes Maybe_statements '}';

syntax Block
= '{' Maybe_statement '}';

syntax Maybe_statement
= Statements
| Statements Nonblock_expression
| Nonblock_expression
| /*empty*/;

syntax Statements
= Statement
| Statements Statement;

syntax Statement
= Let
| Statement_item
| 'pub' Statement_item
| Outer_attributes Statement_item
| Outer_attributes 'pub' Statement_item
| Full_block_expression
| Block
| Nonblock_expression ';'
| ';';

syntax Maybe_expressions
= Expressions
| Expressions ','
| /*empty*/;

syntax Maybe_expression
= Expression
| /*empty*/;

syntax Expressions
= Expression
| Expressions ',' Expression;

syntax Path_expression
= Path_generic_args_with_colons
| '::' Path_generic_args_with_colons
| 'self' '::' Path_generic_args_with_colons;

syntax Path_generic_args_with_colons
= Identifier
| Path_generic_args_with_colons '::' Identifier
| Path_generic_args_with_colons '::' Generic_args;

syntax Macro_expression
= Path_expression '!' Maybe_identifier Parens_delimited_token_trees
| Path_expression '!' Maybe_identifier Brackets_delimited_token_trees;

syntax Nonblock_expression
= Literal
| Path_expression
| 'self'
| Macro_expression
| Path_expression '{' Structure_expression_fields '}'
| Nonblock_expression '.' Path_generic_args_with_colons
| Nonblock_expression '.' Literal_integer
| Nonblock_expression '[' Maybe_expression ']'
| Nonblock_expression '(' Maybe_expression ')'
| '[' Vector_expression ']'
| '(' Maybe_expression ')'
| 'continue'
| 'continue' Lifetime
| 'return'
| 'return' Expression
| 'break'
| 'break' Lifetime
| Nonblock_expression '\<-' Expression
| Nonblock_expression '=' Expression
| Nonblock_expression '\<\<=' Expression
| Nonblock_expression '\>\>=' Expression
| Nonblock_expression '-=' Expression
| Nonblock_expression '&=' Expression
| Nonblock_expression '|=' Expression
| Nonblock_expression '+=' Expression
| Nonblock_expression '*=' Expression
| Nonblock_expression '/=' Expression
| Nonblock_expression '^=' Expression
| Nonblock_expression '%=' Expression
| Nonblock_expression '||' Expression
| Nonblock_expression '&&' Expression
| Nonblock_expression '==' Expression
| Nonblock_expression '!=' Expression
| Nonblock_expression '\<' Expression
| Nonblock_expression '\>' Expression
| Nonblock_expression '\<=' Expression
| Nonblock_expression '\>=' Expression
| Nonblock_expression '|' Expression
| Nonblock_expression '^' Expression
| Nonblock_expression '&' Expression
| Nonblock_expression '\<\<' Expression
| Nonblock_expression '\>\>' Expression
| Nonblock_expression '+' Expression
| Nonblock_expression '-' Expression
| Nonblock_expression '*' Expression
| Nonblock_expression '/' Expression
| Nonblock_expression '%' Expression
| Nonblock_expression '..'
| Nonblock_expression '..' Expression
| '..' Expression
| '..'
| Nonblock_expression 'as' Type
| 'box' Nonparen_expression
| 'box' '(' Maybe_expression ')' Nonblock_expression
| Expression_qualified_path
| Nonblock_prefix_expression;

syntax Expression
= Literal
| Path_expression
| 'self'
| Macro_expression
| Path_expression '{' Structure_expression_fields '}'
| Expression '.' Path_generic_args_with_colons
| Expression '.' Literal_integer
| Expression '[' Maybe_expression ']'
| Expression '(' Maybe_expressions ')'
| '(' Maybe_expressions ')'
| '[' Vector_expression ']'
| 'continue'
| 'continue' Identifier
| 'return'
| 'return' Expression
| 'break'
| 'break' Identifier
| Expression '\<-' Expression
| Expression '=' Expression
| Expression '\<\<=' Expression
| Expression '\>\>=' Expression
| Expression '-=' Expression
| Expression '&=' Expression
| Expression '|=' Expression
| Expression '+=' Expression
| Expression '*=' Expression
| Expression '/=' Expression
| Expression '^=' Expression
| Expression '%=' Expression
| Expression '||' Expression
| Expression '&&' Expression
| Expression '==' Expression
| Expression '!=' Expression
| Expression '\<' Expression
| Expression '\>' Expression
| Expression '\<=' Expression
| Expression '\>=' Expression
| Expression '|' Expression
| Expression '^' Expression
| Expression '&' Expression
| Expression '\<\<' Expression
| Expression '\>\>' Expression
| Expression '+' Expression
| Expression '-' Expression
| Expression '*' Expression
| Expression '/' Expression
| Expression '%' Expression
| Expression '..'
| Expression '..' Expression
| '..' Expression
| '..'
| Expression 'as' Type
| 'box' Nonparen_expression
| 'box' '(' Maybe_expression ')' Expression
| Expression_qualified_path
| Block_expression
| Block
| Nonblock_prefix_expression;

syntax Nonparen_expression
= Literal
| Path_expression
| 'self'
| Macro_expression
| Path_expression '{' Structure_expression_fields '}'
| Nonparen_expression '.' Path_generic_args_with_colons
| Nonparen_expression '.' Literal_integer
| Nonparen_expression '[' Maybe_expression ']'
| Nonparen_expression '(' Maybe_expressions ')'
| '[' Vector_expression ']'
| 'continue'
| 'continue' Identifier
| 'return'
| 'return' Expression
| 'break'
| 'break' Identifier
| Nonparen_expression '\<-' Nonparen_expression
| Nonparen_expression '=' Nonparen_expression
| Nonparen_expression '\<\<=' Nonparen_expression
| Nonparen_expression '\>\>=' Nonparen_expression
| Nonparen_expression '-=' Nonparen_expression
| Nonparen_expression '&=' Nonparen_expression
| Nonparen_expression '|=' Nonparen_expression
| Nonparen_expression '+=' Nonparen_expression
| Nonparen_expression '*=' Nonparen_expression
| Nonparen_expression '/=' Nonparen_expression
| Nonparen_expression '^=' Nonparen_expression
| Nonparen_expression '%=' Nonparen_expression
| Nonparen_expression '||' Nonparen_expression
| Nonparen_expression '&&' Nonparen_expression
| Nonparen_expression '==' Nonparen_expression
| Nonparen_expression '!=' Nonparen_expression
| Nonparen_expression '\<' Nonparen_expression
| Nonparen_expression '\>' Nonparen_expression
| Nonparen_expression '\<=' Nonparen_expression
| Nonparen_expression '\>=' Nonparen_expression
| Nonparen_expression '|' Nonparen_expression
| Nonparen_expression '^' Nonparen_expression
| Nonparen_expression '&' Nonparen_expression
| Nonparen_expression '\<\<' Nonparen_expression
| Nonparen_expression '\>\>' Nonparen_expression
| Nonparen_expression '+' Nonparen_expression
| Nonparen_expression '-' Nonparen_expression
| Nonparen_expression '*' Nonparen_expression
| Nonparen_expression '/' Nonparen_expression
| Nonparen_expression '%' Nonparen_expression
| Nonparen_expression '..'
| Nonparen_expression '..' Nonparen_expression
| '..' Nonparen_expression
| '..'
| Nonparen_expression 'as' Type
| 'box' Nonparen_expression
| 'box' '(' Maybe_expression ')' Expression
| Expression_qualified_path
| Block_expression
| Block
| Nonblock_prefix_expression;

syntax Expression_nostruct
= Literal
| Path_expression
| 'self'
| Macro_expression
| Expression_nostruct '.' Path_generic_args_with_colons
| Expression_nostruct '.' Literal_integer
| Expression_nostruct '[' Maybe_expression ']'
| Expression_nostruct '(' Maybe_expressions ')'
| '[' Vector_expression ']'
| '(' Maybe_expression ')'
| 'continue'
| 'continue' Identifier
| 'return'
| 'return' Expression
| 'break'
| 'break' Identifier
| Expression_nostruct '\<-' Expression_nostruct
| Expression_nostruct '=' Expression_nostruct
| Expression_nostruct '\<\<=' Expression_nostruct
| Expression_nostruct '\>\>=' Expression_nostruct
| Expression_nostruct '-=' Expression_nostruct
| Expression_nostruct '&=' Expression_nostruct
| Expression_nostruct '|=' Expression_nostruct
| Expression_nostruct '+=' Expression_nostruct
| Expression_nostruct '*=' Expression_nostruct
| Expression_nostruct '/=' Expression_nostruct
| Expression_nostruct '^=' Expression_nostruct
| Expression_nostruct '%=' Expression_nostruct
| Expression_nostruct '||' Expression_nostruct
| Expression_nostruct '&&' Expression_nostruct
| Expression_nostruct '==' Expression_nostruct
| Expression_nostruct '!=' Expression_nostruct
| Expression_nostruct '\<' Expression_nostruct
| Expression_nostruct '\>' Expression_nostruct
| Expression_nostruct '\<=' Expression_nostruct
| Expression_nostruct '\>=' Expression_nostruct
| Expression_nostruct '|' Expression_nostruct
| Expression_nostruct '^' Expression_nostruct
| Expression_nostruct '&' Expression_nostruct
| Expression_nostruct '\<\<' Expression_nostruct
| Expression_nostruct '\>\>' Expression_nostruct
| Expression_nostruct '+' Expression_nostruct
| Expression_nostruct '-' Expression_nostruct
| Expression_nostruct '*' Expression_nostruct
| Expression_nostruct '/' Expression_nostruct
| Expression_nostruct '%' Expression_nostruct
| Expression_nostruct '..'
| Expression_nostruct '..' Expression_nostruct
| '..' Expression_nostruct
| '..'
| Expression_nostruct 'as' Type
| 'box' Expression_nostruct
| 'box' '(' Maybe_expression ')' Expression_nostruct
| Expression_qualified_path
| Block_expression
| Block
| Nonblock_prefix_expression;

syntax Nonblock_prefix_expression_nostruct
= '-' Expression_nostruct
| '!' Expression_nostruct
| '*' Expression_nostruct
| '&' Maybe_mut Expression_nostruct
| '&&' Maybe_mut Expression_nostruct
| Lambda_expression_nostruct
| 'move' Lambda_expression_nostruct
| Proc_expression_nostruct;

syntax Nonblock_prefix_expression
= '-' Expression
| '!' Expression
| '*' Expression
| '&' Maybe_mut Expression
| '&&' Maybe_mut Expression
| Lambda_expression
| 'move' Lambda_expression
| Proc_expression;

syntax Expression_qualified_path
= '\<' Type_sum Maybe_as_trait_ref '\>' '::' Identifier\ 
  Maybe_qpath_params
| '\<\<' Type_sum Maybe_as_trait_ref '\>' '::' Identifier\
  Maybe_as_trait_ref '\>' '::' Identifier
| '\<\<' Type_sum Maybe_as_trait_ref '\>' '::' Identifier\ 
  Generic_args Maybe_as_trait_ref '\>' '::' Identifier
| '\<\<' Type_sum Maybe_as_trait_ref '\>' '::' Identifier\
  Maybe_as_trait_ref '\>' Identifier Generic_args
| '\<\<' Type_sum Maybe_as_trait_ref '\>' '::' Identifier\
  Generic_args Maybe_as_trait_ref '\>' '::' Identifier Generic_args ;

syntax Maybe_qpath_params
= '::' Generic_args
| /*empty*/;

syntax Maybe_as_trait_ref
= 'as' Trait_ref
| /*empty*/;

syntax Lambda_expression
= '||' Ret_type Expression
| '|' Maybe_unboxed_closure_kind '|' Ret_type Expression
| '|' Inferrable_params '|' Ret_type Expression
| '|' '&' Maybe_mut ':' Inferrable_params '|' Ret_type Expression
| '|' ':' Inferrable_params '|' Ret_type Expression;

syntax Lambda_expression_nostruct
= '||' Ret_type Expression_nostruct
| '|' Maybe_unboxed_closure_kind '|' Ret_type Expression_nostruct
| '|' Inferrable_params '|' Ret_type Expression_nostruct
| '|' '&' Maybe_mut ':' Inferrable_params '|' Ret_type Expression_nostruct
| '|' ':' Inferrable_params '|' Ret_type Expression_nostruct;

syntax Proc_expression
= 'proc' '(' ')' Expression
| 'proc' '(' Inferrable_params ')' Expression;

syntax Proc_expression_nostruct
= 'proc' '(' ')' Expression_nostruct
| 'proc' '(' Inferrable_params ')' Expression_nostruct;

syntax Vector_expression
= Maybe_expression
| Expressions ';' Expression;

syntax Structure_expression_fields
= Field_inits
| Field_inits ','
| Maybe_field_inits Defailt_field_init;

syntax Maybe_field_inits
= Field_inits
| Field_inits ','
| /*empty*/;

syntax Field_inits
= Field_init
| Field_inits ',' Field_init;

syntax Field_init
= Identifier ':' Expression;

syntax Default_field_init
= '..' Expression;

syntax Block_expression
= Expression_match
| Expression_if
| Expression_if_let
| Expression_while
| Expression_while_let
| Expression_loop
| Expression_for
| 'unsafe' Block
| Path_expression '!' Maybe_identifier Braces_delimited_token_trees;

syntax Full_block_expression
= Block_expression
| Full_block_expression '.' Path_generic_args_with_colons
| Full_block_expression '.' Path_generic_args_with_colons '[' Maybe_expression ']'
| Full_block_expression '.' Path_generic_args_with_colons '(' Maybe_expressions ')'
| Full_block_expression '.' Literal_integer;

syntax Expression_match
= 'match' Expression_nostruct '{' '}'
| 'match' Expression_nostruct '{' Match_clauses'}'
| 'match' Expression_nostruct '{' Match_clauses Nonblock_match_clause '}'
| 'match' Expression_nostruct '{' Nonblock_match_clause'}';

syntax Match_clauses
= Match_clause
| Match_clauses Match_clause;

syntax Match_clause
= Nonblock_match_clause ','
| Block_match_clause
| Block_match_clause ',';

syntax Nonblock_match_clause
= Maybe_outer_attributes Patterns_or Maybe_guard '=\>' Nonblock_expression
| Maybe_outer_attributes Patterns_or Maybe_guard '=\>' Full_block_expression;

syntax Block_match_clause
= Maybe_outer_attributes Patterns_or Maybe_guard '=\>' Block;

syntax Maybe_guard
= 'if' Expression_nostruct
| /*empty*/;

syntax Expression_if
= 'if' Expression_nostruct Block
| 'if' Expression_nostruct Block 'else' Block_or_if;

syntax Expression_if_let
= 'if' 'let' Pattern '=' Expression_nostruct Block
| 'if' 'let' Pattern '=' Expression_nostruct Block 'else' Block_or_if;

syntax Block_or_if
= Block
| Expression_if
| Expression_if_let;


syntax Expression_while
= Maybe_label 'while' Expression_nostruct Block;

syntax Expression_while_let
= Maybe_label 'while' 'let' Pattern '=' Expression_nostruct Block;

syntax Expression_loop
= Maybe_label 'loop' Block;

syntax Expression_for
= Maybe_label 'for' Pattern 'in' Expression_nostruct Block;

syntax Maybe_label
= Lifetime ':'
| /*empty*/;

syntax Let
= 'let' Pattern Maybe_type_ascription Maybe_init_expression ';';

/* #### #### Macros and misc. rules #### ####*/

syntax Literal
= Literal_byte
| Literal_char
| Literal_integer
| Literal_float
| 'true'
| 'false'
| String;

syntax String
= Literal_string
| Literal_string_raw
| Literal_byte_string
| Literal_byte_string_raw;

syntax Maybe_identifier
= /*empty*/
| Identifier;

syntax Identifier
= Ident;

syntax Unpaired_token
= '\<\<'                        
| '\>\>'                        
| '\<='                         
| '=='                       
| '!='                         
| '\>='                         
| '&&'                     
| '||'                       
| '\<-'                     
| '\<\<='                      
| '\>\>='                      
| '-='                    
| '&='                      
| '|='                       
| '+='                     
| '*='                     
| '/='                    
| '^='                    
| '%='                  
| '..'                     
| '...'                  
| '::'                    
| '-\>'                     
| '=\>'                  
| Literal_byte                   
| Literal_char                   
| Literal_integer                
| Literal_float                  
| Literal_string                    
| Literal_string_raw                
| Literal_byte_string               
| Literal_byte_string_raw           
| Ident                      
| '_'                 
| '\''                   
| 'self'                       
| 'static'                     
| 'as'                         
| 'break'                      
| 'crate'                      
| 'else'                       
| 'enum'                       
| 'extern'                     
| 'false'                      
| 'fn'                         
| 'for'                        
| 'if'                         
| 'impl'                       
| 'in'                         
| 'let'                        
| 'loop'                       
| 'match'                      
| 'mod'                        
| 'move'                       
| 'mut'                        
| 'priv'                       
| 'pub'                        
| 'ref'                        
| 'return'                     
| 'struct'                     
| 'true'                       
| 'trait'                      
| 'type'                       
| 'unsafe'                     
| 'use'                        
| 'while'                      
| 'continue'                   
| 'proc'                       
| 'box'                        
| 'const'                      
| 'where'                      
| 'typeof'                     
| Inner_doc_comment       
| Outer_doc_comment          
| Shebang                    
| '\'static'            
| ';'                        
| ','                        
| '.'                        
| '@'                        
| '#'                        
| '~'                        
| ':'                        
| '$'                        
| '='                        
| '?'                        
| '!'                        
| '\<'                        
| '\>'                        
| '-'                        
| '&'                        
| '|'                        
| '+'                        
| '*'                        
| '/'                        
| '^'                        
| '%';

syntax Token_trees
= /*empty*/
| Token_trees Token_tree;

syntax Token_tree
= Delimited_token_trees
| Unpaired_token;

syntax Delimited_token_trees
= Parens_delimited_token_trees
| Braces_delimited_token_trees
| Brackets_delimited_token_trees;

syntax Parens_delimited_token_trees
= '(' Token_Trees ')';

syntax Braces_delimited_token_trees
= '{' Token_trees '}';

syntax Brackets_delimited_token_trees
= '[' Token_trees ']';
