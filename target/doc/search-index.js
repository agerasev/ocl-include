var N=null,E="",T="t",U="u",searchIndex={};
var R=["ocl_include","result","option","try_from","borrow","type_id","typeid","borrow_mut","try_into","FsHook","ListHook"];

searchIndex[R[0]]={"doc":E,"i":[[3,R[9],R[0],"Hook for reading files from filesystem",N,N],[3,"MemHook",E,"Hook for retrieving files from memory",N,N],[3,R[10],E,"Hook for retrieving files from list of other hooks…",N,N],[3,"Index",E,"Index that maps generated code locations to their origins",N,N],[3,"Node",E,"Tree of parsed source files",N,N],[5,"build",E,"Reads and parses source files and resolves dependencies",N,[[["hook"],["path"]],[R[1],["node"]]]],[11,"new",E,E,0,[[],["self"]]],[11,"include_dir",E,E,0,[[["self"],["path"]],[R[1]]]],[11,"new",E,E,1,[[],["self"]]],[11,"add_file",E,E,1,[[["self"],["path"],["string"]],[R[1]]]],[11,"new",E,E,2,[[],["self"]]],[11,"add_hook",E,E,2,[[["self"],[T]],["self"]]],[11,"search",E,"Maps line number in generated code to source file name and…",3,[[["self"],["usize"]],[R[2]]]],[11,"name",E,E,4,[[["self"]],["path"]]],[11,"lines_count",E,E,4,[[["self"]],["usize"]]],[11,"collect",E,"Generates resulting code string and mapping index for it",4,N],[8,"Hook",E,"Something that may provide file content by its name",N,N],[10,"read",E,"Performs file loading",5,[[["self"],["path"],[R[2],["path"]]],[R[1]]]],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[["self"]],[U]]],[11,R[3],E,E,0,[[[U]],[R[1]]]],[11,R[4],E,E,0,[[["self"]],[T]]],[11,R[5],E,E,0,[[["self"]],[R[6]]]],[11,R[7],E,E,0,[[["self"]],[T]]],[11,R[8],E,E,0,[[["self"]],[R[1]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,"into",E,E,1,[[["self"]],[U]]],[11,R[3],E,E,1,[[[U]],[R[1]]]],[11,R[4],E,E,1,[[["self"]],[T]]],[11,R[5],E,E,1,[[["self"]],[R[6]]]],[11,R[7],E,E,1,[[["self"]],[T]]],[11,R[8],E,E,1,[[["self"]],[R[1]]]],[11,"from",E,E,2,[[[T]],[T]]],[11,"into",E,E,2,[[["self"]],[U]]],[11,R[3],E,E,2,[[[U]],[R[1]]]],[11,R[4],E,E,2,[[["self"]],[T]]],[11,R[5],E,E,2,[[["self"]],[R[6]]]],[11,R[7],E,E,2,[[["self"]],[T]]],[11,R[8],E,E,2,[[["self"]],[R[1]]]],[11,"from",E,E,3,[[[T]],[T]]],[11,"into",E,E,3,[[["self"]],[U]]],[11,R[3],E,E,3,[[[U]],[R[1]]]],[11,R[4],E,E,3,[[["self"]],[T]]],[11,R[5],E,E,3,[[["self"]],[R[6]]]],[11,R[7],E,E,3,[[["self"]],[T]]],[11,R[8],E,E,3,[[["self"]],[R[1]]]],[11,"from",E,E,4,[[[T]],[T]]],[11,"into",E,E,4,[[["self"]],[U]]],[11,R[3],E,E,4,[[[U]],[R[1]]]],[11,R[4],E,E,4,[[["self"]],[T]]],[11,R[5],E,E,4,[[["self"]],[R[6]]]],[11,R[7],E,E,4,[[["self"]],[T]]],[11,R[8],E,E,4,[[["self"]],[R[1]]]],[11,"read",E,E,0,[[["self"],["path"],[R[2],["path"]]],[R[1]]]],[11,"read",E,E,1,[[["self"],["path"],[R[2],["path"]]],[R[1]]]],[11,"read",E,E,2,[[["self"],["path"],[R[2],["path"]]],[R[1]]]]],"p":[[3,R[9]],[3,"MemHook"],[3,R[10]],[3,"Index"],[3,"Node"],[8,"Hook"]]};
initSearch(searchIndex);addSearchOptions(searchIndex);