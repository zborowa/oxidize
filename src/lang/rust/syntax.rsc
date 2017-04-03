@contributors{Adrian Zborowski - 11336544 - adrian.zborowski@uva.nl}

module lang::rust::\syntax

lexical Identifier = [a-zA-Z][a-zA-Z0-9]*;

lexical Hash = "#";
lexical Shebang = Hash "!";
