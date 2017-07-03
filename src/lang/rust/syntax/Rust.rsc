@contributors{Adrian Zborowski - ak.zborowski@gmail.com}

module lang::rust::\syntax::Rust


layout NoCurlyBefore 
  = @manual [}] !<< WhiteSpaceOrComment* !>> [\ \t\r\n] !>> "//" !>> "/*"
  ;

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
lexical Identifier
	= 	(
			[a-z A-Z 0-9 \u0080-\u00ff _] !<< 
			[a-z A-Z \u0080-\u00ff _] [a-z A-Z 0-9 \u0080-\u00ff _]* !>> 
			[a-z A-Z 0-9 \u0080-\u00ff _]
		) \ Rust_keywords
	;

lexical Literal_byte
	= "b\'" "\\" [xX] Hexit Hexit "\'"
	| "b\'" "\\" [n r t \\ \' \" 0] "\'"
	| "b\'" ![\\ \' \n \r \t] [\udc00-\udfff]? "\'"
	;

lexical Literal_char 
	= "\'" UnicodeEscape "\'"
	| "\'" ![\\ \' \n \t \r] "\'"
	| "\'" [\ud800-\udbff] [\udc00-\udfff] "\'"
	;

lexical Literal_integer
	= [0-9] [0-9 _]* /*("." ("." | [a-zA-Z]))?*/ Numeric_type?
	| "0b"  [0 1 _]+ Numeric_type?
	| "0o"  [0-8 _]+ Numeric_type?
	| "0x"  [0-9 a-f A-F _]+ Numeric_type?
	;

lexical Literal_float 
	= [0-9] [0-9 _]* "." ([0-9] [0-9 _]*)? ([eE] [\- +] [0-9] [0-9 _]*)? Numeric_type?
	;
	
lexical Numeric_type
	= "i8"
	| "i16"
	| "i32"
	| "i64"
	| "u8"
	| "u16"
	| "u32"
	| "u64"
	| "f32"
	| "f64"
	| "usize"
	| "isize"
	;

lexical Literal_string
	= "\"" StringChar* "\\"? "\""
	;

lexical StringChar
	= UnicodeEscape
	| [\a00] 
	| "\\" [\n]
	| "\\" [\r \n]
	| "\\" Hexit+
	| ![\" \\]+ !>> ![\" \\]	
	;
	
lexical Hexit
	= [0-9 a-f A-F]
	;
	
lexical UnicodeEscape
	= "\\"   [n r t \\ \' \" ?]
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
	= "r" Literal_string
	;

lexical Literal_byte_string 
	= "b" Literal_string
	;

lexical Literal_byte_string_raw
	= "br" Literal_string
	;

lexical Shebang 
	= "#!"
	;

lexical Shebang_line
	// Treat shebangs as a comment: https://github.com/rust-lang/rust/issues/1772 
	= Shebang "[" ![\]]* "]"
	;

/* #### #### Items and attributes #### #### */

start syntax Crate
	= crate: Shebang_line* Mod_item* mode_item
	;

syntax Inner_attribute
	= inner_attributes: Shebang "[" Meta_item meta_item "]"
	| inner_comment: Comment comment
	;

syntax Outer_attribute
	= outer_attributes: "#" "[" Meta_item meta_item "]"
	| outer_comment: Comment comment
	;

syntax Meta_item
	= meta_word: Identifier identifier
	| meta_name_value: Identifier identifier "=" Literal literal
	| meta_list: Identifier identifier "(" {Meta_item ","}* meta_items ","? ")"
	;

syntax Attributes_and_vis
	= AttrsAndVis: Outer_attribute* outer_attributes "pub"? visibility
	;

syntax Mod_item
	= item: Attributes_and_vis attribute_visbility Item item
	;

syntax Item 
	= Statement_item statement_item
	> Item_macro item_macro
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
	
syntax Macro_tokens 
	= ![{}()\[\]\"]
	| Macro_enclosure
	| String
	;
	
syntax Macro_braces
	= bracket "{" Macro_tokens* "}"
	;
	
syntax Macro_parens
	= bracket "(" Macro_tokens* ")"
	;
	
syntax Macro_brackets
	= bracket "[" Macro_tokens* "]"
	;

syntax Macro_enclosure
	= Macro_braces
	| Macro_parens 
	| Macro_brackets
	;

syntax Item_macro
	= macro_rules: Path_expression "!" Identifier? Macro_enclosure ";"?
	;

syntax View_item
	= Use_item
	| Extern_fn_item
	| view_item_extern_crate:"extern" "crate" Identifier identifier ("as" Identifier as)? ";"
	;

syntax Extern_fn_item 
	= view_item_extern_fn:"extern" String? abi Item_fn item_fn
	;

syntax Use_item
	= view_item_use:"use" View_path view_path ";"
	;

syntax View_path
	= view_path_simple: Path_no_types_allowed
	| view_path_list: Path_no_types_allowed? "::" "{" (Identifiers_or_self ","?)? "}"
	| view_path_glob: Path_no_types_allowed  "::" "*"
	| view_path_list_empty: "{" (Identifiers_or_self ","?)? "}"
	| view_path_simple: Path_no_types_allowed "as" Identifier
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
	= item_enum: "enum" Identifier identifier Generic_params? generic_params 
		Where_clause? where "{" {Enum_def ","}* enum_defs ","? "}"
	;

syntax Enum_def
	= enum_def: Attributes_and_vis Identifier identifier Enum_args? enum_args
	;

syntax Enum_args
	= bracket enum_args: "{" {Struct_decl_field ","}* struct_decl_fields ","? "}"
	| bracket enum_args: "(" {Type_sum ","}* type_sums ")"
	| enum_args: "=" Expression expression
	;

syntax Item_mod
	= item_mod:"mod" Identifier identifier ";"
	| item_mod:"mod" Identifier identifier "{" Inner_attribute* inner_attributes Mod_item* mode_items"}"
	;

syntax Item_foreign_mod
	= item_foreign_mod:"extern" String? name "{" Inner_attribute* Foreign_item* "}"
	;

syntax Foreign_item
	= foreign_item: Attributes_and_vis "static" "mut"? Identifier identifier 
		":" Type type ";"
	| foreign_item: Attributes_and_vis "unsafe"? "fn" Identifier identifier 
		Generic_params? generic_params "(" Params? params "..."? ")" Ret_type? Where_clause? where ";"
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
		Identifier Generic_params? Fn_decl_with_self_allow_anon_params Where_clause? Inner_attributes_and_block
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
		Fn_decl_with_self_allow_anon_params Where_clause? Inner_attributes_and_block
	;

syntax Impl_method
	= method: Attributes_and_vis "unsafe"? ("extern" String?)? "fn" Identifier Generic_params? 
		Fn_decl_with_self Where_clause? Inner_attributes_and_block
	;
  
syntax Item_impl
	= item_impl: "unsafe"? "impl" Generic_params? 
		Type_primitive_sum Where_clause? "{" Inner_attribute* Impl_item* "}"
	| item_impl: "unsafe"? "impl" Generic_params? 
		"(" Type ")" Where_clause? "{" Inner_attribute* Impl_item* "}"
	| item_impl: "unsafe"? "impl" Generic_params? 
		Trait_ref "for" Type_sum Where_clause? "{" Inner_attribute* Impl_item* "}"
	| item_impl_neg: "unsafe"? "impl" Generic_params? 
		"!" Trait_ref "for" Type_sum Where_clause? "{" Inner_attribute* Impl_item* "}"
	| item_impl_default: "unsafe"? "impl" Generic_params? 
		Trait_ref "for" ".." "{" "}"
	| item_impl_default_neg: "unsafe"? "impl" Generic_params? 
		"!" Trait_ref "for" ".." "{" "}"
	;

syntax Impl_item
	= impl_method: Impl_method
	| impl_macro_item: Attributes_and_vis Item_macro
	| impl_const: Attributes_and_vis Item_const
	| impl_type: Attributes_and_vis "type" Identifier Generic_params? "=" Type_sum ";"
	;

syntax Item_fn
	= item_fn:"fn" Identifier identifier Generic_params? generic_params 
		Fn_decl Where_clause? Inner_attributes_and_block
	;

syntax Item_unsafe_fn
	= item_unsafe_fn:"unsafe" ("extern" String?)? "fn" Identifier Generic_params? 
		Fn_decl Where_clause? Inner_attributes_and_block
	;

syntax Fn_decl
	= fn_decl:Fn_params params Ret_type? type
	;

syntax Fn_decl_with_self
	= fn_decl:Fn_params_with_self Ret_type?
	;

syntax Fn_decl_with_self_allow_anon_params
	= fn_decl:Fn_anon_params_with_self Ret_type?
	;

syntax Fn_params
	= "(" Params? ")"
	;

syntax Fn_anon_params
	= "(" Anon_params_allow_variadic_tail ")"
	;

syntax Fn_params_with_self
	= self_value: "(" ("&" Lifetime?)? "mut"? "self" Type_ascription? ("," | ("," Params ","?))? ")"
	| self_static:"(" Params? ")"
	;

syntax Fn_anon_params_with_self
	= self_value: "(" ("&" Lifetime?)? "mut"? "self" Type_ascription? ("," | ("," Anon_params ","?))? ")"
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
	= args: {Anon_param ","}* ("," "...")?
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
	= ret_ty: "-\>" Type type
	;

syntax Generic_params
	= generics: "\<" Lifetimes ","? "\>"
	| generics: "\<" Lifetimes ","? "\>\>"
	| generics: "\<" Lifetimes "," Type_params ","? "\>"
	| generics: "\<" Lifetimes "," Type_params ","? "\>\>"
	| generics: "\<" Type_params ","? "\>"
	| generics: "\<" Type_params ","? "\>\>"
	;

syntax Where_clause
	= where_clause:"where" {Where_predicate ","}+ ","?
	;
	
syntax Where_predicate
	= where_predicate:("for" "\<" Lifetime "\>")? Lifetime ":" Bounds
	| where_predicate:("for" "\<" Lifetime "\>")? Type ":" Bounds_sequence?
	;

syntax Type_params
	= type_params: {Type_param ","}+
	;

syntax Path_no_types_allowed
	= view_path:"::"? Identifier
	| view_path:"::"? "self"
	| view_path:"::"? "super"
	| Path_no_types_allowed "::" Identifier
	;

syntax Path_generic_args_without_colons
	= components:(Path_generic_args_without_colons "::")? Identifier Generic_args?
	| components:(Path_generic_args_without_colons "::")? Identifier "(" {Type_sums ","}* ")" Ret_type?
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
	= {Lifetimes ","}* ","? Type_sums_and_or_bindings?
	;

syntax Type_sums_and_or_bindings
	= Type_sums ("," Bindings?)?
	| Bindings ","?
	;

/* #### #### Patterns #### #### */

syntax Pattern
	= //"_"
	 "&" "mut"? Pattern
	| "&&" Pattern
	| "(" {Pattern ","}+? ")"
	| "(" {Pattern ","}+? "," ")"
	| "[" Pattern_vector? "]"
	| Literal_or_path ("..." Literal_or_path)?
	| Path_expression "{" Pattern_structure "}"
	| Path_expression "(" ".." ")"
	| Path_expression "(" {Pattern ","}+? ")"
	| Item_macro
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
	= "box"? Binding_mode? Identifier
	| Binding_mode? Identifier ":" Pattern
	;

syntax Pattern_structure
	= {Pattern_field ","}+ ("," ".."?)?
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
	= "Self" ("::" Path_generic_args_without_colons)?
	| ("self"? "::")? Path_generic_args_without_colons
	| "Box" Type
	| "*" ("mut" | "const")? Type
	| "&" "mut"? Type
	| "&&" "mut"? Type
	| "&" Lifetime "mut"? Type
	| "&&" Lifetime "mut"? Type
	| "[" Type "]"
	| "[" Type "," ".." Expression"]"
	| "[" Type ";" Expression "]"
	| "typeof" "(" Expression ")"
	//| "_"
	// This is an experimental value ("!") only usable on Nightly
	| "!"
	//| Numeric_type
	| Type_bare_fn
	| Type_proc
	| For_in_type
	;

syntax Type_bare_fn 
	= "unsafe"? "fn" Type_fn_decl
	| "unsafe"? "extern" String? "fn" Type_fn_decl
	;

syntax Type_fn_decl
	= Generic_params? Fn_anon_params Ret_type?
	;

syntax Type_closure
	= "unsafe"? "|" Anon_params? "|" (":" Bounds)? Ret_type?
	//| "unsafe"? "||" (":" Bounds)? Ret_type
	;

syntax Type_proc 
	= "proc" Generic_params? Fn_params (":" Bounds)? Ret_type?
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
	= Outer_attribute* outer_attrs Type type ("+" Bounds_sequence?)?
	;

syntax Type_primitive_sum
	= Type_primitive
	| Type_primitive "+" Bounds_sequence?
	;

syntax Type_param_bounds
	= ":" Bounds_sequence
	;

syntax Bounds_sequence
	= Poly_bound
	| Bounds_sequence "+" Poly_bound
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
	= ("#" "[" Meta_item meta_item "]")? Identifier identifier ("?" Identifier)? Type_param_bounds? ("=" Type_sum)?
	;

syntax Bounds
	//= {Bound "+"}+
	= Bound
	| Bounds "+" Bound
	;

syntax Bound
	= Lifetime
	| Trait_ref
	;

syntax Ltbounds
	= {Lifetime "+"}+
	;

syntax Lifetimes
	= {Lifetime_and_bounds ","}+
	;

syntax Lifetime_and_bounds
	= lifetime_bounds: "\'" Identifier identifier (":" Ltbounds bounds)?
	| static_lifetime: "\'static"
	;

lexical Lifetime
	= lifetime: "\'" Identifier identifier
	| static_lifetime: "\'static"
	;

syntax Trait_ref
	= Path_generic_args_without_colons
	| "::" Path_generic_args_without_colons
	;

/* #### #### Blocks, Statements, and expressions #### #### */

syntax Inner_attributes_and_block
	= bracket "{" Inner_attribute* ia Statements stmts "}"
	;

syntax Block
	= bracket "{" Statements stmts "}"
	;

syntax Statements
	= Statement* stmts Expression!blockExpr!blockStmt? expr
	;

syntax Statement
	= Let
	| Outer_attribute* "pub"? Statement_item
	| Full_block_expression
	| Block
	| Expression!blockExpr!blockStmt? ";"
	;

syntax Expressions
	= exprs:{Expression ","}+
	;

syntax Path_expression
	= "::"? Path_generic_args_with_colons
	| self_path:"self" "::" Path_generic_args_with_colons
	| super_path:"super" "::" Path_generic_args_with_colons
	;

syntax Path_generic_args_with_colons
	= components:Identifier
	| components:Path_generic_args_with_colons "::" Identifier
	| components:Path_generic_args_with_colons "::" Generic_args
	;

syntax Macro_expression
	= Path_expression "!" Identifier? Macro_parens
	| Path_expression "!" Identifier? Macro_brackets
	;

syntax Expression
	= exExpr: "!" Expression
	> lit: Literal
	> pathExpr: Path_expression
	| self: "self"
	| macroExpr: Macro_expression
	| pathStruct: Path_expression "{" Struct_expression_fields? "}"
	> left exprPath: Expression "." Path_generic_args_with_colons
	> left exprInt: Expression "." Literal_integer
	| vecExprs: Expression "[" Expression? "]"
	> parenExprs: Expression!returnExpr NoCurlyBefore "(" (Expressions ","?)? ")"
	| parenExpr: "(" (Expressions ","?)? ")"
	> vecExpr: "[" Vector_expression "]"
	> contnIdent: "continue" Lifetime?
	| returnExpr: "return" Expression?
	| breakIdent: "break" Lifetime?
	
	> starExpr: "*" Expression
	| left  ( Expression NoCurlyBefore "*" Expression
			| Expression "/" Expression
			| Expression "%" Expression)
			
	> minExpr: "-" Expression
	| left  ( Expression "+" Expression
			| Expression!returnExpr "-" Expression)
			
	> left  ( Expression "\<\<" Expression
            | Expression "\>\>" Expression)
            
    > andExpr: "&" !>> "&" "mut"? Expression
	| left    Expression NoCurlyBefore "&" !>> "&" Expression // & not followed by &
	> left 	  Expression "^" Expression
	> left    Expression "|" Expression	
	> left  ( Expression "\<" Expression
	        | Expression "\>" Expression
			| Expression "\<=" Expression
			| Expression "\>=" Expression)
	> left  ( exprEqEq: Expression "==" Expression
			| exprNe: Expression "!=" Expression)
			
	> andandExpr: "&&" "mut"? Expression
	| left exprAndAnd: Expression NoCurlyBefore "&&" Expression
	> left exprOrOr: Expression NoCurlyBefore"||" Expression
	
	> right   Expression "\<-" Expression
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
			| Expression "%=" Expression)
			
	> left Expression ".." Expression
	| Expression ".."
	| ".." Expression
	| ".."
	
	> right asType: Expression!returnExpr "as" Type
	
	> "box" "(" Expression? ")" Expression
	> "box" Expression!parenExpr
	> Expression_qualified_path
	| blockExpr: Block_expression
	| blockStmt: Block

	> lambda: "move"? "|" (("&" "mut"?)? ":")? Inferrable_params? "|" Ret_type? Expression
	> procExpr: "proc" "(" Inferrable_params? ")" Expression
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

syntax Vector_expression
	= (Expressions ","?)?
	| Expressions ";" Expression
	;

syntax Struct_expression_fields
	= {Field_init ","}+ ","?
	| {Field_init ","}* ","? ".." Expression
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
	| Path_expression "!" Identifier? Macro_braces
	;

syntax Full_block_expression
	= Block_expression
	| Full_block_expression "." Path_generic_args_with_colons
	| Full_block_expression "." Path_generic_args_with_colons "[" Expression? "]"
	| Full_block_expression "." Path_generic_args_with_colons "(" (Expressions ","?)? ")"
	| Full_block_expression "." Literal_integer
	;

syntax Expression_match
	= "match" Expression!pathStruct "{" Match_clauses? Nonblock_match_clause? "}"
	;

syntax Match_clauses
	= match_clause:Match_clause+? match_clauses
	;

syntax Match_clause
	= Nonblock_match_clause ","
	| Block_match_clause ","?
	;

syntax Nonblock_match_clause
	= Outer_attribute* {Pattern "|"}+ Guard? "=\>" Expression!blockExpr!blockStmt
	| Outer_attribute* {Pattern "|"}+ Guard? "=\>" Full_block_expression
	;

syntax Block_match_clause
	= Outer_attribute* {Pattern "|"}+ Guard? "=\>" Block
	;

syntax Guard
	= "if" Expression!pathStruct
	;

syntax Expression_if
	= "if" Expression!pathStruct Block
	| "if" Expression!pathStruct Block "else" Block_or_if
	;

syntax Expression_if_let
	= "if" "let" Pattern "=" Expression!pathStruct Block
	| "if" "let" Pattern "=" Expression!pathStruct Block "else" Block_or_if
	;

syntax Block_or_if
	= Block
	| Expression_if
	| Expression_if_let
	;


syntax Expression_while
	= (Lifetime ":")? "while" Expression!pathStruct Block
	;

syntax Expression_while_let
	= (Lifetime ":")? "while" "let" Pattern "=" Expression!pathStruct Block
	;

syntax Expression_loop
	= (Lifetime ":")? "loop" Block
	;

syntax Expression_for
	= (Lifetime ":")? "for" Pattern "in" Expression!pathStruct Block
	;

syntax Let
	= "let" Pattern pattern Type_ascription? type ("=" Expression expression)? ";"
	;

/* #### #### Macros and misc. rules #### ####*/

lexical Literal
	= lit_byte: Literal_byte lit_byte
	| lit_char: Literal_char lit_char
	| lit_integer: Literal_integer lit_integer
	| lit_float: Literal_float lit_float
	| lit_bool_true: "true" lit_bool_true
	| lit_bool_false: "false" lit_bool_false
	> lit_str: String lit_str
	;

lexical String
	= lit_str: Literal_string lit_str
	| lit_str_raw: Literal_string_raw lit_str_raw
	| lit_byte_str: Literal_byte_string lit_byte_str
	| lit_byte_str_raw: Literal_byte_string_raw lit_byte_str_raw
	;
