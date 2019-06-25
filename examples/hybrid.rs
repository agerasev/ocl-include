use std::path::Path;
use ocl_include::*;

fn main() {
    let main = r"
    #include <header.h>
    int main() {
        return ~RET_CODE;
    }
    ";

    let hook = ListHook::new()
    .add_hook(
        MemHook::new()
        .add_file(&Path::new("main.c"), main.to_string()).unwrap()
    )
    .add_hook(
        FsHook::new()
        .include_dir(&Path::new("./examples")).unwrap()
    );

    let node = build(&hook, Path::new("main.c")).unwrap();

    println!("{}", node.collect().0);
}
