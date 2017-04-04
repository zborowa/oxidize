@contributors{Adrian Zborowski - 11336544 - adrian.zborowski@uva.nl}

module lang::rust::\syntax

layout Whitespace = [\ \t\r\n]* !>> [\ \t\r\n];

lexical Identifier = [a-zA-Z][a-zA-Z0-9]* !>> [a-zA-Z0-9];

lexical Hash = '#';
lexical Shebang = Hash '!' '[';
lexical Shebang_line = Hash '!' '^[\n]*\n';

/* #### #### Items and attributes #### #### */

lexical Crate
= Maybe_shebang Inner_attributes Maybe_mod_items
| Maybe_shebang Maybe_mod_items;

lexical Maybe_shebang
= Shebang_line
| /*empty*/;

lexical Maybe_inner_attributes
= Inner_attributes
| /*empty*/;

lexical Inner_attributes
= Inner_attribute
| Inner_attributes Inner_attribute;

lexical Inner_attribute
= Shebang '[' Meta_item ']'
| Inner_doc_comment;

lexical Maybe_outer_attributes
= Outer_attributes
| /*empty*/;

lexical Outer_attributes
= Outer_attributes
| Outers_attributes Outer_attribute;

lexical Outer_attribute
= '#' '[' Meta_item ']'
| Outer_document_comment;

lexical Meta_item
= Identifier
| Identifier '=' Literal
| Identifier '(' Meta_sequence ')'
| Identifier '(' Meta_sequence ',' ')';

lexical Meta_sequence 
= /*empty*/
| Meta_item
| Meta_sequence ',' Meta_item;

lexical Maybe_mod_items
= Mod_items
| /*empty*/;

lexical Mod_items
= Mod_item
| Mod_items Mod_item;

lexical Attributes_and_vis
= Maybe_outer_attributes Visibility;

lexical Mod_item
= Attributes_and_vis Item;

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

lexical Structure_declaration_args
= '{' Structure_declaration_fields '}'
| '{' Structure_declaration_fields ',' '}';

lexical Structure_typle_args
= '(' Structure_tuple_fields ')'
| '(' Structure_tuple_fields ',' ')';

lexical Structure_declaration_fields
= Structure_declaration_field
| Structure_declaration_fields ',' Structure_declaration_field
| /*empty*/;

lexical Structure_decl_field
= Attributes_and_vis Identifier ':' Type_sum;

lexical Structure_tuple_fields
= Structure_tuple_field
| Structure_tuple_fields ',' Structure_tuple_field;

lexical Structure_tuple_field
= Attributes_and_vis Type_sum;

lexical Item_enum
= 'enum' Identifier Generic_params Maybe_where_clause '{' Enum_defs '}'
| 'enum' Identifier Generic_params Maybe_where_clause '{' Enum_defs ',' '}';

lexical Enum_defs 
= Enum_def
| Enum_defs ',' Enum_def
| /*empty*/;

lexical Enum_def
= Attributes_and_vis Identifier Enum_args;

lexical Enum_args
= '{' Structure_decl_fields '}'
| '{' Structure_decl_fields ',' '}'
| '{' Maybe_type_sum '}'
| '=' Expression
| /*empty*/;

lexical Item_mod
= 'mod' Identifier ';'
| 'mod' Identifier '{' Maybe_mod_items '}'
| 'mod' Identifier '{' Inner_attrs Maybe_mod_items '}';

lexical Item_foreign_mod
= 'extern' Maybe_abi '{' Maybe_foreign_items '}'
| 'extern' Maybe_abi '{' Inner_attrs Maybe_foreign_items '}';

lexical Maybe_abi
= String
| /*empty*/;

lexical Maybe_foreign_items
= Foreign_items
| /*empty*/;

lexical Foreign_items
= Foreign_item
| Foreign_items Foreign_item;

lexical Foreign_item
= Attributes_and_vis 'static' Item_foreign_static
| Attributes_and_vis Item_foreign_fn
| Attributes_and_vis 'unsafe' Item_foreign_fn;

lexical Item_foreign_static
= Maybe_mut Identifier ':' Types ';';

lexical Item_foreign_fn
= 'fn' Identifier Generic_params Fn_declaration_allow_variadic Maybe_where_clause ';';

lexical Fn_params_allow_variadic
= '(' ')'
| '(' Params ')'
| '(' Params ',' ')'
| '(' Params ',' '...' ')';

lexical Visibility
= 'pub'
| /*empty*/;

lexical Identifiers_or_self
= Identifier_or_self
| Identifier_or_self 'as' Identifier
| Identifiers_or_self ',' Identifier_or_self;

lexical Identifier_or_self
= Identifier
| 'self';

lexical Item_type
= 'type' Identifier Generic_params Maybe_where_clause '=' Type_sum ';';

lexical For_sized 
= 'for' '?' Identifier
| 'for' Identifier '?'
| /*empty*/;

lexical Item_trait
= Maybe_unsafe 'trait' Identifier Generic_params For_sized Maybe_type_param_bounds Maybe_where_clause \
  '{' Maybe_trait_items '}';
  
lexical Maybe_trait_items
= Trait_items
| /*empty*/;

lexical Trait_items
= Trait_item
| Trait_items Trait_item;

lexical Trait_item
= Trait_const
| Trait_type
| Trait_method;

lexical Trait_const
= Maybe_outer_attributes 'const' Identifier Maybe_type_ascription Maybe_const_defailt ';';

lexical Maybe_const_default
= '=' Expression
| /*empty*/;

lexical Trait_type
= Maybe_outer_attributes 'type' Type_param ';';

lexical Maybe_unsafe
= 'unsafe'
| /*empty*/;

lexical Trait_method
= Type_method
| Method;

lexical Type_method
= Attributes_and_vis Maybe_unsafe 'fn' Identifier Generic_params \
  Fn_decl_with_self_allow_anon_params Maybe_where_clause ';'
| Attributes_and_vis Maybe_unsafe 'extern' Maybe_abi 'fn' Identifier Generic_params \
  Fn_decl_with_self_allow_anon_params Maybe_where_clause ';';
  
lexical Method
= Attributes_and_vis Maybe_unsafe 'fn' Identifier Generic_params \
  Fn_decl_with_self_allow_anon_params Maybe_where_claus Inner_attributes_and_block
| Attributes_and_vis Maybe_unsafe 'extern' Maybe_abi 'fn' Identifier Generic_params \
  Fn_decl_with_self_allow_anon_params Maybe_where_claus Inner_attributes_and_block;

lexical Impl_method
= Attributes_and_vis Maybe_unsafe 'fn' Identifier Generic_params \
  Fn_decl_with_self Maybe_where_clause Inner_attributes_and_block
| Attributes_and_vis Maybe_unsafe 'extern' Maybe_abi 'fn' Identifier Generic_params \
  Fn_decl_with_self Maybe_where_clause Inner_attributes_and_block;
  
lexical Item_impl
= Maybe_unsafe 'impl' Generic_params Type_primitive_sum Maybe_where_clause '{' Maybe_inner_attributes Maybe_impl_items '}'
| Maybe_unsafe 'impl' Generic_params '(' Type ')' Maybe_where_clause '{' Maybe_inner_attributes Maybe_impl_items '}'
| Maybe_unsafe 'impl' Generic_params Trait_ref 'for' Type_sum Maybe_where_clause '{' Maybe_inner_attributes Maybe_impl_items '}'
| Maybe_unsafe 'impl' Generic_params '!' Trait_ref 'for' Type_sum Maybe_where_clause '{' Maybe_inner_attributes Maybe_impl_items '}'
| Maybe_unsafe 'impl' Generic_params Trait_ref 'for' '..' '{' '}'
| Maybe_unsafe 'impl' Generic_params '!' Trait_ref 'for' '..' '{' '}';

lexical Maybe_impl_items
= Impl_items
| /*empty*/;

lexical Impl_items
= Impl_item
| Impl_item Impl_items;

lexical Impl_item
= Impl_method
| Attributes_and_vis Item_macro
| Impl_const
| Impl_type;

lexical Impl_const
= Attributes_and_vis Item_const;

lexical Impl_type
= Attributes_and_vis 'type' Identifier Generic_params '=' Type_sum ';';

lexical Item_fn
= 'fn' Identifier Generic_params Fn_decl Maybe_where_clause Inner_attributes_and_block;

lexical Item_unsafe_fn
= 'unsafe' 'fn' Identifier Generic_params Fn_decl Maybe_where_clause Inner_attributes_and_block
| 'unsafe' 'extern' Maybe_abi 'fn' Identifier Generic_params Fn_decl Maybe_where_clause Inner_attributes_and_block;

lexical Fn_decl
= Fn_params Ret_type;

lexical Fn_decl_with_self
= Fn_params_with_self Ret_type;

lexical Fn_decl_with_self_allow_anon_params
= Fn_anon_params_with_self Ret_type;

lexical Fn_params
= '(' Maybe_params ')';

lexical Fn_anon_params
= '(' Anon_param Anon_params_allow_variadic_tail ')'
| '('  ')';

lexical Fn_params_with_self
= '(' Maybe_mut 'self' Maybe_type_ascription Maybe_comma_params ')'
| '(' '&' Maybe_mut 'self' Maybe_type_ascription Maybe_comma_params ')'
| '(' '&'  Lifetime Maybe_mut 'self' Maybe_type_ascription Maybe_comma_params ')'
| '(' Maybe_params ')';

lexical Fn_anon_params_with_self
= '(' Maybe_mut 'self' Maybe_type_ascription Maybe_comma_anon_params ')'
| '(' '&' Maybe_mut 'self' Maybe_type_ascription Maybe_comma_anon_params ')'
| '(' '&'  Lifetime Maybe_mut 'self' Maybe_type_ascription Maybe_comma_anon_params ')'
| '(' Maybe_anon_params ')';

lexical Maybe_params
= Params
| Params ','
| /*empty*/;

lexical Params
= Param
| Params ',' Param;

lexical Param
= Pat ':' Type_sum;

lexical Inferrable_params
= Inferrable_param
| Inferrable_params ',' Inferrable_param;

lexical Inferrable_param
= Pattern Maybe_type_ascription;

lexical Maybe_unboxed_closure_kind
= /*empty*/
| ':'
| '&' Maybe_mut ':';

lexical Maybe_comma_params
= ','
| ',' Params
| ',' Params ','
| /*empty*/;

lexical Maybe_comma_anon_params
= ','
| ',' Anon_params
| ',' Anon_params ','
| /*empty*/;

lexical Maybe_anon_params
= Anon_params
| Anon_params ','
| /*empty*/;

lexical Anon_params
= Anon_param
| Anon_params ',' Anon_param;

lexical Anon_param
= Named_arg ':' Type
| Type;

lexical Anon_params_allow_variadic_tail
= ',' '...'
| ',' Anon_param Anon_params_allow_variadic_tail;

lexical Named_arg
= Identifier
| '_'
| '&' Identifier
| '&' '_'
| '&&' Identifier
| '&&' '_'
| 'mut' Identifier;

lexical Ret_type
= '-\>' '!'
| '-\>' Type
| Identifier /*empty*/
;

lexical Generic_params
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

lexical Maybe_where_clause
= /*empty*/
| Where_clause;

lexical Where_clause
= 'where' Where_predicates
| 'where' Where_predicates ',';

lexical Where_predicates
= Where_predicate
| Where_predicates ',' Where_predicate;

lexical Where_predicate
= Maybe_for_lifetimes LifeTime ':' Bounds
| Maybe_for_lifetimes Type ':' Type_param_bounds;

lexical Maybe_for_lifetimes
= 'for' '\<' LifeTime '\>'
| 'for' /*empty*/
;

lexical Type_params
= Type_param
| Type_params ',' Type_param;

lexical Path_no_types_allowed
= Identifier
| '::' Identifier
| 'self'
| '::' 'self'
| Path_no_types_allowed '::' Identifier;

lexical Path_generic_args_without_colons
= Identifier
| Identifier Generic_args
| Identifier '(' Maybe_type_sums ')' Ret_type
| Path_generic_args_without_colons '::' Identifier
| Path_generic_args_without_colons '::' Identifier Generic_args
| Path_generic_args_without_colons '::' Identifier '(' Maybe_type_sums ')' Ret_type
;

lexical Generic_args
= '\<' Generic_values '\>'
| '\<' Generic_values '\>\>'
| '\<' Generic_values '\>='
| '\<' Generic_values '\>\>='
| '\<\<' Type_qualified_path_and_generic_values '\>'
| '\<\<' Type_qualified_path_and_generic_values '\>\>'
| '\<\<' Type_qualified_path_and_generic_values '\>='
| '\<\<' Type_qualified_path_and_generic_values '\>\>=';

lexical Generic_values
= Maybe_lifetimes Maybe_type_sums_and_or_bindings;

lexical Maybe_type_sums_and_or_bindings
= Type_sums
| Type_sums ','
| Type_sums ',' Bindings
| Bindings
| Bindings ','
| /*empty*/;

lexical Bindings
= ',' Bindings
| /*empty*/;

/* #### #### Patterns #### #### */

lexical Pattern
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

lexical Patterns_or
= Pattern
| Patterns_or '|' Pattern;

lexical Binding_mode
= 'ref'
| 'ref' 'mut'
| 'mut';

lexical Literal_or_path
= Path_expression
| Literal
| '-' Literal;

lexical Pattern_field
= Identifier
| Binding_mode Identifier
| 'box' Identifier
| 'box' Binding_mode Identifier
| Identifier ':' Pattern
| Binding_mode Identifier ':' Pattern;

lexical Pattern_fields
= Pattern_field
| Pattern_fields ',' Pattern_field;

lexical Pattern_structure
= Patterns_fields
| Pattern_fields ','
| Pattern_fields ',' '..'
| '..';

lexical Pattern_tuple
= Pattern
| Patterns_tuple ',' Pattern;

lexical Pattern_vector
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

lexical Pattern_vector_elts
= Pattern
| Pattern_vector_elts ',' Pattern;

/* #### #### Types #### #### */

lexical Type 
= Type_primitive
| Type_closure
| '\<' Type_sum Maybe_as_trait_ref '\>' '::' Identifier
| '\<\<' Type_sum Maybe_as_trait_ref '\>\>' '::' Identifier Maybe_as_trait_ref '\>' '::' Identifier
| '(' Type_sums ')'
| '(' Type_sums ',' ')'
| '(' ')';

lexical Type_primitive
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

lexical Type_bare_fn 
= 'fn' Type_fn_decl
| 'unsafe' 'fn' Type_fn_decl
| 'extern' Maybe_abi 'fn' Type_fn_decl
| 'unsafe' 'extern' Maybe_abi 'fn' Type_fn_decl;

lexical Type_fn_decl
= Generic_params Fn_anon_params Ret_type;

lexical Type_closure
= 'unsafe' '|' Anon_params '|' Maybe_bounds Ret_type
| '|' Anon_params '|' Maybe_bounds Ret_type
| 'unsafe' '||' Maybe_bounds Ret_type
| '||' Maybe_bounds Ret_type;

lexical Type_process 
= 'proc' Generic_params Fn_params Maybe_bounds Ret_type;

lexical For_in_type
= 'for' '\<' Maybe_lifetime '\>' For_in_type_suffix;

lexical For_in_type_suffix
= Type_process
| Type_bare_fn
| Trait_ref
| Type_closure;

lexical Maybe_mut
= 'mut'
| 'mut' /*empty*/;

lexical Maybe_mut_or_const
= 'mut'
| 'const' 
| /*empty*/;

lexical Type_qualified_path_and_generic_values
= Type_qualified_path Maybe_bindings
| Type_qualified_path ',' Type_sums Maybe_bindings;

lexical Type_qualified_path
= Type_sum 'as' Trait_ref '\>' '::' Identifier
| Type_sum 'as' Trait_ref '\>' '::' Identifier '+' Type_param_bounds;

lexical Maybe_type_sums
= Type_sums
| Type_sums ','
| /*empty*/;

lexical Type_sums
= Type_sum
| Type_sums ',' Type_sum;

lexical Type_sum
= Type
| Type '+' Type_param_bounds;

lexical Type_primitive_sum
= Type_primitive
| Type_primitive '+' Type_param_bounds;

lexical Maybe_type_param_bounds
= ':' Type_param_bounds
| /*empty*/;

lexical Type_param_bounds
= Bounds_sequence
| /*empty*/;

lexical Bounds_sequence
= Poly_bound
| Bound_sequence '+' Poly_bound;

lexical Poly_bound
= 'for' '\<' Maybe_lifetime '\>' Bound
| Bound
| '?' Bound;

lexical Bindings
= Binding
| Bindings ',' Binding;

lexical Binding
= Identifier '=' Type;

lexical Type_param
= Identifier Maybe_type_param_bounds Maybe_type_default
| Identifier '?' Identifier Maybe_type_param_bounds Maybe_type_default;

lexical Maybe_bounds
= ':' Bounds
| /*empty*/;

lexical Bounds
= Bound
| Bounds '+' Bound;

lexical Bound
= Lifetime
| Trait_ref;

lexical Maybe_ltbounds
= ':' Ltbounds
| /*empty*/;

lexical Ltbounds
= Lifetime
| LtBounds '+' Lifetime;

lexical Maybe_type_default
= '=' Type_sum
| /*empty*/;

lexical Maybe_lifetimes
= Lifetimes
| Lifetimes ','
| /*empty*/;

lexical Lifetimes
= Lifetime_and_bounds
| Lifetimes ',' Lifetime_and_bounds;

lexical Lifetime_and_bounds
= '\'' Maybe_ltbounds
| '\'static';

lexical Lifetime
= '\''
| '\'static';

lexical Trait_ref
= Path_generic_args_without_colons
| '::' Path_generic_args_without_colons;
