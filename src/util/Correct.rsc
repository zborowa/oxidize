module util::Correct

import IO;
import Map;
import Set;
import ParseTree;
import lang::rust::\syntax::Rust;

//Tree correct(Tree crate) = visit(crate){
start[Crate] correct(start[Crate] crate){

	crate = innermost visit(crate){
		case (Crate) `<Shebang_line* sl> 
					 '<Mod_item* mi>` => 
			 (Crate) `<Shebang_line* sl>
			 		 '
			 		 'extern crate mbox;
			 		 'use self::mbox::MArray;
			 		 '
					 '<Mod_item* mi>`
			when /(View_item) `extern crate mbox;` !:= mi && /(View_item) `use self::mbox::MArray;` !:= mi,
				 /(Identifier) `MArray` := mi
	};
	
	crate = innermost visit(crate){	 
		case (Crate) `<Shebang_line* sl> 
					 '<Mod_item* mi>` => 
			 (Crate) `<Shebang_line* sl>
			 		 '
			 		 'extern crate core;
					 'use self::core::nonzero::NonZero;
			 		 '
					 '<Mod_item* mi>`
			when /(View_item) `extern crate core;` !:= mi && /(View_item) `use self::core::nonzero::NonZero;` !:= mi,
				 /(Identifier) `NonZero` := mi
	};
	
	return crate;
}