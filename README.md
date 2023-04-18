# ocl-include

[![Crates.io][crates_badge]][crates]
[![Docs.rs][docs_badge]][docs]
[![Travis CI][travis_badge]][travis]
[![Appveyor][appveyor_badge]][appveyor]
[![Codecov.io][codecov_badge]][codecov]
[![License][license_badge]][license]

[crates_badge]: https://img.shields.io/crates/v/ocl-include.svg
[docs_badge]: https://docs.rs/ocl-include/badge.svg
[travis_badge]: https://api.travis-ci.org/agerasev/ocl-include.svg
[appveyor_badge]: https://ci.appveyor.com/api/projects/status/github/agerasev/ocl-include?branch=master&svg=true
[codecov_badge]: https://codecov.io/gh/agerasev/ocl-include/graphs/badge.svg
[license_badge]: https://img.shields.io/crates/l/ocl-include.svg

[crates]: https://crates.io/crates/ocl-include
[docs]: https://docs.rs/ocl-include
[travis]: https://travis-ci.org/agerasev/ocl-include
[appveyor]: https://ci.appveyor.com/project/agerasev/ocl-include
[codecov]: https://codecov.io/gh/agerasev/ocl-include
[license]: #license

Simple preprocessor that implements #include mechanism for OpenCL source files.

## About

OpenCL API doesn't provide mechanism for including header files into the main one, like in C and C++. This crate is a simple preprocessor that handles `#include ...` and `#pragma once` directives in source files, collects them over filesystem or memory, and gives a single string to the output that could be passed to OpenCL kernel builder. Also it provides mechanism to find the source file and location in it by line number in resulting string, that is helpful for OpenCL compiler messages handling.

## [Documentation](https://docs.rs/ocl-include)

## Examples

Let you have `main.c` and `header.h` files in `./examples/` folder:

`main.c`:
```c
#include <header.h>

int main() {
    return RET_CODE;
}
```

`header.h`:
```c
#pragma once

static const int RET_CODE = 0;
```

### Filesystem only

The follwong code takes `main.c` from the filesystem and includes `header.h` into it.

```rust
use ocl_include::*;
use std::path::Path;

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
```

### Filesystem and memory

The follwong code takes `main.c` source from the memory and includes `header.h` into it from the filesystem.

```rust
use ocl_include::*;
use std::path::Path;

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
```

### Indexing

`Node.collect()` also returns `Index` instance as seconds value. It could be used to find the source file and line number in it by line number in generated string.

Let's imagine that our OpenCL compiler takes generated string and fails at some line. But line number isn't helpful for us because we don't know in which source file this line originate. Fortunately there is `Index::search` for this.

```rust
use ocl_include::*;
use std::path::Path;

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

    println!("origin: '{}' at line {}", path, local_line + 1);
}
```

## Other preprocessing functionality 

The parser also supports filtering code with simple preprocessor gates (`#if(n)def`, `#else`, `#endif`).

By default the filtration is disabled, to enable it for specific definitions use `ParserBuilder::add_flag(flag_name, is_defined)`.

## Sources

Source is a handler that retrieves files by their names.

The crate contains the following sources now: 

+ `Fs`: takes files from the filesystem.
+ `Mem`: retrieves the source from the memory.

Also the following compositions are also sources:

+ `Vec<S> where S: Source`: Tries to retrieve file from sources subsequently.
+ `&S where S: Source`
+ `Box<dyn Source>`
+ `Rc<S> where S: Source`

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
