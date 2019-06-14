use std::{
    path::{Path, PathBuf},
};


pub struct Node {
    name: PathBuf,
    inner: Vec<(Node, usize)>,
    text: String,
    index: Vec<usize>,
}

impl Node {
    pub fn new(name: &Path) -> Self {
        Self {
            name: name.to_path_buf(),
            inner: Vec::new(),
            text: String::new(),
            index: Vec::new(),
        }
    }

    pub fn name(&self) -> &Path {
        &self.name
    }

    pub fn add_line(&mut self, line: &str) {
        self.index.push(self.text.len());
        self.text.push_str(line.trim_end());
        self.text.push('\n');
    }

    pub fn add_child(&mut self, node: Node) {
        self.add_line("");
        self.inner.push((node, self.index.len()));
    }

    pub fn lines_count(&self) -> usize {
        self.index.len()
    }

    pub fn collect(&self) -> String {
        let mut accum = String::new();

        let mut ppos = 0;
        for (node, pos) in self.inner.iter() {
            accum.push_str(&self.text[ppos..*pos]);
            accum.push_str(&node.collect());
            ppos = *pos
        }
        accum.push_str(&self.text[ppos..]);

        accum
    }
}
