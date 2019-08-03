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
        self.hook.read(path, dir).and_then(|(path, text)| {
            match self.cache.entry(path.to_path_buf()) {
                Entry::Occupied(mut v) => { v.get_mut().occured += 1; },
                Entry::Vacant(v) => { v.insert(CacheEntry::new()); },
            }
            Ok((path, text))
        })
    }

    fn build(&mut self, path: &Path, dir: Option<&Path>) -> io::Result<Option<Node>> {
        self.read(path, dir)
        .and_then(|(path, text)| {
            if self.stack.iter().map(|p| (*p == path) as u32).fold(0, |a, x| a + x) >= 2 {
                Err(io::Error::new(io::ErrorKind::InvalidInput, format!(
                    "recursion found in file: '{}'", path.to_string_lossy()
                )))
            } else {
                self.stack.push(path.clone());
                Ok((path, text))
            }
        })
        .and_then(|(path, text)| self.parse(&path, text).map(|x| (x, path)))
        .and_then(|(x, path)| {
            assert_eq!(self.stack.pop().unwrap(), path);
            Ok(x)
        })
    }

    fn parse_line<'b>(
        &mut self, path: &Path, line: &'b str, node: &Node
    ) -> ParseLine<'b> {
        match INCLUDE.captures(line) {
            Some(cap) => Some({
                match {
                    let (lb, inc_path, rb) = (&cap[1], &cap[2], &cap[3]);
                    if lb == "<" && rb == ">" {
                        Ok(None)
                    } else if lb == "\"" && rb == "\"" {
                        Ok(Some(path.parent().unwrap().to_path_buf()))
                    } else {
                        Err(io::Error::new(io::ErrorKind::InvalidData, format!(
                            "error parsing file '{}' line {}: bad #include syntax",
                            path.to_string_lossy(), node.lines_count(),
                        )))
                    }
                    .map(|dir| (Path::new(inc_path).to_path_buf(), dir))
                }
                .and_then(|(path, dir)| {
                    let dir_ref = match dir {
                        Some(ref path_buf) => Some(path_buf.as_path()),
                        None => None,
                    };
                    self.build(&path, dir_ref)
                }) {
                    Ok(node_opt) => match node_opt {
                        Some(node) => ParseLine::Node(node),
                        None => ParseLine::Text(""),
                    },
                    Err(err) => ParseLine::Err(err),
                }
            }),
            None => None,
        }
        .or_else(|| {
            if PRAGMA_ONCE.is_match(line) {
                Some(if self.cache.get(path).unwrap().occured > 1 {
                    ParseLine::Break
                } else {
                    ParseLine::Text("")
                })
            } else {
                None
            }
        })
        .unwrap_or_else(|| {
            ParseLine::Text(line)
        })
    }

    fn parse(&mut self, path: &Path, text: String) -> io::Result<Option<Node>> {
        let mut node = Node::new(path);
        for (line_no, line) in text.lines().enumerate() {
            match self.parse_line(path, line, &node) {
                ParseLine::Text(text) => {
                    node.add_line(text);
                },
                ParseLine::Node(child_node) => {
                    node.add_child(child_node);
                },
                ParseLine::Break => return Ok(None),
                ParseLine::Err(e) => {
                    return Err(io::Error::new(
                        e.kind(),
                        format!("{} in {} at line {}", e, path.display(), line_no),
                    ))
                },
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
    Builder::new(hook).build(&main, cwd.as_ref().map(|p| p.as_path()))
    .map(|root| root.unwrap())
}
