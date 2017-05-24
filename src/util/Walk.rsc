@contributors{Adrian Zborowski - ak.zborowski@gmail.com}

module util::Walk

import IO;
import String;

@doc{
.Synopsis
Traverse the directories present in the given project path.

.Usage
Walk(|<Uri>|, extension);

.Description
Traversal function for the given project path. It recursively checks if an entry is a directory and traverser into it 
to find files.
}
public list[loc] Walk(loc project_path, str extension){
	list[loc] files = [];

	for (entry <- listEntries(project_path)){
		if(endsWith(entry, extension)){
			files += (project_path + entry);
		}elseif(isDirectory(project_path + entry)){
			files += Walk(project_path + entry, extension);
		}
	}
			
	return files;
}
