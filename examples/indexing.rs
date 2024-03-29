use ocl_include::*;
use std::path::Path;

fn main() {
    let parser = Parser::builder()
        .add_source(
            source::Fs::builder()
                .include_dir(Path::new("./examples"))
                .unwrap()
                .build(),
        )
        .build();
    let node = parser.parse(Path::new("main.c")).unwrap();
    let (generated, index) = node.collect();

    // Let's imagine that we complie the code here
    // and got a compiler message at specific line
    let line = 4;
    println!(
        "line {}: '{}'",
        line,
        generated.lines().nth(line - 1).unwrap()
    );

    // This will find the origin of this line
    let (path, local_line) = index.search(line - 1).unwrap();

    println!("origin: {:?} at line {}", path, local_line + 1);
}
