module \test::func::LoadFile

import Prelude;

str FileExtension(str file_name){
	if(!endsWith(file_name, ".rs")); file_name += ".rs";
	return file_name;
}

str LoadFile(str input_file_name){
	str file_name = FileExtension(input_file_name);
	str file_contents;
	
	file_contents = readFile(|project://oxidize/src/test/input/| + file_name);
		
	return file_contents;
}
