use ocl_include::*;
use std::path::Path;

fn main() {
    let main = r"
    #include <header.h>
    int main() {
        return ~RET_CODE;
    }
    ";

    let hook = ListHook::builder()
        .add_hook(
            MemHook::builder()
                .add_file(&Path::new("main.c"), main.to_string()).unwrap()
                .build(),
        )
        .add_hook(
            FsHook::builder()
                .include_dir(&Path::new("./examples")).unwrap()
                .build(),
        )
        .build();

    let node = build(&hook, Path::new("main.c")).unwrap();

    println!("{}", node.collect().0);
}
