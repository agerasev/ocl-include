use std::{
    path::{Path, PathBuf},
};


pub struct Node {
    pub name: PathBuf,
    pub data: Data,
    inner: Vec<Node>,
}

impl Node {
    pub fn new(name: &Path) -> Self {
        Self {
            name: name.to_path_buf(),
            data: Data::new(),
            inner: Vec::new(),
        }
    }
    pub fn add_child(&mut self, node: Node) {
        self.inner.push(node);
    }
}

pub struct Data {
    string: String,
    lines: usize,
}

impl Data {
    pub fn new() -> Self {
        Self {
            string: String::new(),
            lines: 0,
        }
    }
    pub fn add_line(&mut self, line: &str) {
        self.string.push_str(line.trim_end());
        self.string.push('\n');
    }
    pub fn add_data(&mut self, other: &Data) {
        self.string.push_str(&other.string);
        self.lines += other.lines;
    }
    pub fn lines_count(&self) -> usize {
        self.lines
    }
    pub fn collect(mut self) -> String {
        self.string.shrink_to_fit();
        self.string
    }
}
