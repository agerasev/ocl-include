use ocl_include::*;
use uni_path::Path;

fn main() {
    let main = r"
    #include <header.h>
    int main() {
        return ~RET_CODE;
    }
    ";

    let parser = Parser::builder()
        .add_source(
            source::Mem::builder()
                .add_file(&Path::new("main.c"), main.to_string())
                .unwrap()
                .build(),
        )
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
