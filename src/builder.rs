use std::{
    io,
    env,
    path::{Path, PathBuf},
    collections::hash_map::{HashMap, Entry},
};

use regex::{Regex, RegexBuilder};

use lazy_static::lazy_static;

use crate::{
    node::{Node},
    hook::{Hook},
};


lazy_static!{
    static ref INCLUDE: Regex = RegexBuilder::new(
        r#"^\s*#include\s*(.)(.*)(.)\s*$"#
    ).multi_line(true).build().unwrap();
    static ref PRAGMA_ONCE: Regex = RegexBuilder::new(
        r#"^\s*#pragma\s+once\s*$"#
    ).multi_line(true).build().unwrap();
}

enum ParseLine<'a> {
    Text(&'a str),
    Node(Node),
    Break,
    Err(io::Error),
}

struct CacheEntry {
    occured: usize,
}
impl CacheEntry {
    fn new() -> Self {
        Self { occured: 1 }
    }
}

struct Builder<'a> {
    hook: &'a dyn Hook,
    cache: HashMap<PathBuf, CacheEntry>,
    stack: Vec<PathBuf>,
}

impl<'a> Builder<'a> {
    fn new(hook: &'a dyn Hook) -> Self {
        Self {
            hook,
            cache: HashMap::new(),
            stack: Vec::new(),
        }
    }

    fn read(&mut self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        self.hook.read(path, dir)
        .map(|(path, text)| {
            match self.cache.entry(path.clone()) {
                Entry::Occupied(mut v) => { v.get_mut().occured += 1; },
                Entry::Vacant(v) => { v.insert(CacheEntry::new()); },
            }
            (path, text)
        })
    }

    fn build(&mut self, path: &Path, dir: Option<&Path>) -> io::Result<Option<Node>> {
        self.read(path, dir)
        .and_then(|(path, text)| {
            if self.stack.iter().filter(|p| **p == path).count() >= 2 {
                Err(io::Error::new(io::ErrorKind::InvalidData, "recursion found"))
            } else {
                self.stack.push(path.clone());
                Ok((path, text))
            }
        })
        .and_then(|(path, text)| self.parse(&path, text).map(|x| (x, path)))
        .map(|(x, path)| {
            assert_eq!(self.stack.pop().unwrap(), path);
            x
        })
    }

    

    fn parse_line<'b>(&mut self, path: &Path, line: &'b str, node: &Node) -> ParseLine<'b> {
        if let Some(cap) = INCLUDE.captures(line) {
            let inc_path = Path::new(&cap[2]);
            let (lb, rb) = (&cap[1], &cap[3]);
            let inc_res = {
                if lb == "<" && rb == ">" {
                    Ok(None)
                } else if lb == "\"" && rb == "\"" {
                    Ok(Some(path.parent().unwrap().to_path_buf()))
                } else {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "bad #include syntax"))
                }
                .and_then(|dir_opt| self.build(inc_path, dir_opt.as_deref()))
                .map_err(|err| {
                    io::Error::new(err.kind(), format!(
                        "{}\nin file '{}' at line {}",
                        err, path.display(), node.lines_count(),
                    ))
                })
            };
            match inc_res {
                Ok(node_opt) => match node_opt {
                    Some(node) => ParseLine::Node(node),
                    None => ParseLine::Text(""),
                },
                Err(err) => ParseLine::Err(err),
            }
        } else if PRAGMA_ONCE.is_match(line) {
            if self.cache.get(path).unwrap().occured > 1 {
                ParseLine::Break
            } else {
                ParseLine::Text("")
            }
        } else {
            ParseLine::Text(line)
        }
    }

    fn parse(&mut self, path: &Path, text: String) -> io::Result<Option<Node>> {
        let mut node = Node::new(path);
        for line in text.lines() {
            match self.parse_line(path, line, &node) {
                ParseLine::Text(text) => {
                    node.add_line(text);
                },
                ParseLine::Node(child_node) => {
                    node.add_child(child_node);
                },
                ParseLine::Break => return Ok(None),
                ParseLine::Err(e) => return Err(e),
            }
        }
        Ok(Some(node))
    }
}

/// Reads and parses source files and resolves dependencies
///
/// Returns node tree that could be collected into resulting code string and index
pub fn build(hook: &dyn Hook, main: &Path) -> io::Result<Node> {
    let cwd = env::current_dir().ok();
    Builder::new(hook).build(&main, cwd.as_deref())
    .map(|root| root.unwrap())
}
