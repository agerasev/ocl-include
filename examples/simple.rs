use ocl_include::*;
use uni_path::Path;

fn main() {
    let parser = Parser::builder()
        .add_source(
            source::Fs::builder()
                .include_dir(&Path::new("./examples"))
                .unwrap()
                .build(),
        )
        .build();
    let node = parser.parse(Path::new("main.c")).unwrap();

    println!("{}", node.collect().0);
}
