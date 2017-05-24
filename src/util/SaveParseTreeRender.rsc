module util::SaveParseTreeRender

import vis::Figure;
import vis::Render;
import vis::ParseTree;

extend ParseTree;
extend lang::rust::\syntax::Rust;

public void saveParseTreeRender(Tree pt){
	renderSave(space(visParsetree(pt),std(gap(4,15))),|project://oxidize/parsetree.png|);
}
