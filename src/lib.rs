mod hook;
mod node;
mod collector;

pub use hook::*;
pub use collector::*;

#[cfg(test)]
mod test {
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

        let hook = MemHook::new()
        .add_file(&Path::new("main.c"), main.to_string()).unwrap();

        let node = collect(&hook, Path::new("main.c")).unwrap();

        assert_eq!(node.collect(), main);
    }
    
    #[test]
    fn single_header() {
        let main = indoc!("
            #include <header.h>
            #include <header.h>
            int main() {
                return RET_CODE;
            }
        ");
        let header = indoc!("
            #pragma once
            static const int RET_CODE = 0;
        ");
        let result = indoc!("


            static const int RET_CODE = 0;

            int main() {
                return RET_CODE;
            }
        ");

        let hook = MemHook::new()
        .add_file(&Path::new("main.c"), main.to_string()).unwrap()
        .add_file(&Path::new("header.h"), header.to_string()).unwrap();

        let node = collect(&hook, Path::new("main.c")).unwrap();

        assert_eq!(node.collect(), result);
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

        let hook = MemHook::new()
        .add_file(&Path::new("first.h"), first.to_string()).unwrap()
        .add_file(&Path::new("second.h"), second.to_string()).unwrap();

        collect(&hook, Path::new("first.h")).unwrap();
    }

    #[test]
    #[should_panic]
    fn recursion_prevented() {
        let first = indoc!("
            #pragma once
            #include <second.h>
        ");
        let second = indoc!("
            #pragma once
            #include <first.h>
        ");

        let hook = MemHook::new()
        .add_file(&Path::new("first.h"), first.to_string()).unwrap()
        .add_file(&Path::new("second.h"), second.to_string()).unwrap();

        let node = collect(&hook, Path::new("first.h")).unwrap();

        assert_eq!(node.collect(), "/n/n/n/n");
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

        let hook = MemHook::new()
        .add_file(&Path::new("main.c"), main.to_string()).unwrap()
        .add_file(&Path::new("h01.h"), h01.to_string()).unwrap()
        .add_file(&Path::new("h02.h"), h02.to_string()).unwrap();

        let node = collect(&hook, Path::new("main.c")).unwrap();

        assert_eq!(node.collect(), "\n\n\n\n\nh01\nh02\n\n");
    }
}