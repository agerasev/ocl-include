use std::path::Path;

use indoc::indoc;

use crate::*;

#[test]
fn main_only() {
    let main = indoc!("
        int main() {
            return RET_CODE;
        }
    ");

    let hook = MemHook::builder()
    .add_file(&Path::new("main.c"), main.to_string()).unwrap()
    .build();

    let node = build(&hook, Path::new("main.c")).unwrap();

    assert_eq!(node.collect().0, main);
}

#[test]
fn single_header() {
    let main = indoc!("
        #include <header.h>
        #include <header.h>
        // Main function
        int main() {
            return RET_CODE;
        }
    ");
    let header = indoc!("
        #pragma once
        // Return code
        static const int RET_CODE = 0;
    ");
    let result = indoc!("


        // Return code
        static const int RET_CODE = 0;

        // Main function
        int main() {
            return RET_CODE;
        }
    ");

    let hook = MemHook::builder()
    .add_file(&Path::new("main.c"), main.to_string()).unwrap()
    .add_file(&Path::new("header.h"), header.to_string()).unwrap()
    .build();

    let node = build(&hook, Path::new("main.c")).unwrap();

    assert_eq!(node.collect().0, result);
}

#[test]
#[should_panic]
fn recursion() {
    let first = indoc!("
        #include <second.h>
    ");
    let second = indoc!("
        #include <first.h>
    ");

    let hook = MemHook::builder()
    .add_file(&Path::new("first.h"), first.to_string()).unwrap()
    .add_file(&Path::new("second.h"), second.to_string()).unwrap()
    .build();

    build(&hook, Path::new("first.h")).unwrap();
}

#[test]
fn recursion_prevented() {
    let first = indoc!("
        #pragma once
        #include <second.h>
    ");
    let second = indoc!("
        #pragma once
        #include <first.h>
    ");

    let hook = MemHook::builder()
    .add_file(&Path::new("first.h"), first.to_string()).unwrap()
    .add_file(&Path::new("second.h"), second.to_string()).unwrap()
    .build();

    let node = build(&hook, Path::new("first.h")).unwrap();

    assert_eq!(node.collect().0, "\n\n\n\n");
}

#[test]
fn multiple_headers() {
    let main = indoc!("
        #include <h02.h>
        #include <h01.h>
    ");
    let h01 = indoc!("
        #pragma once
        #include <h02.h>
        h01
    ");
    let h02 = indoc!("
        #pragma once
        #include <h01.h>
        h02
    ");

    let hook = MemHook::builder()
    .add_file(&Path::new("main.c"), main.to_string()).unwrap()
    .add_file(&Path::new("h01.h"), h01.to_string()).unwrap()
    .add_file(&Path::new("h02.h"), h02.to_string()).unwrap()
    .build();

    let node = build(&hook, Path::new("main.c")).unwrap();

    assert_eq!(node.collect().0, "\n\n\n\n\nh01\nh02\n\n");
}

#[test]
fn line_numbers() {
    let main = indoc!("
        0
        1
        2
        #include <h01.h>
        9
        10
        #include <h03.h>
        15
        16
    ");
    let h01 = indoc!("
        4
        #include <h02.h>
        8
    ");
    let h02 = indoc!("
        6
        7
    ");
    let h03 = indoc!("
        12
        13
        14
    ");

    let hook = MemHook::builder()
    .add_file(&Path::new("main.c"), main.to_string()).unwrap()
    .add_file(&Path::new("h01.h"), h01.to_string()).unwrap()
    .add_file(&Path::new("h02.h"), h02.to_string()).unwrap()
    .add_file(&Path::new("h03.h"), h03.to_string()).unwrap()
    .build();

    let node = build(&hook, Path::new("main.c")).unwrap();
    
    let source = node.collect().0;
    for (pos, line) in source.lines().enumerate() {
        let tline = line.trim_end();
        if !tline.is_empty() {
            assert_eq!(tline.parse::<usize>().unwrap(), pos);
        }
    }
}

#[test]
fn indexing() {
    let main = indoc!("
        00
        01
        02
        #include <h01.h>
        04
        05
        #include <h03.h>
        07
        08
    ");
    let h01 = indoc!("
        10
        #include <h02.h>
        12
    ");
    let h02 = indoc!("
        20
        21
    ");
    let h03 = indoc!("
        30
        31
        32
    ");

    let hook = MemHook::builder()
    .add_file(&Path::new("main.c"), main.to_string()).unwrap()
    .add_file(&Path::new("h01.h"), h01.to_string()).unwrap()
    .add_file(&Path::new("h02.h"), h02.to_string()).unwrap()
    .add_file(&Path::new("h03.h"), h03.to_string()).unwrap()
    .build();

    let node = build(&hook, Path::new("main.c")).unwrap();
    
    let (source, index) = node.collect();
    for (pos, line) in source.lines().enumerate() {
        let (name, lpos) = index.search(pos).unwrap();
        let tline = line.trim_end();
        if !tline.is_empty() {
            let n = tline.parse::<usize>().unwrap();
            let (f, l) = (n/10, n%10);
            assert_eq!(match f {
                0 => "main.c".to_string(),
                x => format!("h0{}.h", x),
            }, name.to_string_lossy());
            assert_eq!(l, lpos);
        }
    }
}
