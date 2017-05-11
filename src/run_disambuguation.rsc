module run_disambuguation

import lang::rust::\syntax::Rust;
import ParseTree;
import Forest;
import Prelude;

public void main(parse_rule, loc file_location, allow_disambiguation){
	// #start[Crate]
	// |project://oxidize/src/test/input/troublesome.rs|
	// true
	parse_tree = parse(parse_rule, file_location, allowAmbiguity=allow_disambiguation);
	drambiguation = forest(parse_tree);
	drambiguation.serve();
}
