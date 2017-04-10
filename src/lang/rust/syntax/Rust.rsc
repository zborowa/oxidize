@contributors{Adrian Zborowski - 11336544 - adrian.zborowski@uva.nl}

module lang::rust::\syntax::Rust

import Prelude;
import vis::ParseTree;
import analysis::grammars::Ambiguity;


layout Whitespace 
	= WhiteSpaceOrComment* !>> [\ \t\r\n] !>> "//" !>> "/*"
	;

lexical WhiteSpaceOrComment
	= [\ \t \r \n]
	| Comment
	;

lexical Comment
	= @category="Comment" "/*" (![*] | [*] !>> [/])* "*/" 
	| @category="Comment" "//" ![\n]* !>> [\ \t\r \u00A0 \u1680 \u2000-\u200A \u202F \u205F \u3000] $ // the restriction helps with parsing speed
	;
	
keyword Rust_keywords
	= "abstract" | "alignof" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do" | "else" 
	| "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" 
	| "mod" | "move" | "mut" | "offsetof" | "override" | "priv" | "proc" | "pub" | "pure" | "ref" | "return" | "Self" 
	| "self" | "sizeof" | "static" | "struct" | "super" | "trait" | "true" | "type" | "typeof" | "unsafe" | "unsized" 
	| "use" | "virtual" | "where" | "while" | "yield"
	;

// Identifier regex not to be confused with syntax Identifier present in the file
lexical Ident
	= 	(
			[a-z A-Z \u0080-\u00ff _] !<< 
			[a-z A-Z \u0080-\u00ff _] [a-z A-Z 0-9 \u0080-\u00ff _]* !>> 
			[a-z A-Z 0-9 \u0080-\u00ff _]
		) \ Rust_keywords
	;

lexical Literal_byte
	= "b\'" "\\" [xX] Hexit Hexit "\'"
	| "b\'" "\\" [n r t \\ \' \' 0] "\'"
	| "b\'" ![\\ \' \n \r \t] [\udc00-\udfff]? "\'"
	;

lexical Literal_char 
	= "\'" "\\" UnicodeEscape "\'"
	| "\'" ![\\ \' \n \t \r] "\'"
	| "\'" [\ud800-\udbff] [\udc00-\udfff] "\'"
	;

lexical Literal_integer
	= [0-9][0-9 _]*
	| "0b" [0 1 _]+
	| "0o" [0-7 _]+
	| "0x" [0-9 a-f A-F _]+
	;

lexical Literal_float 
	= [0-9] [0-9 _]* [.] [0-9 _]* ([eE] [\- +]? [0-9 _]+)?
	| [0-9] [0-9 _]* ([.] [0-9 _]*)? [eE] [\- +]? [0-9 _]+
	;

// ASCII representations of unicode ranges
//ASC     [\x00-\x7f]
//ASCN    [\x00-\t\v-\x7f]
//U       [\x80-\xbf]
//U2      [\xc2-\xdf]
//U3      [\xe0-\xef]
//U4      [\xf0-\xf4]

lexical Literal_string
	= "\"" StringChar* "\""
	;

lexical StringChar
	= UnicodeEscape
	| [\a00] 
	| "\\" [\n]
	| "\\" [\r \n]
	| ![\" \\]+ !>> ![\" \\]	
	;
	
lexical Hexit
	= [0-9 a-f A-F]
	;
	
lexical UnicodeEscape
	= "\\"   [n r t \\ \' \" 0]
	| "\\"   [xX] [0-9 a-f A-F] [0-9 a-f A-F]
	| "\\u{" Hexit Hexit Hexit Hexit Hexit Hexit "}"
	| "\\u{" Hexit Hexit Hexit Hexit Hexit "}"
	| "\\u{" Hexit Hexit Hexit Hexit  "}"
	| "\\u{" Hexit Hexit Hexit  "}"
	| "\\u{" Hexit Hexit  "}"
	| "\\u{" Hexit  "}"
	| "\\u"  Hexit Hexit Hexit Hexit
	| "\\U"  Hexit Hexit Hexit Hexit Hexit Hexit Hexit Hexit
	; 

lexical Literal_string_raw 
	= [r] Literal_string
	;

lexical Literal_byte_string 
	= [b] Literal_string
	;

lexical Literal_byte_string_raw
	= [b] Literal_string_raw
	;

lexical Shebang 
	= "#!"
	;

lexical Shebang_line 
	= Shebang [^\n]*
	;

/* #### #### Items and attributes #### #### */

start syntax Crate
	= crate:Shebang_line? Inner_attributes inner_attributes Mod_item? mod_idem
	| crate:Shebang_line? Mod_item? mode_item
	;
	
syntax Inner_attributes
	= Inner_attribute+
	;

syntax Inner_attribute
	= inner_attributes:Shebang "[" Meta_item meta_item "]"
	| inner_attributes:Comment comment
	;

syntax Outer_attribute
	= "#" "[" Meta_item meta_item "]"
	;

syntax Meta_item
	= meta_word:Identifier identifier
	| meta_name_value:Identifier identifier "=" Literal literal
	| meta_list:Identifier identifier "(" {Meta_item ","}* meta_items ","? ")"
	;

syntax Attributes_and_vis
	= AttrsAndVis:Outer_attribute* outer_attributes "pub"? visibility
	;

syntax Mod_item
	= item:Attributes_and_vis attribute_visbility Item item
	;

syntax Item 
	= Statement_item statement_item
	| Item_macro item_macro
	;

syntax Statement_item 
	= Item_static item_static
	| Item_const item_const
	| Item_type item_type
	| Block_item block_item
	| View_item view_item
	;

syntax Item_static
	= item_static:"static" "mut"? Identifier identifier ":" Type type "=" Expression expression ";"
	;

syntax Item_const
	= item_const:"const" Identifier identifier ":" Type type "=" Expression expression ";"
	;

syntax Item_macro
	= item_macro:Path_expression path_expression "!" Identifier? identifier 
		Parens_delimited_token_trees token_trees ";"
	| item_macro:Path_expression path_expression "!" Identifier? identifier 
		Braces_delimited_token_trees token_trees
	| item_macro:Path_expression path_expression "!" Identifier? identifier 
		Brackets_delimited_token_trees token_trees ";"
	;

syntax View_item
	= Use_item
	| Extern_fn_item
	| view_item_extern_crate:"extern" "crate" Identifier identifier ";"
	| view_item_extern_crate:"extern" "crate" Identifier identifier "as" Identifier as ";"
	;

syntax Use_item
	= view_item_use:"use" View_path view_path ";"
	;

syntax Extern_fn_item 
	= view_item_extern_fn:"extern" String? abi Item_fn item_fn
	;

syntax View_path
	= view_path_simple:Path_no_types_allowed
	| view_path_list:Path_no_types_allowed "::" "{" "}"
	| view_path_list:"::" "{" "}"
	| view_path_list:Path_no_types_allowed "::" "{" Identifiers_or_self "}"
	| view_path_list:"::" "{" Identifiers_or_self "}"
	| view_path_list:Path_no_types_allowed "::" "{" Identifiers_or_self "," "}"
	| view_path_list:"::" "{" Identifiers_or_self "}"
	| view_path_glob:Path_no_types_allowed "::" "*"
	| view_path_list_empty:"{" "}"
	| view_path_list:"{" Identifiers_or_self "}"
	| view_path_list:"{" Identifiers_or_self "," "}"
	| view_path_simple:Path_no_types_allowed "as" Identifier
	;

syntax Block_item
	= Item_fn
	| Item_unsafe_fn
	| Item_mod
	| item_foreign_mod:Item_foreign_mod
	| Item_struct
	| Item_enum
	| Item_trait
	| Item_impl
	;

syntax Type_ascription
	= ":" Type_sum type_sum
	;

syntax Init_expression
	= "=" Expression expression
	;

syntax Item_struct
	= item_struct:"struct" Identifier identifier Generic_params? generic_params 
		Where_clause? where Struct_decl_args
	| item_struct:"struct" Identifier identifier Generic_params? generic_params 
		Struct_tuple_args struct_tuple_args Where_clause? where ";"
	| item_struct:"struct" Identifier identifier Generic_params? generic_params 
		Where_clause? where ";"
	;

syntax Struct_decl_args
	= "{" {Struct_decl_field ","}* struct_decl_fields ","? "}"
	;

syntax Struct_tuple_args
	= "(" {Struct_tuple_field ","}+ struct_tuple_fields ","? ")"
	;

syntax Struct_decl_field
	= struct_field:Attributes_and_vis Identifier identifier ":" Type_sum type_sum
	;

syntax Struct_tuple_field
	= struct_field:Attributes_and_vis Type_sum type_sum
	;

syntax Item_enum
	= item_enum:"enum" Identifier identifier Generic_params? generic_params 
		Where_clause? where "{" {Enum_def ","}* enum_defs","? "}"
	;

syntax Enum_def
	= enum_def:Attributes_and_vis Identifier identifier Enum_args? enum_args
	;

syntax Enum_args
	= enum_args:"{" {Struct_decl_field ","}* struct_decl_fields"}"
	| enum_args:"{" {Type_sum ","}* type_sums "}"
	| enum_args:"=" Expression expression
	;

syntax Item_mod
	= item_mod:"mod" Identifier identifier ";"
	| item_mod:"mod" Identifier identifier "{" Inner_attribute+ inner_attributes Mod_item* mode_items"}"
	;

syntax Item_foreign_mod
	= item_foreign_mod:"extern" String? name "{" Inner_attribute* Foreign_item* "}"
	;

syntax Foreign_item
	= foreign_item:Attributes_and_vis "static" "mut"? Identifier identifier 
		":" Type type ";"
	| foreign_item:Attributes_and_vis "unsafe"? "fn" Identifier identifier 
		Generic_params? generic_params "(" Params? params ")" Ret_type Where_clause? where ";"
	;

syntax Identifiers_or_self
	= idents_or_self:{(Identifier identifier | "self" self) ","}+ ("as" Identifier identifier)?
	;

syntax Item_type
	= item_type:"type" Identifier identifier Generic_params? generic_params 
		Where_clause? where "=" Type_sum type_sum ";"
	;

syntax For_sized 
	= for_sized:"for" (("?" Identifier identifier) | (Identifier identifier "?"))
	;

syntax Item_trait
	= item_trait:"unsafe"? "trait" Identifier identifier Generic_params? generic_params For_sized? for_sized 
		Type_param_bounds? bounds Where_clause? where "{" Trait_item+ trait_items "}"
	;

syntax Trait_item
	= Trait_const
	| Trait_type
	| Trait_method
	;

syntax Trait_const
	= const_trait_item:Outer_attribute? outer_attribute "const" Identifier identifier 
		Type_ascription? type_ascription ("=" Expression)? ";"
	;

syntax Trait_type
	= type_trait_item:Outer_attribute? "type" Type_param ";"
	;

syntax Trait_method
	= required:Attributes_and_vis "unsafe"? ("extern" String?)? "fn" 
		Identifier Generic_params? Fn_decl_with_self_allow_anon_params Where_clause? ";"
	| provided:Attributes_and_vis "unsafe"? ("extern" String?)? "fn" 
		Identifier Generic_params? Fn_decl_with_self_allow_anon_params Maybe_where_claus Inner_attributes_and_block
	;

/*
// spelen met de syntax!!
syntax Modifiers
	= "unsafe"
	| "extern" String abi?
	| "pub"
	;
	
syntax Method
	= Attributes* attrs Modifiers* mods "fn" Identifier name GenericParams generic "(" {Parameter ","}+ params ")" 
		Where? clause InnerAttributes* Block; 
*/

syntax Method
	= Attributes_and_vis "unsafe"? ("extern" String?)? "fn" Identifier identifier Generic_params? generic_params 
		Fn_decl_with_self_allow_anon_params Where_clause? Inner_attrs_and_block
	;

syntax Impl_method
	= method:Attributes_and_vis "unsafe"? ("extern" String?)? "fn" Identifier Generic_params? 
		Fn_decl_with_self Where_clause? Inner_attributes_and_block
	;
  
syntax Item_impl
	= item_impl:"unsafe"? "impl" Generic_params? 
		Type_primitive_sum Where_clause? "{" Inner_attribute+ Impl_items? "}"
	| item_impl:"unsafe"? "impl" Generic_params? 
		"(" Type ")" Where_clause? "{" Inner_attribute+ Impl_items? "}"
	| item_impl:"unsafe"? "impl" Generic_params? 
		Trait_ref "for" Type_sum Where_clause? "{" Inner_attribute+ Impl_items? "}"
	| item_impl_neg:"unsafe"? "impl" Generic_params? 
		"!" Trait_ref "for" Type_sum Where_clause? "{" Inner_attribute+ Impl_items? "}"
	| item_impl_default:"unsafe"? "impl" Generic_params? 
		Trait_ref "for" ".." "{" "}"
	| item_impl_default_neg:"unsafe"? "impl" Generic_params? 
		"!" Trait_ref "for" ".." "{" "}"
	;

syntax Impl_items
	= ImplItems:Impl_item+ impl_items
	;

syntax Impl_item
	= Impl_method
	| impl_macro_item:Attributes_and_vis Item_macro
	| impl_const:Attributes_and_vis Item_const
	| impl_type:Attributes_and_vis "type" Identifier Generic_params? "=" Type_sum ";"
	;

syntax Item_fn
	= item_fn:"fn" 
		Identifier identifier 
		Generic_params? generic_params 
		Fn_decl 
		Where_clause? 
		Inner_attributes_and_block
	;

syntax Item_unsafe_fn
	= item_unsafe_fn:"unsafe" ("extern" String?)? "fn" Identifier Generic_params? 
		Fn_decl Where_clause? Inner_attributes_and_block
	;

syntax Fn_decl
	= fn_decl:Fn_params params Ret_type? type
	;

syntax Fn_decl_with_self
	= fn_decl:Fn_params_with_self Ret_type
	;

syntax Fn_decl_with_self_allow_anon_params
	= fn_decl:Fn_anon_params_with_self Ret_type
	;

syntax Fn_params
	= "(" Params? ")"
	;

syntax Fn_anon_params
	= "(" Anon_param Anon_params_allow_variadic_tail? ")"
	| "("  ")"
	;

syntax Fn_params_with_self
	= self_value: "(" ("&" Lifetime?)? "mut"? "self" Type_ascription? ("," | ("," Params ","?))? ")"
	| self_static:"(" Params? ")"
	;

syntax Fn_anon_params_with_self
	= self_value: "(" ("&" LifeTime?)? "mut"? "self" Type_ascription? ("," | ("," Anon_params ","?)) ")"
	| self_static:"(" Anon_params? ")"
	;

syntax Params
	= {Param ","}+ ","?
	;

syntax Param
	= arg:Pattern ":" Type_sum
	;

syntax Inferrable_params
	= {Inferrable_param ","}+
	;

syntax Inferrable_param
	= inferrable_param:Pattern Type_ascription?
	;

syntax Anon_params
	= Args:{Anon_param ","}+ ","?
	;

syntax Anon_param
	= arg:Named_arg ":" Type
	| Type
	;

syntax Anon_params_allow_variadic_tail
	= "," "..."
	| args:"," Anon_param Anon_params_allow_variadic_tail
	;

syntax Named_arg
	= Identifier identifier
	| pat_wild:"_"
	| "&" Identifier identifier
	| pat_wild:"&" "_"
	| "&&" Identifier identifier
	| pat_wild:"&&" "_"
	| "mut" Identifier identifier
	;

syntax Ret_type
	= "-\>" "!"
	| ret_ty:"-\>" Type
	//| Identifier /*empty*/
	;

syntax Generic_params
	= generics:"\<" Lifetime "\>"
	| generics:"\<" Lifetime "," "\>"
	| generics:"\<" Lifetime "\>\>"
	| generics:"\<" Lifetime "," "\>\>"
	| generics:"\<" Lifetime "," Type_params "\>"
	| generics:"\<" Lifetime "," Type_params "," "\>"
	| generics:"\<" Lifetime "," Type_params "\>\>"
	| generics:"\<" Lifetime "," Type_params "," "\>\>"
	| generics:"\<" Type_params "\>"
	| generics:"\<" Type_params "," "\>"
	| generics:"\<" Type_params "\>\>"
	| generics:"\<" Type_params "," "\>\>"
	;

syntax Where_clause
	= where_clause:"where" {Where_predicate ","}+ ","?
	;
syntax Where_predicate
	= where_predicate:("for" "\<" LifeTime "\>")? LifeTime ":" Bounds
	| where_predicate:("for" "\<" LifeTime "\>")? Type ":" Bounds_sequence?
	;

syntax Type_params
	= type_params:","? Type_param
	;

syntax Path_no_types_allowed
	= view_path:"::"? Identifier
	| view_path:"::"? "self"
	| Path_no_types_allowed "::" Identifier
	;

syntax Path_generic_args_without_colons
	= components:(Path_generic_args_without_colons "::")? Identifier
	| components:(Path_generic_args_without_colons "::")? Identifier Generic_args
	| components:(Path_generic_args_without_colons "::")? Identifier "(" {Type_sums ","}* ")" Ret_type
	;

syntax Generic_args
	= "\<" Generic_values "\>"
	| "\<" Generic_values "\>\>"
	| "\<" Generic_values "\>="
	| "\<" Generic_values "\>\>="
	| "\<\<" Type_qualified_path_and_generic_values "\>"
	| "\<\<" Type_qualified_path_and_generic_values "\>\>"
	| "\<\<" Type_qualified_path_and_generic_values "\>="
	| "\<\<" Type_qualified_path_and_generic_values "\>\>="
	;

syntax Generic_values
	= {Lifetimes ","}* Type_sums_and_or_bindings?
	;

syntax Type_sums_and_or_bindings
	= Type_sums ("," Bindings?)?
	| Bindings ","?
	;

/* #### #### Patterns #### #### */

syntax Pattern
	= "_"
	| "&" "mut"? Pattern
	| "&&" Pattern
	| "(" {Pattern ","}+? ")"
	| "(" {Pattern ","}+ "," ")"
	| "[" Pattern_vector? "]"
	| Literal_or_path ("..." Literal_or_path)?
	| Path_expression "{" Pattern_structure "}"
	| Path_expression "(" ".." ")"
	| Path_expression "(" {Pattern ","}+ ")"
	| Path_expression "!" Identifier? Delimited_token_trees
	| Binding_mode Identifier
	| Binding_mode? Identifier "@" Pattern
	| "box" Pattern
	| "\<" Type_sum ("as" Trait_ref)? "\>" "::" Identifier
	| "\<\<" Type_sum ("as" Trait_ref)? "\>" "::" Identifier ("as" Trait_ref)? "\>" "::" Identifier
	;

syntax Binding_mode
	= "ref" "mut"?
	| "mut"
	;

syntax Literal_or_path
	= Path_expression
	| "-"? Literal
	;

syntax Pattern_field
	= Identifier
	| Binding_mode Identifier
	| "box" Identifier
	| "box" Binding_mode Identifier
	| Identifier ":" Pattern
	| Binding_mode Identifier ":" Pattern
	;

syntax Pattern_structure
	= {Pattern_field ","}+
	| {Pattern_field ","}+ ","
	| {Pattern_field ","}+ "," ".."
	| ".."
	;

syntax Pattern_vector
	= {Pattern ","}+
	| {Pattern ","}+ ","
	| {Pattern ","}+ ".."
	| {Pattern ","}+ "," ".."
	| {Pattern ","}+ ".." "," {Pattern ","}+
	| {Pattern ","}+ ".." "," {Pattern ","}+ ","
	| {Pattern ","}+ "," ".." "," {Pattern ","}+
	| {Pattern ","}+ "," ".." "," {Pattern ","}+ ","
	| ".." "," {Pattern ","}+
	| ".." "," {Pattern ","}+ ","
	| ".."
	;

/* #### #### Types #### #### */

syntax Type 
	= Type_primitive
	| Type_closure
	| "\<" Type_sum ("as" Trait_ref)? "\>" "::" Identifier
	| "\<\<" Type_sum ("as" Trait_ref)? "\>" "::" Identifier ("as" Trait_ref)? "\>" "::" Identifier
	| "(" Type_sums ","? ")"
	| "(" ")"
	;

syntax Type_primitive
	= Path_generic_args_without_colons
	| "::" Path_Generic_args_without_colons
	| "self" "::" Path_generic_args_without_colons
	| "Box" Type
	| "*" ("mut" | "const")? Type
	| "&" Type
	| "&" "mut" Type
	| "&&" Type
	| "&&" "mut" Type
	| "&" Lifetime "mut"? Type
	| "&&" Lifetime "mut"? Type
	| "[" Type "]"
	| "[" Type "," ".." Expression"]"
	| "[" Type ";" Expression "]"
	| "typeof" "(" Expression ")"
	| "_"
	| Type_bare_fn
	| Type_proc
	| For_in_type
	;

syntax Type_bare_fn 
	= "unsafe"? "fn" Type_fn_decl
	| "unsafe"? "extern" String? "fn" Type_fn_decl
	;

syntax Type_fn_decl
	= Generic_params? Fn_anon_params Ret_type
	;

syntax Type_closure
	= "unsafe"? "|" Anon_params "|" (":" Bounds)? Ret_type
	| "unsafe"? "||" (":" Bounds)? Ret_type
	;

syntax Type_proc 
	= "proc" Generic_params? Fn_params (":" Bounds)? Ret_type
	;

syntax For_in_type
	= "for" "\<" {Lifetimes ","}* "\>" For_in_type_suffix
	;

syntax For_in_type_suffix
	= Type_proc
	| Type_bare_fn
	| Trait_ref
	| Type_closure
	;

syntax Type_qualified_path_and_generic_values
	= Type_qualified_path ("," Bindings)?
	| Type_qualified_path "," Type_sums ("," Bindings)?
	;

syntax Type_qualified_path
	= Type_sum "as" Trait_ref "\>" "::" Identifier ("+" Bounds_sequence?)?
	;

syntax Type_sums
	= {Type_sum ","}+
	;

syntax Type_sum
	= Type
	| Type "+" Bounds_sequence?
	;

syntax Type_primitive_sum
	= Type_primitive
	| Type_primitive "+" Bounds_sequence?
	;

syntax Maybe_type_param_bounds
	= ":" Bounds_sequence
	;

syntax Bounds_sequence
	= Poly_bound
	| Bound_sequences "+" Poly_bound
	;

syntax Poly_bound
	= "for" "\<" ({Lifetimes ","}+ ","?)? "\>" Bound
	| "?"? Bound
	;

syntax Bindings
	= {Binding ","}+
	;

syntax Binding
	= Identifier "=" Type
	;

syntax Type_param
	= Identifier Type_param_bounds? ("=" Type_sum)?
	| Identifier "?" Identifier Type_param_bounds? ("=" Type_sum)?
	;

syntax Bounds
	= {Bound "+"}+
	;

syntax Bound
	= Lifetime
	| Trait_ref
	;

syntax Ltbounds
	= Lifetime
	| LtBounds "+" Lifetime
	;

syntax Lifetimes
	= Lifetime_and_bounds
	| Lifetimes "," Lifetime_and_bounds
	;

syntax Lifetime_and_bounds
	= "\'" (":" Ltbounds)?
	| "\'static"
	;

syntax Lifetime
	= "\'"
	| "\'static"
	;

syntax Trait_ref
	= Path_generic_args_without_colons
	| "::" Path_generic_args_without_colons
	;

/* #### #### Blocks, Statements, and expressions #### #### */

syntax Inner_attributes_and_block
	= "{" Inner_attributes? Maybe_statements? "}"
	;

syntax Block
	= "{" Maybe_statements? "}"
	;

syntax Maybe_statements
	= Statements Nonblock_expression?
	| Nonblock_expression
	;

syntax Statements
	= Statements:Statement statements
	| Statements:Statements Statement
	;

syntax Statement
	= Let
	| Outer_attribute* "pub"? Statement_item
	| Full_block_expression
	| Block
	| Nonblock_expression? ";"
	;

syntax Expressions
	= exprs:{Expression ","}+
	;

syntax Path_expression
	= "::"? Path_generic_args_with_colons
	| self_path:"self" "::" Path_generic_args_with_colons
	;

syntax Path_generic_args_with_colons
	= components:Identifier
	| components:Path_generic_args_with_colons "::" Identifier
	| components:Path_generic_args_with_colons "::" Generic_args
	;

syntax Macro_expression
	= Path_expression "!" Identifier? Parens_delimited_token_trees
	| Path_expression "!" Identifier? Brackets_delimited_token_trees
	;

syntax Nonblock_expression
	= Literal
	> Path_expression
	| "self"
	| Macro_expression
	| Path_expression "{" Structure_expression_fields "}"
	| Nonblock_expression "." Path_generic_args_with_colons
	//| Nonblock_expression "." Literal_integer
	| Nonblock_expression "[" Expression? "]"
	| Nonblock_expression "(" Expression? ")"
	| "[" Vector_expression "]"
	| "(" Expression? ")"
	| "continue"
	| "continue" Lifetime
	| "return"
	| "return" Expression
	| "break"
	| "break" Lifetime
	> left  ( Nonblock_expression "*" Expression
			| Nonblock_expression "/" Expression
			| Nonblock_expression "%" Expression
			)
	> left  ( Nonblock_expression "+" Expression
			| Nonblock_expression "-" Expression
			> Nonblock_expression "\<\<" Expression
			| Nonblock_expression "\>\>" Expression
			> Nonblock_expression "&" Expression
			> Nonblock_expression "^" Expression
			> Nonblock_expression "|" Expression
			> Nonblock_expression "\<" Expression
			| Nonblock_expression "\>" Expression
			| Nonblock_expression "\<=" Expression
			| Nonblock_expression "\>=" Expression
			> Nonblock_expression "==" Expression
			| Nonblock_expression "!=" Expression
			> Nonblock_expression "||" Expression
			> Nonblock_expression "&&" Expression
			)
	> right Nonblock_expression "\<-" Expression
	> right ( Nonblock_expression "=" Expression
			| Nonblock_expression "\<\<=" Expression
			| Nonblock_expression "\>\>=" Expression
			| Nonblock_expression "-=" Expression
			| Nonblock_expression "&=" Expression
			| Nonblock_expression "|=" Expression
			| Nonblock_expression "+=" Expression
			| Nonblock_expression "*=" Expression
			| Nonblock_expression "/=" Expression
			| Nonblock_expression "^=" Expression
			| Nonblock_expression "%=" Expression
			)
	| Nonblock_expression ".."
	| Nonblock_expression ".." Expression
	| ".." Expression
	| ".."
	| Nonblock_expression "as" Type
	| "box" Nonparen_expression
	> "box" "(" Expression? ")" Nonblock_expression
	| Expression_qualified_path
	| Nonblock_prefix_expression
	;

syntax Expression
	= Literal
	> Path_expression
	| "self"
	| Macro_expression
	| Path_expression "{" Structure_expression_fields "}"
	| Expression "." Path_generic_args_with_colons
	//> left Expression "." Literal_integer
	| Expression "[" Expression? "]"
	| Expression "(" (Expressions ","?)? ")"
	| "(" (Expressions ","?)? ")"
	| "[" Vector_expression "]"
	| "continue"
	| "continue" Identifier
	| "return"
	| "return" Expression
	| "break"
	| "break" Identifier
	> left  ( Expression "*" Expression
			| Expression "/" Expression
			| Expression "%" Expression
			)
	> left  ( Expression "+" Expression
			| Expression "-" Expression
			> Expression "\<\<" Expression
			| Expression "\>\>" Expression
			> Expression "&" Expression
			> Expression "^" Expression
			> Expression "|" Expression
			> Expression "\<" Expression
			| Expression "\>" Expression
			| Expression "\<=" Expression
			| Expression "\>=" Expression
			> Expression "==" Expression
			| Expression "!=" Expression
			> Expression "||" Expression
			> Expression "&&" Expression
			)
	> right Expression "\<-" Expression
	> right ( Expression "=" Expression
			| Expression "\<\<=" Expression
			| Expression "\>\>=" Expression
			| Expression "-=" Expression
			| Expression "&=" Expression
			| Expression "|=" Expression
			| Expression "+=" Expression
			| Expression "*=" Expression
			| Expression "/=" Expression
			| Expression "^=" Expression
			| Expression "%=" Expression
			)
	| Expression ".."
	| Expression ".." Expression
	| ".." Expression
	| ".."
	| Expression "as" Type
	| "box" Nonparen_expression
	> "box" "(" Expression? ")" Expression
	| Expression_qualified_path
	| Block_expression
	| Block
	| Nonblock_prefix_expression
	;

syntax Nonparen_expression
	= Literal
	> Path_expression
	| "self"
	| Macro_expression
	| Path_expression "{" Structure_expression_fields "}"
	| Nonparen_expression "." Path_generic_args_with_colons
	//| Nonparen_expression "." Literal_integer
	| Nonparen_expression "[" Expression? "]"
	| Nonparen_expression "(" (Expressions ","?)? ")"
	| "[" Vector_expression "]"
	| "continue"
	| "continue" Identifier
	| "return"
	| "return" Expression
	| "break"
	| "break" Identifier
	> left  ( Nonparen_expression "*" Nonparen_expression
			| Nonparen_expression "/" Nonparen_expression
			| Nonparen_expression "%" Nonparen_expression
			)
	> left  ( Nonparen_expression "+" Nonparen_expression
			| Nonparen_expression "-" Nonparen_expression
			> Nonparen_expression "\<\<" Nonparen_expression
			| Nonparen_expression "\>\>" Nonparen_expression
			> Nonparen_expression "&" Nonparen_expression
			> Nonparen_expression "^" Nonparen_expression
			> Nonparen_expression "|" Nonparen_expression
			> Nonparen_expression "\<" Nonparen_expression
			| Nonparen_expression "\>" Nonparen_expression
			| Nonparen_expression "\<=" Nonparen_expression
			| Nonparen_expression "\>=" Nonparen_expression
			> Nonparen_expression "==" Nonparen_expression
			| Nonparen_expression "!=" Nonparen_expression
			> Nonparen_expression "||" Nonparen_expression
			> Nonparen_expression "&&" Nonparen_expression
			)
	> right Nonparen_expression "\<-" Nonparen_expression
	> right ( Nonparen_expression "=" Nonparen_expression
			| Nonparen_expression "\<\<=" Nonparen_expression
			| Nonparen_expression "\>\>=" Nonparen_expression
			| Nonparen_expression "-=" Nonparen_expression
			| Nonparen_expression "&=" Nonparen_expression
			| Nonparen_expression "|=" Nonparen_expression
			| Nonparen_expression "+=" Nonparen_expression
			| Nonparen_expression "*=" Nonparen_expression
			| Nonparen_expression "/=" Nonparen_expression
			| Nonparen_expression "^=" Nonparen_expression
			| Nonparen_expression "%=" Nonparen_expression
			)
	| Nonparen_expression ".."
	| Nonparen_expression ".." Nonparen_expression
	| ".." Nonparen_expression
	| ".."
	| Nonparen_expression "as" Type
	| "box" Nonparen_expression
	> "box" "(" Expression? ")" Expression
	| Expression_qualified_path
	| Block_expression
	| Block
	| Nonblock_prefix_expression
	;

syntax Expression_nostruct
	= Literal
	> Path_expression
	| "self"
	| Macro_expression
	| Expression_nostruct "." Path_generic_args_with_colons
	//| Expression_nostruct "." Literal_integer
	| Expression_nostruct "[" Expression? "]"
	| Expression_nostruct "(" (Expressions ","?)? ")"
	| "[" Vector_expression "]"
	| "(" Expression? ")"
	| "continue"
	| "continue" Identifier
	| "return"
	| "return" Expression
	| "break"
	| "break" Identifier
	> left  ( Expression_nostruct "*" Expression_nostruct
			| Expression_nostruct "/" Expression_nostruct
			| Expression_nostruct "%" Expression_nostruct
			)
	> left  ( Expression_nostruct "+" Expression_nostruct
			| Expression_nostruct "-" Expression_nostruct
			> Expression_nostruct "\<\<" Expression_nostruct
			| Expression_nostruct "\>\>" Expression_nostruct
			> Expression_nostruct "&" Expression_nostruct
			> Expression_nostruct "^" Expression_nostruct
			> Expression_nostruct "|" Expression_nostruct
			> Expression_nostruct "\<" Expression_nostruct
			| Expression_nostruct "\>" Expression_nostruct
			| Expression_nostruct "\<=" Expression_nostruct
			| Expression_nostruct "\>=" Expression_nostruct
			> Expression_nostruct "==" Expression_nostruct
			| Expression_nostruct "!=" Expression_nostruct
			> Expression_nostruct "||" Expression_nostruct
			> Expression_nostruct "&&" Expression_nostruct
			)
	> right Expression_nostruct "\<-" Expression_nostruct
	> right ( Expression_nostruct "=" Expression_nostruct
			| Expression_nostruct "\<\<=" Expression_nostruct
			| Expression_nostruct "\>\>=" Expression_nostruct
			| Expression_nostruct "-=" Expression_nostruct
			| Expression_nostruct "&=" Expression_nostruct
			| Expression_nostruct "|=" Expression_nostruct
			| Expression_nostruct "+=" Expression_nostruct
			| Expression_nostruct "*=" Expression_nostruct
			| Expression_nostruct "/=" Expression_nostruct
			| Expression_nostruct "^=" Expression_nostruct
			| Expression_nostruct "%=" Expression_nostruct
			)
	| Expression_nostruct ".."
	| Expression_nostruct ".." Expression_nostruct
	| ".." Expression_nostruct
	| ".."
	| Expression_nostruct "as" Type
	| "box" Expression_nostruct
	> "box" "(" Expression? ")" Expression_nostruct
	| Expression_qualified_path
	| Block_expression
	| Block
	| Nonblock_prefix_expression
	;

syntax Nonblock_prefix_expression_nostruct
	= "-" Expression_nostruct
	| "!" Expression_nostruct
	| "*" Expression_nostruct
	| "&" "mut"? Expression_nostruct
	| "&&" "mut"? Expression_nostruct
	| Lambda_expression_nostruct
	| "move" Lambda_expression_nostruct
	| Proc_expression_nostruct
	;

syntax Nonblock_prefix_expression
	= "-" Expression
	| "!" Expression
	| "*" Expression
	| "&" "mut"? Expression
	| "&&" "mut"? Expression
	| Lambda_expression
	| "move" Lambda_expression
	| Proc_expression
	;

syntax Expression_qualified_path
	= "\<" Type_sum ("as" Trait_ref)? "\>" "::" Identifier 
		("::" Generic_args)?
	| "\<\<" Type_sum ("as" Trait_ref)? "\>" "::" Identifier 
		("as" Trait_ref)? "\>" "::" Identifier
	| "\<\<" Type_sum ("as" Trait_ref)? "\>" "::" Identifier 
		Generic_args ("as" Trait_ref)? "\>" "::" Identifier
	| "\<\<" Type_sum ("as" Trait_ref)? "\>" "::" Identifier 
		("as" Trait_ref)? "\>" Identifier Generic_args
	| "\<\<" Type_sum ("as" Trait_ref)? "\>" "::" Identifier 
		Generic_args ("as" Trait_ref)? "\>" "::" Identifier Generic_args
	;

syntax Lambda_expression
	= "||" Ret_type Expression
	| "|" (("&" "mut"?)? ":")? "|" Ret_type Expression
	| "|" Inferrable_params "|" Ret_type Expression
	| "|" "&" "mut"? ":" Inferrable_params "|" Ret_type Expression
	| "|" ":" Inferrable_params "|" Ret_type Expression
	;

syntax Lambda_expression_nostruct
	= "||" Ret_type Expression_nostruct
	| "|" (("&" "mut"?)? ":")? "|" Ret_type Expression_nostruct
	| "|" Inferrable_params "|" Ret_type Expression_nostruct
	| "|" "&" "mut"? ":" Inferrable_params "|" Ret_type Expression_nostruct
	| "|" ":" Inferrable_params "|" Ret_type Expression_nostruct
	;

syntax Proc_expression
	= "proc" "(" Inferrable_params? ")" Expression
	;

syntax Proc_expression_nostruct
	= "proc" "(" Inferrable_params? ")" Expression_nostruct
	;

syntax Vector_expression
	= {Expression ","}*
	| {Expression ","}+ ";" Expression
	;

syntax Structure_expression_fields
	= Field_inits ","?
	| {Field_inits ","}* ".." Expression
	;

syntax Field_inits
	= {Field_init ","}+
	;

syntax Field_init
	= Identifier ":" Expression
	;

syntax Block_expression
	= Expression_match
	| Expression_if
	| Expression_if_let
	| Expression_while
	| Expression_while_let
	| Expression_loop
	| Expression_for
	| "unsafe" Block
	| Path_expression "!" Identifier? Braces_delimited_token_trees
	;

syntax Full_block_expression
	= Block_expression
	| Full_block_expression "." Path_generic_args_with_colons
	| Full_block_expression "." Path_generic_args_with_colons "[" Expression? "]"
	| Full_block_expression "." Path_generic_args_with_colons "(" (Expressions ","?)? ")"
	| Full_block_expression "." Literal_integer
	;

syntax Expression_match
	= "match" Expression_nostruct "{" "}"
	| "match" Expression_nostruct "{" Match_clauses "}"
	| "match" Expression_nostruct "{" Match_clauses Nonblock_match_clause "}"
	| "match" Expression_nostruct "{" Nonblock_match_clause"}"
	;

syntax Match_clauses
	= match_clause:Match_clause+ match_clauses
	;

syntax Match_clause
	= Nonblock_match_clause ","
	| Block_match_clause ","?
	;

syntax Nonblock_match_clause
	= Outer_attribute? {Pattrn "|"}+ Guard? "=\>" Nonblock_expression
	| Outer_attribute? {Pattrn "|"}+ Guard? "=\>" Full_block_expression
	;

syntax Block_match_clause
	= Outer_attribute? {Pattrn "|"}+ Guard? "=\>" Block
	;

syntax Guard
	= "if" Expression_nostruct
	;

syntax Expression_if
	= "if" Expression_nostruct Block
	| "if" Expression_nostruct Block "else" Block_or_if
	;

syntax Expression_if_let
	= "if" "let" Pattern "=" Expression_nostruct Block
	| "if" "let" Pattern "=" Expression_nostruct Block "else" Block_or_if
	;

syntax Block_or_if
	= Block
	| Expression_if
	| Expression_if_let
	;


syntax Expression_while
	= Label? "while" Expression_nostruct Block
	;

syntax Expression_while_let
	= Label? "while" "let" Pattern "=" Expression_nostruct Block
	;

syntax Expression_loop
	= Label? "loop" Block
	;

syntax Expression_for
	= Label? "for" Pattern "in" Expression_nostruct Block
	;

syntax Label
	= Lifetime ":"
	;

syntax Let
	= "let" Pattern pattern Type_ascription? type Init_expression? expression ";"
	;

/* #### #### Macros and misc. rules #### ####*/

lexical Literal
	= lit_byte:Literal_byte
	| lit_char:Literal_char
	| lit_integer:Literal_integer
	| lit_float:Literal_float
	| lit_bool:"true"
	| lit_bool:"false"
	> String
	;

lexical String
	= lit_str:Literal_string
	| lit_str:Literal_string_raw
	| lit_byte_str:Literal_byte_string
	| lit_byte_str:Literal_byte_string_raw
	;

lexical Identifier
	= ident:Ident
	;

lexical Unpaired_token
	= "\<\<"                        
	| "\>\>"                        
	| "\<="                         
	| "=="                       
	| "!="                         
	| "\>="                         
	| "&&"                     
	| "||"                       
	| "\<-"                     
	| "\<\<="                      
	| "\>\>="                      
	| "-="                    
	| "&="                      
	| "|="                       
	| "+="                     
	| "*="                     
	| "/="                    
	| "^="                    
	| "%="                  
	| ".."                     
	| "..."                  
	| "::"                    
	| "-\>"                     
	| "=\>"                  
	| Literal_byte                   
	| Literal_char                   
	| Literal_integer                
	| Literal_float                  
	| Literal_string                    
	| Literal_string_raw                
	| Literal_byte_string               
	| Literal_byte_string_raw           
	| Identifier                      
	| "_"                 
	| "\""                   
	| "self"                       
	| "static"                     
	| "as"                         
	| "break"                      
	| "crate"                      
	| "else"                       
	| "enum"                       
	| "extern"                     
	| "false"                      
	| "fn"                         
	| "for"                        
	| "if"                         
	| "impl"                       
	| "in"                         
	| "let"                        
	| "loop"                       
	| "match"                      
	| "mod"                        
	| "move"                       
	| "mut"                        
	| "priv"                       
	| "pub"                        
	| "ref"                        
	| "return"                     
	| "struct"                     
	| "true"                       
	| "trait"                      
	| "type"                       
	| "unsafe"                     
	| "use"                        
	| "while"                      
	| "continue"                   
	| "proc"                       
	| "box"                        
	| "const"                      
	| "where"                      
	| "typeof"                     
	//| Inner_doc_comment       
	//| Outer_doc_comment
	| Comment          
	| Shebang                    
	| "\'static"            
	| ";"                        
	| ","                        
	| "."                        
	| "@"                        
	| "#"                        
	| "~"                        
	| ":"                        
	| "$"                        
	| "="                        
	| "?"                        
	| "!"                        
	| "\<"                        
	| "\>"                        
	| "-"                        
	| "&"                        
	| "|"                        
	| "+"                        
	| "*"                        
	| "/"                        
	| "^"                        
	| "%"
	;

syntax Token_trees
	= token_tree:Token_tree+ token_trees
;

syntax Token_tree
	= Delimited_token_trees
	| tttok:Unpaired_token
	;

syntax Delimited_token_trees
	= Parens_delimited_token_trees
	| Braces_delimited_token_trees
	| Brackets_delimited_token_trees
	;

syntax Parens_delimited_token_trees
	= tttok:"(" Token_trees ")"
	;

syntax Braces_delimited_token_trees
	= tttok:"{" Token_trees? "}"
	;

syntax Brackets_delimited_token_trees
	= tttok:"[" Token_trees? "]"
	;
