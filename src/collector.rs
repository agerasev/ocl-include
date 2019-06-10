use std::{
    io,
    env,
    path::{Path, PathBuf},
};

use regex::{Regex, RegexBuilder};

use lazy_static::lazy_static;

use crate::{
    node::{Node},
    cache::{Cache},
    hook::{Hook},
};


lazy_static!{
    static ref INCLUDE: Regex = RegexBuilder::new(
        "^\\s*#include\\s*([<\"])([^>\"])([>\"])\\s*$"
    ).multi_line(true).build().unwrap();
    static ref PRAGMA_ONCE: Regex = RegexBuilder::new(
        "^\\s*#pragma\\s+once\\s*$"
    ).multi_line(true).build().unwrap();
}

enum ParseLine<'a> {
    Text(&'a str),
    Node(Node),
    Break,
    Err(io::Error),
}

pub struct Collector {
    hook: Box<dyn Hook>,
    cache: Cache,
    stack: Vec<PathBuf>,
}

impl Collector {
    pub fn new(hook: Box<dyn Hook>) -> Self {
        Collector {
            hook,
            cache: Cache::new(),
            stack: Vec::new(),
        }
    }

    fn read(&mut self, path: &Path) -> io::Result<String> {
        match self.cache.get_mut(path) {
            Some(entry) => {
                entry.occured += 1;
                Ok(entry.text.clone())
            },
            None => {
                self.hook.read(path)
                .and_then(|text| {
                    self.cache.put(path, text.clone())
                    .map(|()| text)
                })
            },
        }
    }

    pub fn collect(&mut self, path: &Path, dir: Option<&Path>) -> io::Result<Option<Node>> {
        if path.is_absolute() {
            Ok(path)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput,format!(
                "absolute path expected, got '{}'", path.to_string_lossy()
            )))
        }
        .and_then(|rpath| self.hook.find(rpath, dir))
        .and_then(|path| {
            if self.stack.iter().map(|p| (*p == path) as u32).fold(0, |a, x| a + x) >= 2 {
                Err(io::Error::new(io::ErrorKind::InvalidInput, format!(
                    "recursion found in file: {}", path.to_string_lossy()
                )))
            } else {
                self.stack.push(path.clone());
                Ok(path)
            }
        })
        .and_then(|path| self.read(&path).map(|t| (t, path)))
        .and_then(|(text, path)| self.parse(&path, text))
        .and_then(|x| {
            assert_eq!(self.stack.pop().unwrap(), path);
            Ok(x)
        })
    }

    fn parse_line<'a>(
        &mut self, path: &Path, line: &'a str, node: &Node
    ) -> ParseLine<'a> {
        match INCLUDE.captures(line) {
            Some(cap) => Some({
                match {
                    let (lb, inc_path, rb) = (&cap[0], &cap[1], &cap[2]);
                    if lb == "<" && rb == ">" {
                        Ok(None)
                    } else if lb == "\"" && rb == "\"" {
                        Ok(Some(path.parent().unwrap().to_path_buf()))
                    } else {
                        Err(io::Error::new(io::ErrorKind::InvalidData, format!(
                            "error parsing file '{}' line {}: bad #include syntax",
                            path.to_string_lossy(), node.data.lines_count(),
                        )))
                    }
                    .map(|dir| (Path::new(inc_path).to_path_buf(), dir))
                }
                .and_then(|(path, dir)| {
                    let dir_ref = match dir {
                        Some(ref path_buf) => Some(path_buf.as_path()),
                        None => None,
                    };
                    self.collect(&path, dir_ref)
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
        for line in text.lines() {
            match self.parse_line(path, line, &node) {
                ParseLine::Text(text) => {
                    node.data.add_line(text);
                },
                ParseLine::Node(child_node) => {
                    node.add_child(child_node);
                    node.data.add_line("");
                },
                ParseLine::Break => return Ok(None),
                ParseLine::Err(e) => return Err(e),
            }
        }
        Ok(Some(node))
    }
}

pub fn collect(main: &Path, hook: Box<dyn Hook>) -> io::Result<Node> {
    if main.is_absolute() {
        Ok(main.to_path_buf())
    } else {
        env::current_dir()
        .map(|cwd| cwd.join(main))
    }
    .and_then(|p| p.canonicalize())
    .and_then(|p| {
        Collector::new(hook).collect(&p, None)
        .map(|root| root.unwrap() )
    })
}
