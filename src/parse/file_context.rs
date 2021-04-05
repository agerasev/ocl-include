use std::{io, path::Path};
use regex::{Regex, RegexBuilder};
use lazy_static::lazy_static;
use crate::node::Node;
use super::{context::Context, gate::{Gate, GateStack}};

fn make_regex(expr: &str) -> Regex {
    RegexBuilder::new(expr)
        .multi_line(true)
        .build()
        .unwrap()
}

lazy_static! {
    static ref INCLUDE: Regex = make_regex(r#"^\s*#include\s*([<"])(.*)([>"])\s*(?://)?.*$"#);
    static ref PRAGMA_ONCE: Regex = make_regex(r#"^\s*#pragma\s+once\s*(?://)?.*$"#);
    static ref IFDEF: Regex = make_regex(r#"^\s*#if(n?)def\s+(.*)\s*(?://)?.*$"#);
    static ref ENDIF: Regex = make_regex(r#"^\s*#endif\s*(?://)?.*$"#);
}

enum ParseLine<'a> {
    Text(&'a str),
    Node(Node),
    Break,
    Err(io::Error),
}

pub struct FileContext<'a, 'b> {
    node: Node,
    context: &'b mut Context<'a>,
    gate_stack: GateStack,
}


impl<'a, 'b> FileContext<'a, 'b> {
    pub fn new(path: &Path, context: &'b mut Context<'a>) -> Self {
        Self {
            node: Node::new(path),
            context,
            gate_stack: GateStack::new(),
        }
    }

    pub fn parse(mut self, text: String) -> io::Result<Option<Node>> {
        for line in text.lines() {
            match self.parse_line(line) {
                ParseLine::Text(text) => {
                    self.node.add_line(text);
                }
                ParseLine::Node(child_node) => {
                    self.node.add_child(child_node);
                }
                ParseLine::Break => return Ok(None),
                ParseLine::Err(e) => return Err(e),
            }
        }
        Ok(Some(self.node))
    }

    fn parse_line<'c>(&mut self, line: &'c str) -> ParseLine<'c> {
        let path = self.node.name().to_path_buf();
        if let Some(cap) = IFDEF.captures(line) {
            let name = &cap[2];
            match self.context.flags().get(name) {
                Some(&flag_value) => {
                    let value = cap[1].is_empty();
                    self.gate_stack.push(Gate::Known(String::from(name), value != flag_value));
                    ParseLine::Text("")
                },
                None => {
                    self.gate_stack.push(Gate::Unknown);
                    ParseLine::Text(line)
                }
            }
        } else if ENDIF.captures(line).is_some() {
            match self.gate_stack.pop() {
                Some(gate) => match gate {
                    Gate::Known(_, _) => ParseLine::Text(""),
                    Gate::Unknown => ParseLine::Text(line),
                },
                None => ParseLine::Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Extra #endif",
                )),
            }
        } else if PRAGMA_ONCE.is_match(line) {
            if self.context.is_file_occured(&path) {
                ParseLine::Break
            } else {
                ParseLine::Text("")
            }
        } else if self.gate_stack.is_open() {
            if let Some(cap) = INCLUDE.captures(line) {
                let inc_path = Path::new(&cap[2]);
                let (lb, rb) = (&cap[1], &cap[3]);
                let inc_res = {
                    if lb == "<" && rb == ">" {
                        Ok(None)
                    } else if lb == "\"" && rb == "\"" {
                        Ok(Some(path.parent().unwrap().to_path_buf()))
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "bad #include syntax",
                        ))
                    }
                    .and_then(|dir_opt| self.context.build_tree(inc_path, dir_opt.as_deref()))
                    .map_err(|err| {
                        io::Error::new(
                            err.kind(),
                            format!(
                                "{}\nin file '{}' at line {}",
                                err,
                                path.display(),
                                self.node.lines_count(),
                            ),
                        )
                    })
                };
                match inc_res {
                    Ok(node_opt) => match node_opt {
                        Some(node) => ParseLine::Node(node),
                        None => ParseLine::Text(""),
                    },
                    Err(err) => ParseLine::Err(err),
                }
            } else {
                ParseLine::Text(line)
            }
        } else {
            ParseLine::Text("")
        }
    }
}
