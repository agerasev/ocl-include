use std::path::Path;
use ocl_include::*;

fn main() {
    let hook = FsHook::new()
    .include_dir(&Path::new("./examples")).unwrap();

    let node = build(&hook, Path::new("main.c")).unwrap();

    println!("{}", node.collect().0);
}
